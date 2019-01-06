blinky:
	cargo build --example blinky
	cargo size --example blinky
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/debug/examples/blinky blinky.hex
