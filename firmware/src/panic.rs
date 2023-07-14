use attiny_hal as hal;

use crate::delay::delay_ms;
use crate::ssd::Ssd;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();
    let dp = unsafe { hal::Peripherals::steal() };
    let pins = hal::pins!(dp);

    let mut ssd = Ssd::new(
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
        for _ in 0..200 {
            ssd.display_panic();
        }

        for _ in 0..200 {
            ssd.light_off();
            delay_ms(5);
        }
    }
}
