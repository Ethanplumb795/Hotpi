# Hotpi

## To-Do:

1. Switch spi control over to Rust from Python

2. re-design with a mux, connect to SPI0 so it is compatible with all Pi models

	Pins for SPI0:

		MISO: GPIO9 (pin 21)

		MOSI: GPIO10 (pin 19)

		SCLK: GPIO11 (pin 23)

		CS: Ss0=GPIO8 (pin 24), Ss1=GPIO7 (pin 26)

3. ...
