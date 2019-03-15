use core::ops;

use hal::prelude::*;

use hal::gpio::gpioa::{
    self,
    PAx,
    PA1,  // blue
    PA2,  // red
    PA3,  // green
};

use hal::gpio::{
    Output,
    PushPull,
};

pub enum Color {
    Red,    // pa2
    Green,  // pa3
    Blue,   // pa1
}

pub type RedLed = PA2<Output<PushPull>>;
pub type GreenLed = PA3<Output<PushPull>>;
pub type BlueLed = PA1<Output<PushPull>>;

pub struct Led {
    pax: PAx<Output<PushPull>>,
}

impl Led {
    /// turn on the LED
    pub fn on(&mut self) {
        self.pax.set_low()
    }

    /// turn off the LED
    pub fn off(&mut self) {
        self.pax.set_high()
    }
}

pub struct Leds {
    leds: [Led; 3],
}

impl Leds {
    /// initialize all the LEDs
    /// NB: "on" = "high"
    /// Todo: try and initialize as off, to prevent flashing
    pub fn new(mut gpioa: gpioa::Parts) -> Self {

        let mut red = gpioa.pa2
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        red.set_high();

        let mut green = gpioa.pa3
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        green.set_high();

        let mut blue = gpioa.pa1
            .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
        blue.set_high();

        Leds {
            leds: [
                red.into(),
                green.into(),
                blue.into(),
            ],
        }
    }
}

// implement indexing into Leds
impl ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl ops::Index<Color> for Leds {
    type Output = Led;

    fn index(&self, color: Color) -> &Led {
        &self.leds[color as usize]
    }
}

impl ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl ops::IndexMut<Color> for Leds {
    fn index_mut(&mut self, color: Color) -> &mut Led {
        &mut self.leds[color as usize]
    }
}

// ability to turn specific into generic LED type
macro_rules! ctor {
    ($($ldx:ident),+) => {
        $(
            impl Into<Led> for $ldx {
                fn into(self) -> Led {
                    Led {
                        pax: self.downgrade(),
                    }
                }
            }
        )+
    }
}

ctor!(RedLed, GreenLed, BlueLed);


impl ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}
