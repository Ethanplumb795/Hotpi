use rppal::spi;

fn main() {
    let spi0 = spi::Spi {
        bus: spi::Bus::Spi0,
        slave_select: spi::SlaveSelect::Ss0,
        clock_speed: 5000000,
        mode: spi::Mode::Mode0,
    };

    let mut buf: [u8; 16] = [0; 16];

    spi0.read(&mut buf);
}
