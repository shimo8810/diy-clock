#![no_std]
#![no_main]

mod delay;
mod ssd;

use attiny_hal as hal;

use panic_halt as _;

use delay::delay_ms;

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

    loop {
        for i in 0usize..10000 {
            ssd.dot_on();
            ssd.on_dig(4);
            delay_ms(2);
            ssd.light_off();

            let n0 = i / 1000;
            ssd.set_number(n0);
            ssd.on_dig(0);
            delay_ms(2);
            ssd.light_off();

            let n1 = (i / 100) % 10;
            ssd.set_number(n1);
            ssd.on_dig(1);
            delay_ms(2);
            ssd.light_off();

            let n2 = (i / 10) % 10;
            ssd.set_number(n2);
            ssd.on_dig(2);
            delay_ms(2);
            ssd.light_off();

            let n3 = i % 10;
            ssd.set_number(n3);
            ssd.on_dig(3);
            delay_ms(2);
            ssd.light_off();
        }
    }
}
