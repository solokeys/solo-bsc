[![License](https://img.shields.io/github/license/solokeyssec/solo-bsp.svg)](https://github.com/SoloKeysSec/solo-bsp/blob/master/LICENSE)
[![Crates](https://img.shields.io/crates/v/solo-bsp.svg)](https://crates.io/crates/solo-bsp)

# solo-bsp
This is a (WIP!) [Rust](https://github.com/rust-embedded) board support package for the open source Solo security key.

This key [consists of](https://github.com/SoloKeysSec/solo-hw):
- an STM32L432KC microcontroller
- either a USB-A or USB-C connector
- a clicky dome button
- 3 LEDs
- an NCP114 voltage regulator
- and various resistors, capacitors, and Zener diodes

One specialty is that it has a [custom USB bootloader](https://solo.solokeys.io/building/), allowing easy updates.
Alternatively, the ST DFU bootloader can be used.
Additionally, serial TX/RX and all SWD pins (SWDIO, SWCLK, SWO) are [kind of broken out](https://conorpp.com/3d-printing-a-programming-jig-and-embedding-pogo-pins-using-eagle-and-fusion-360).

## Quickstart
You need stable Rust 2018 edition, for details see the [embedded book](https://docs.rust-embedded.org/book/intro/install.html), in short:
```
curl https://sh.rustup.rs -sSf | sh
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

To build blinky, run `make blinky`. You end up with a `blinky.hex` file.

To flash it to your Solo Hacker:
- insert Solo and boot to Solo bootloader by pressing the button for ~2 seconds (it starts to blink)
- [setup Solotool](https://github.com/SoloKeysSec/solo/blob/master/README.md#solo-for-hackers) and make sure you can flash the original [`solo.hex`](https://github.com/SoloKeysSec/solo/releases/download/basic-hacker-build/solo.hex)
- run `tools/solotool.py /path/to/blinky.hex`
- watch the green LED blink :tada:

## License
[GPLv3](https://github.com/SoloKeysSec/solo-bsp/blob/master/LICENSE)
