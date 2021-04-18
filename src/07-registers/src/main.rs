#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM};

fn iprint_odr(itm: &mut ITM, gpioe: &aux7::RegisterBlock) {
    iprintln!(
        &mut itm.stim[0],
        "ODR = 0x{:04x}",
        gpioe.odr.read().bits()
    );
}

#[entry]
fn main() -> ! {
    let (mut itm, gpioe) = aux7::init();

    iprint_odr(&mut itm, gpioe);

    // Turn on the "North" LED (red)
    gpioe.bsrr.write(|w| w.bs9().set_bit());
    iprint_odr(&mut itm, gpioe);

    // Turn on the "East" LED (green)
    gpioe.bsrr.write(|w| w.bs11().set_bit());
    iprint_odr(&mut itm, gpioe);

    // Turn off the "North" LED
    gpioe.bsrr.write(|w| w.br9().set_bit());
    iprint_odr(&mut itm, gpioe);

    // Turn off the "East" LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    iprint_odr(&mut itm, gpioe);


    loop {}
}
