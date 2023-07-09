use attiny_hal as hal;
use embedded_hal::blocking::delay::DelayMs;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;

pub fn delay_ms(ms: u16) {
    Delay::new().delay_ms(ms);
}
