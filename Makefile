blinky: blinky-debug.hex blinky-release.hex

clean:
	cargo clean
	rm -f blinky-debug.hex blinky-release.hex

.PHONY: blinky-debug.hex
blinky-debug.hex:
	cargo build --example blinky
	cargo size --example blinky
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/debug/examples/blinky blinky-debug.hex

.PHONY: blinky-release.hex
blinky-release.hex:
	cargo build --example blinky --release
	cargo size --example blinky --release
	arm-none-eabi-objcopy -O ihex target/thumbv7em-none-eabihf/release/examples/blinky blinky-release.hex

.PHONY: ed25519
ed25519:
	cargo build --example ed25519 --release
	cargo size --example ed25519 --release
