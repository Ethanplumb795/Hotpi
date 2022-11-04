use std::error::Error;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    // NOTE: in Python we used bus 1, device 0
    // Speed was 5_000_000
    // Current spi device: Bus 0, CS 0
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 5_000_000, Mode::Mode0)?;
    let mut read_buffer: [u8; 4] = [0; 4];
    let write_buffer: [u8; 4] = [0; 4];
    let num_bytes_transferred;

    // Testing rpi spi
    println!("Testing SPI on a {}.", DeviceInfo::new()?.model());

    // Make a single measurement
    num_bytes_transferred = spi.transfer(&mut read_buffer, &write_buffer);

    // Testing measurement
    println!("\nNuber of Bytes Transferred: {}", num_bytes_transferred.unwrap());
    print!("Read buffer:\n");
    for c in read_buffer {
        print!("{:b}: ", c);
        for i in (0..8).rev() {
            print!("{}", (c & (1 << i)) >> i);
        }
        println!()
    }
    print!("\nWrite buffer: ");
    for c in write_buffer {
        print!("{:b}, ", c);
    }

    let oc_fault:bool = (read_buffer[3] & 0b1) == 0b1;
    let scg_fault:bool = (read_buffer[3] & 0b10) == 0b10;
    let scv_fault:bool = (read_buffer[3] & 0b100) == 0b100;
    let fault:bool = (read_buffer[2] & 0b1) == 0b1;
    println!("fault: {} ocf: {} scg: {} scv: {}", fault, oc_fault, scg_fault, scv_fault);

    // Bit 0 is least significant bit, buffer 0 is most significant buffer
    let mut temp_12:u16 = 0;
    temp_12 |= (read_buffer[2] as u16) << 4;
    temp_12 |= (read_buffer[3] as u16 & 0b11110000) >> 4;
    println!("12 bit temp data: {:b}", temp_12);

    // Bit 0 is lsb, buffer 0 is most significant buffer
    let mut temp_14:u16 = 0;
    temp_14 |= (read_buffer[0] as u16) << 6;
    temp_14 |= (read_buffer[1] as u16 & 0b11111100) >> 2;
    println!("14 bit temp data: {:b}", temp_14);

    let mut therm_temp_12:f32 = 0.0;
    for bit in 0..11 {
        let tmp = temp_12 & (1 << bit);
        if tmp == (1 << bit) {
            therm_temp_12 += f32::powf(2.0, bit as f32 - 4.0);
        }
        println!("Temperature at bit = {}: {}", bit, therm_temp_12);
    }
    if (temp_12 & 0b100000000000) == 0b100000000000 {
        therm_temp_12 *= -1.0;
    }
    println!("temp = {}", therm_temp_12);

    let mut therm_temp_14:f32 = 0.0;
    for bit in 0..13 {
        let tmp = temp_14 & (1 << bit);
        if tmp == (1 << bit) {
            therm_temp_14 += f32::powf(2.0, bit as f32 - 2.0);
        }
        println!("Temperature at bit = {}: {}", bit, therm_temp_14);
    }
    if (temp_14 & 0b10000000000000) == 0b10000000000000 {
        therm_temp_14 *= -1.0;
    }
    println!("temp = {}", therm_temp_14);

    Ok(())
}
