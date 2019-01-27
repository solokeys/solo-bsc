#![no_main]
#![no_std]

// panicking behavior
extern crate panic_halt;

use core::fmt;
use core::fmt::Write;

// board support
extern crate solo_bsp as board;
use board::hal::stm32;
use board::hal::prelude::*;
use board::hal::delay::Delay;
use board::hal::serial::Serial;
use board::hal::i2c::I2c;

use ssd1306::prelude::*;
use ssd1306::Builder;

use ed25519_dalek::*;


macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        fmt::write($serial, format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}


// https://tools.ietf.org/html/rfc8032#section-7.3
fn ed25519ph_rf8032_test_vector() -> (bool, Sha512, Signature, Signature) {
    let secret_key: [u8; SECRET_KEY_LENGTH] = [
        0x83, 0x3f, 0xe6, 0x24, 0x09, 0x23, 0x7b, 0x9d,
        0x62, 0xec, 0x77, 0x58, 0x75, 0x20, 0x91, 0x1e,
        0x9a, 0x75, 0x9c, 0xec, 0x1d, 0x19, 0x75, 0x5b,
        0x7d, 0xa9, 0x01, 0xb9, 0x6d, 0xca, 0x3d, 0x42,
    ];
    let public_key: [u8; PUBLIC_KEY_LENGTH] = [
        0xec, 0x17, 0x2b, 0x93, 0xad, 0x5e, 0x56, 0x3b,
        0xf4, 0x93, 0x2c, 0x70, 0xe1, 0x24, 0x50, 0x34,
        0xc3, 0x54, 0x67, 0xef, 0x2e, 0xfd, 0x4d, 0x64,
        0xeb, 0xf8, 0x19, 0x68, 0x34, 0x67, 0xe2, 0xbf,
    ];
    let message: [u8; 3] = [0x61, 0x62, 0x63];
    let signature: [u8; SIGNATURE_LENGTH] = [
        0x98, 0xa7, 0x02, 0x22, 0xf0, 0xb8, 0x12, 0x1a,
        0xa9, 0xd3, 0x0f, 0x81, 0x3d, 0x68, 0x3f, 0x80,
        0x9e, 0x46, 0x2b, 0x46, 0x9c, 0x7f, 0xf8, 0x76,
        0x39, 0x49, 0x9b, 0xb9, 0x4e, 0x6d, 0xae, 0x41,
        0x31, 0xf8, 0x50, 0x42, 0x46, 0x3c, 0x2a, 0x35,
        0x5a, 0x20, 0x03, 0xd0, 0x62, 0xad, 0xf5, 0xaa,
        0xa1, 0x0b, 0x8c, 0x61, 0xe6, 0x36, 0x06, 0x2a,
        0xaa, 0xd1, 0x1c, 0x2a, 0x26, 0x08, 0x34, 0x06,
    ];
    let secret: SecretKey = SecretKey::from_bytes(&secret_key[..SECRET_KEY_LENGTH]).unwrap();
    let public: PublicKey = PublicKey::from_bytes(&public_key[..PUBLIC_KEY_LENGTH]).unwrap();
    let keypair: Keypair = Keypair { secret: secret, public: public };
    let reference_signature: Signature = Signature::from_bytes(&signature[..]).unwrap();

    let mut prehash_for_signing: Sha512 = Sha512::default();
    let mut prehash_for_verifying: Sha512 = Sha512::default();

    prehash_for_signing.input(&message[..]);
    prehash_for_verifying.input(&message[..]);

    let generated_signature: Signature = keypair.sign_prehashed(prehash_for_signing, None);

    // assert!(reference_signature == generated_signature,
    //         "Original signature from test vectors doesn't equal signature produced:\
    //         \noriginal:\n{:?}\nproduced:\n{:?}", reference_signature, generated_signature);
    // assert!(keypair.verify_prehashed(prehash_for_verifying, None, &generated_signature).is_ok(),
    //         "Could not verify ed25519ph signature!");

    (
        reference_signature == generated_signature,
        prehash_for_verifying,
        reference_signature,
        generated_signature,
    )
}

#[board::entry]
fn main() -> ! {
    let (sig_match, hash, ref_sig, sig) = ed25519ph_rf8032_test_vector();
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32::Peripherals::take().unwrap();

    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);

    // setup usart2
    let tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    let rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    // let baud_rate = 115_200;
    let baud_rate = 9_600;
    let serial = Serial::usart2(
        device.USART2,
        (tx, rx),
        baud_rate.bps(),
        clocks,
        &mut rcc.apb1r1
    );
    let (mut tx, _) = serial.split();
    /*
    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    let baud_rate = 115_200;
    let serial = Serial::usart1(
        device.USART1,
        (tx, rx),
        baud_rate.bps(),
        clocks,
        &mut rcc.apb2
    );
    let (mut tx, _) = serial.split();
    */

    // setup led
    let mut gpiob = device.GPIOB.split(&mut rcc.ahb2);
    let mut led_pin = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    uprintln!(&mut tx, "the clocks: {:?}", clocks);
    uprintln!(&mut tx, "sig:\n {:?}\n", sig);
    uprintln!(&mut tx, "ref_sig:\n {:?}\n", ref_sig);
    uprintln!(&mut tx, "hash:\n {:?}\n", hash);

    // setup I2C and Display
    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let i2c = I2c::i2c1(
        device.I2C1,
        (scl, sda),
        100.khz(),
        clocks,
        &mut rcc.apb1r1
    );

    let mut oled_display: TerminalMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x32)
        // Chinese clones have different addresses
        .with_i2c_addr(0x78)
        .connect_i2c(i2c).into();

    oled_display.init().unwrap();
    let _ = oled_display.clear();

    // get a timer
    let mut timer = Delay::new(core.SYST, clocks);

    let delay_ms: u32 = 1500;
    let mut i = 1;
    loop {
        // uprintln!(
        //     &mut tx,
        //     "the generated signature: {:?}",
        //     sig,
        // );
        // timer.delay_ms(delay_ms);
        led_pin.set_high();
        timer.delay_ms(delay_ms);

        led_pin.set_low();
        timer.delay_ms(delay_ms);

        // let _ = oled_display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
        // let _ = oled_display.write_str("Hello! ");
        let _ = oled_display.clear();
        // let _ = oled_display.write_fmt(
        //     format_args!("hello #{}: hsi48 = {:?}\n", i, clocks.hsi48())
        // );
        let _ = oled_display.write_fmt(
            // format_args!("#{} sig:\n {:?}\n", i, sig)
            format_args!("#{} sig match:\n {:?}\n", i, sig_match)
        );
        i += 1;
    }
}
