// TODO: Frequency of measurements
//       Duration
//       Number of averages: done
//       Start Time
//       Save to CSV
// Later: Initiate measurement experiment from a skeletonized flask web app

use std::error::Error;
use std::{thread, time};

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

use std::fs::File;
use std::io::prelude::*;

use chrono::{Datelike, Timelike, Local};

struct Measurement {
    m_time: time::SystemTime,
    board: f32,
    couple: f32,
}

fn take_measurement(spi:&Spi) -> Measurement {
    let mut read_buffer: [u8; 4] = [0; 4];
    let write_buffer: [u8; 4] = [0; 4];

    // Make a single measurement
    let transfer_result = spi.transfer(&mut read_buffer, &write_buffer);
    let num_bytes_transferred = match transfer_result {
        Ok(bytes) => bytes,
        Err(error) => panic!("Problem transferring to spi: {:?}", error),
    };

    let oc_fault:bool = (read_buffer[3] & 0b1) == 0b1;
    let scg_fault:bool = (read_buffer[3] & 0b10) == 0b10;
    let scv_fault:bool = (read_buffer[3] & 0b100) == 0b100;
    let fault:bool = (read_buffer[2] & 0b1) == 0b1;
    if fault || oc_fault || scg_fault || scv_fault {
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

    let measurement_time = time::SystemTime::now();

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
        //thread::sleep(time::Duration::from_millis(10));
    }

    // Take average of measurements
    for _i in 0..n {
        board_avg += board.pop().unwrap();
        couple_avg += couple.pop().unwrap();
    }
    board_avg /= n as f32;
    couple_avg /= n as f32;
    let measurement_time = time::SystemTime::now();
    let avg = Measurement {
        m_time: measurement_time,
        board: board_avg,
        couple: couple_avg,
    };

    println!("Time: {}", avg.m_time);

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
    let mut file = File::create(name)?;
    file.write_all(b"time,couple,board\n")?;
    //for v in measurement_v {
        //file.write_all(
    //}

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // NOTE: in Python we used bus 1, device 0
    // Speed was 5_000_000
    // Current spi device: Bus 0, CS 0
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 5_000_000, Mode::Mode0)?;

    // Testing rppal
    println!("Testing SPI on a {}.", DeviceInfo::new()?.model());

    // Test avg measurement
    let avg_ten = avg_measurement(10, &spi);
    println!("\nBoard average: {}. Thermocouple average: {}.", avg_ten.board, avg_ten.couple);

    // Test meas_over_time()
    let mut measurement_name = String::from("testing_measurement");
    let mut measurement_v = Vec::new();
    meas_over_time(10, 2.0, 10, &spi, &mut measurement_v);

    // Test save_to_csv()
    measurement_name.insert_str(0, "resources/");
    save_to_csv(&measurement_v, measurement_name);

    Ok(())
}
