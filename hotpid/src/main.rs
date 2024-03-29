// TODO: Read measurement setup from Flask app, execute measurement
// Later: Initiate measurement experiment from a skeletonized flask web app

use std::error::Error;
use std::{thread, time};

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

use std::fs;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use chrono::Local;

struct Measurement {
    m_time: chrono::DateTime<Local>,
    board: f32,
    couple: f32,
}

fn take_measurement(spi:&Spi) -> Measurement {
    let mut read_buffer: [u8; 4] = [0; 4];
    let write_buffer: [u8; 4] = [0; 4];

    // Make a single measurement
    let transfer_result = spi.transfer(&mut read_buffer, &write_buffer);
    let _num_bytes_transferred = match transfer_result {
        Ok(bytes) => bytes,
        Err(error) => panic!("Problem transferring to spi: {:?}", error),
    };

    let oc_fault:bool = (read_buffer[3] & 0b1) == 0b1;
    let scg_fault:bool = (read_buffer[3] & 0b10) == 0b10;
    let scv_fault:bool = (read_buffer[3] & 0b100) == 0b100;
    let fault:bool = (read_buffer[2] & 0b1) == 0b1;
    if fault && (oc_fault || scg_fault || scv_fault) {
        println!("fault:{} ocf:{} scg:{} scv:{}", fault, oc_fault, scg_fault, scv_fault);
    }

    // Bit 0 is lsb, buffer 0 is most significant buffer
    let mut temp_12:u16 = 0;
    temp_12 |= (read_buffer[2] as u16) << 4;
    temp_12 |= (read_buffer[3] as u16 & 0b11110000) >> 4;

    // Bit 0 is lsb, buffer 0 is most significant buffer
    let mut temp_14:u16 = 0;
    temp_14 |= (read_buffer[0] as u16) << 6;
    temp_14 |= (read_buffer[1] as u16 & 0b11111100) >> 2;

    let mut therm_temp_12:f32 = 0.0;
    for bit in 0..11 {
        let tmp = temp_12 & (1 << bit);
        if tmp == (1 << bit) {
            therm_temp_12 += f32::powf(2.0, bit as f32 - 4.0);
        }
    }
    if (temp_12 & 0b100000000000) == 0b100000000000 {
        therm_temp_12 *= -1.0;
    }

    let mut therm_temp_14:f32 = 0.0;
    for bit in 0..13 {
        let tmp = temp_14 & (1 << bit);
        if tmp == (1 << bit) {
            therm_temp_14 += f32::powf(2.0, bit as f32 - 2.0);
        }
    }
    if (temp_14 & 0b10000000000000) == 0b10000000000000 {
        therm_temp_14 *= -1.0;
    }

    let measurement_time = chrono::offset::Local::now();

    let measurement = Measurement {
        m_time: measurement_time,
        board: therm_temp_12,
        couple: therm_temp_14,
    };

    // Return the Measurement struct
    measurement
}

fn avg_measurement(n:u8, spi:&Spi) -> Measurement {
    if n <= 0 {
        panic!("[ERROR] must make at least 1 measurement.");
    }

    let mut board = Vec::new();
    let mut couple = Vec::new();
    let mut board_avg:f32 = 0.0;
    let mut couple_avg:f32 = 0.0;

    // Take n measurements
    for _i in 0..n {
        let tmp_measurement = take_measurement(&spi);
        board.push(tmp_measurement.board);
        couple.push(tmp_measurement.couple);
        //thread::sleep(time::Duration::from_millis(1));
    }

    // Take average of measurements
    for _i in 0..n {
        board_avg += board.pop().unwrap();
        couple_avg += couple.pop().unwrap();
    }
    board_avg /= n as f32;
    couple_avg /= n as f32;
    let measurement_time = chrono::offset::Local::now();
    let avg = Measurement {
        m_time: measurement_time,
        board: board_avg,
        couple: couple_avg,
    };

    println!("Time: {:?}", avg.m_time);

    // Return avg measurement
    avg
}

fn meas_over_time(duration:u32, freq:f32, num_avg:u8, spi:&Spi, vec:&mut Vec<Measurement>) {
    // Duration measured in seconds -> max duration = 49710.26963 days
    // freq measured in hz
    if freq <= 0.0 {
        panic!("[ERROR] frequency must be a positive value.");
    }

    // period in seconds
    let period = time::Duration::from_secs_f32(1.0/freq);
    vec.clear(); // Remove all elements

    if duration == 0 {
        vec.push(take_measurement(spi));
    }
    else {
        // For the selected duration, take avg measurement and wait the inverse of frequency
        let mut i:u32 = 0;
        while i < duration {
            // Do thing
            let new_meas = avg_measurement(num_avg, spi);
            println!("Measurement #{}: {}", i+1, new_meas.couple);
            vec.push(new_meas);
            // Wait inverse of frequency
            thread::sleep(period);
            i += 1;
        }
    }
}

fn save_to_csv(measurement_v:&Vec<Measurement>, name:String) -> std::io::Result<()> {
    let mut file = fs::File::create(name)?;
    write!(file, "time,couple,board\n")?;
    for v in measurement_v {
        write!(file, "{:?},{},{}\n", v.m_time, v.couple, v.board)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // NOTE: in Python we used bus 1, device 0
    // Speed was 5_000_000
    // Current spi device: Bus 0, CS 0
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 5_000_000, Mode::Mode0)?;

    // Testing rppal
    // println!("Testing SPI on a {}.", DeviceInfo::new()?.model());

    let measurement_dir = Path::new("../measurements");
    let wait_time = time::Duration::from_secs_f32(1.0);
    if measurement_dir.is_dir() {
        println!("../measurements exists");
        loop {
            for request in fs::read_dir(measurement_dir)? {
                let request = request?;
                let path = request.path();
                let path_clone1 = path.clone();
                let path_clone2 = path.clone();
                if !(path.is_dir()) {
                    println!("[Status] Executing measurement");
                    thread::sleep(wait_time);
                    let mut measurement_v = Vec::new();
                    let contents = fs::read_to_string(path)
                        .expect("Should have been able to read the file");
                    let mut measurement_name = String::from(<&str as Into<String>>::into(path_clone1.file_name().unwrap()
                                                            .to_str().unwrap()));
                    let field: Vec<&str> = contents.split(",").collect();
                    let meas_freq:f32 = field[0].parse::<f32>().unwrap();
                    let num_avgs:u8 = field[1].parse::<u8>().unwrap();
                    let duration:u32 = field[2].parse::<u32>().unwrap();
                    fs::remove_file(path_clone2).expect("File delete failed");
                    meas_over_time(duration, meas_freq, num_avgs, &spi, &mut measurement_v);
                    measurement_name.insert_str(0, "../measurement_results/");
                    save_to_csv(&measurement_v, measurement_name)?;
                }
                else {
                    println!("[ERROR] There should exist no subdirectories in the measurement directory.\n[Status] Quitting program...");
                    break;
                }
            }
            thread::sleep(wait_time);
        }
    }

    // Test measuring time:
    println!("Time: {:?} using chrono::offset::Local::now()", chrono::offset::Local::now());

    Ok(())
}
