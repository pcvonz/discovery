#![no_main]
#![no_std]

use aux9::{entry, tim6};

#[inline(never)]
fn delay(_tim6: &tim6::RegisterBlock, ms: u16) {
    let cycle = (72_u32) * (ms as u32);
    for _ in 0..cycle{ 
        aux9::nop()
    }
}

#[entry]
fn main() -> ! {
    let (mut leds, _rcc, tim6) = aux9::init();

    // TODO initialize TIM6

    let ms = 1000;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay(tim6, ms);
            leds[curr].off();
            delay(tim6, ms);
        }
    }
}
