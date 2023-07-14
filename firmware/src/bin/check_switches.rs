#![no_std]
#![no_main]

use attiny_hal as hal;

use firmware::ssd;

use firmware::switches::{state2number, Edge, Switches};

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins: hal::Pins = hal::pins!(dp);

    let mut ssd = ssd::Ssd::new(
        pins.pd0.into_output(),
        pins.pd1.into_output(),
        pins.pd2.into_output(),
        pins.pd3.into_output(),
        pins.pd4.into_output(),
        pins.pd5.into_output(),
        pins.pd6.into_output(),
        pins.pb0.into_output(),
    );
    ssd.light_off();

    let mut switches = Switches::new(
        pins.pb1.into_output(),
        pins.pb2.into_output(),
        pins.pb3.into_floating_input(),
    );

    loop {
        let state = switches.state(Edge::No);
        let number = state2number(&state) as usize;
        ssd.display_number(number, false);
    }
}
