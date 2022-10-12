use std::error::Error;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0)?;

    println!("Testing SPI on a {}.", DeviceInfo::new()?.model());

    Ok(())
}
