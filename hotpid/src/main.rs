use std::error::Error;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::system::DeviceInfo;

fn main() -> Result<(), Box<dyn Error>> {
    // NOTE: in Python we used bus 1, device 0
    // Speed was 5_000_000
    let spi = Spi::new(Bus::Spi1, SlaveSelect::Ss0, 5_000_000, Mode::Mode0)?;

    println!("Testing SPI on a {}.", DeviceInfo::new()?.model());

    Ok(())
}
