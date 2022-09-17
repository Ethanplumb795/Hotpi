use rppal::spi;

fn main() {
    let spi0 = spi::Spi::new(spi::Bus::Spi0, spi::SlaveSelect::Ss0, 5000000, spi::Mode::Mode0);
}
