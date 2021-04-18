#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{ entry, Delay, DelayMs, LedArray, OutputSwitch };
use volatile::{Volatile};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut half_period = 300_u16;
    let v_half_period = Volatile::new(&mut half_period);

    let mut i = 0;
    let led_count = 8;
    
    loop {
        i = i % led_count;
        let b = (i  + 1) % led_count;
        leds[i].on().ok();
        leds[b].on().ok();
        delay.delay_ms(v_half_period.read());
        leds[i].off().ok();
        i += 1;
    }
}
