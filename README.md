[![License](https://img.shields.io/github/license/solokeyssec/solo-bsp.svg)](https://github.com/SoloKeysSec/solo-bsp/blob/master/LICENSE)
[![Crates](https://img.shields.io/crates/v/solo-bsp.svg)](https://crates.io/crates/solo-bsp)

# solo-bsp
This is a (WIP!) board support package for the open source Solo security key.

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


## License
[GPLv3](https://github.com/SoloKeysSec/solo-bsp/blob/master/LICENSE)
