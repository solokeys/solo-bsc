#![no_main]
#![no_std]

// panicking behavior
extern crate panic_halt;

// board support
extern crate solo_bsc as board;
use board::stm32;
use board::hal::prelude::*;
use board::hal::delay;
use board::led::{Color, Leds};

#[board::entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();

    let clocks = rcc.cfgr
        .hclk(8.mhz())
        .freeze(&mut flash.acr);

     let gpioa = device.GPIOA.split(&mut rcc.ahb2);
     let mut leds = Leds::new(gpioa);

    // let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
    // let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = delay::Delay::new(core.SYST, clocks);
    let second: u32 = 100;

    loop {
        // led.set_high();
        // timer.delay_ms(second);
        // led.set_low();
        // timer.delay_ms(second);

        leds[Color::Red].on();
        timer.delay_ms(second);
        leds[Color::Red].off();
        timer.delay_ms(second);

        leds[Color::Green].on();
        timer.delay_ms(second);
        leds[Color::Green].off();
        timer.delay_ms(second);

        leds[Color::Blue].on();
        timer.delay_ms(second);
        leds[Color::Blue].off();
        timer.delay_ms(second);

    }
}
