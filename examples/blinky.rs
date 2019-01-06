#![no_main]
#![no_std]

// panicking behavior
extern crate panic_halt;

// board support
extern crate solo_bsp as board;
use board::stm32;
use board::hal::prelude::*;
use board::hal::delay;

#[board::entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();

    let clocks = rcc.cfgr
        .freeze(&mut flash.acr);

    let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);

    // let mut led_blue = gpioa.pa1
    //     .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    // let mut led_red = gpioa.pa2
    //     .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut led_green = gpioa.pa3
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut timer = delay::Delay::new(core.SYST, clocks);
    let half_second: u32 = 500;

    loop {
        // led_red.set_high();
        // timer.delay_ms(half_second);
        // led_red.set_low();
        // timer.delay_ms(half_second);

        led_green.set_high();
        timer.delay_ms(half_second);
        led_green.set_low();
        timer.delay_ms(half_second);

        // led_blue.set_high();
        // timer.delay_ms(half_second);
        // led_blue.set_low();
        // timer.delay_ms(half_second);
    }
}
