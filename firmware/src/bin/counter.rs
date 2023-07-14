#![no_std]
#![no_main]

use attiny_hal as hal;

use firmware::ssd;
use firmware::switches::{Edge, Switches};

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

    let mut number = 0;

    loop {
        let state = switches.state(Edge::Rising);

        for (i, &s) in state.iter().enumerate() {
            if s {
                if i < 4 {
                    let x = 10usize.pow(i as u32);
                    number = if number + x >= 9999 { 9999 } else { number + x };
                } else {
                    let x = 10usize.pow((i - 4) as u32);
                    number = if number <= x { 0 } else { number - x };
                }
            }
        }

        ssd.display_number(number, false);
    }
}
