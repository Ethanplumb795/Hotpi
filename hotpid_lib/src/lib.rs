// Library to be called by Python in the webapp
use std::{thread, time};

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use std::fs::File;
use std::io::prelude::*;

use chrono::Local;

#[repr(C)]
pub struct Measurement {
    pub m_time: chrono::DateTime<Local>,
    pub board: f32,
    pub couple: f32,
}

#[no_mangle]
pub extern "C" fn take_measurement(spi:&Spi) -> Measurement {
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

#[no_mangle]
pub extern "C" fn avg_measurement(n:u8, spi:&Spi) -> Measurement {
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

#[no_mangle]
pub extern "C" fn meas_over_time(duration:u32, freq:f32, num_avg:u8, vec:&mut Vec<Measurement>) {
    // Duration measured in seconds -> max duration = 49710.26963 days
    // freq measured in hz
    if freq <= 0.0 {
        panic!("[ERROR] frequency must be a positive value.");
    }

    // period in seconds
    let period = time::Duration::from_secs_f32(1.0/freq);
    vec.clear(); // Remove all elements

    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 5_000_000, Mode::Mode0).unwrap();

    if duration == 0 {
        vec.push(take_measurement(&spi));
    }
    else {
        // For the selected duration, take avg measurement and wait the inverse of frequency
        let mut i:u32 = 0;
        while i < duration {
            // Do thing
            let new_meas = avg_measurement(num_avg, &spi);
            println!("Measurement #{}: {}", i+1, new_meas.couple);
            vec.push(new_meas);
            // Wait inverse of frequency
            thread::sleep(period);
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn save_to_csv(measurement_v:&Vec<Measurement>, name:String) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    write!(file, "time,couple,board\n")?;
    for v in measurement_v {
        write!(file, "{:?},{},{}\n", v.m_time, v.couple, v.board)?;
    }

    Ok(())
}
