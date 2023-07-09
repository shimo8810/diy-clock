#![no_std]
#![no_main]

use attiny_hal as hal;
use embedded_hal::blocking::delay::DelayMs;
use panic_halt as _;

pub type Delay = hal::delay::Delay<hal::clock::MHz16>;

pub fn delay_ms(ms: u16) {
    Delay::new().delay_ms(ms);
}

const NUMS: [u8; 10] = [
    0xFC, // 0
    0x60, // 1
    0xDA, // 2
    0xF2, // 3
    0x66, // 4
    0xB6, // 5
    0xBE, // 6
    0xE0, // 7
    0xFE, // 8
    0xF6, // 9
];

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    // digit pin
    let mut d1 = pins.pd3.into_output();
    let mut d2 = pins.pd4.into_output();
    let mut d3 = pins.pd5.into_output();
    let mut d4 = pins.pd6.into_output();

    // serial pin
    let mut rclk = pins.pd0.into_output();
    let mut ser = pins.pd1.into_output();
    let mut srclk = pins.pd2.into_output();

    rclk.set_low();
    for _ in 0..8 {
        srclk.set_low();
        ser.set_low();
        srclk.set_high();
    }
    rclk.set_high();

    loop {
        for i in 0usize..10000 {
            let n0 = i / 1000;

            // データ書き込み
            rclk.set_low();
            for s in 0..8 {
                srclk.set_low();
                if ((NUMS[n0] >> s) & 0x1) == 0 {
                    ser.set_low();
                } else {
                    ser.set_high();
                }
                srclk.set_high();
            }
            rclk.set_high();

            d1.set_high();
            delay_ms(3);

            rclk.set_low();
            for _ in 0..8 {
                srclk.set_low();
                ser.set_low();
                srclk.set_high();
            }
            rclk.set_high();

            d1.set_low();

            let n1 = (i / 100) % 10;

            // データ書き込み
            rclk.set_low();
            for s in 0..8 {
                srclk.set_low();
                if ((NUMS[n1] >> s) & 0x1) == 0 {
                    ser.set_low();
                } else {
                    ser.set_high();
                }
                srclk.set_high();
            }
            rclk.set_high();

            d2.set_high();
            delay_ms(3);

            rclk.set_low();
            for _ in 0..8 {
                srclk.set_low();
                ser.set_low();
                srclk.set_high();
            }
            rclk.set_high();

            d2.set_low();

            let n2 = (i / 10) % 10;

            // データ書き込み
            rclk.set_low();
            for s in 0..8 {
                srclk.set_low();
                if ((NUMS[n2] >> s) & 0x1) == 0 {
                    ser.set_low();
                } else {
                    ser.set_high();
                }
                srclk.set_high();
            }
            rclk.set_high();

            d3.set_high();
            delay_ms(3);

            rclk.set_low();
            for _ in 0..8 {
                srclk.set_low();
                ser.set_low();
                srclk.set_high();
            }
            rclk.set_high();

            d3.set_low();

            let n3 = i % 10;

            // データ書き込み
            rclk.set_low();
            for s in 0..8 {
                srclk.set_low();
                if ((NUMS[n3] >> s) & 0x1) == 0 {
                    ser.set_low();
                } else {
                    ser.set_high();
                }
                srclk.set_high();
            }
            rclk.set_high();

            d4.set_high();
            delay_ms(3);

            rclk.set_low();
            for _ in 0..8 {
                srclk.set_low();
                ser.set_low();
                srclk.set_high();
            }
            rclk.set_high();

            d4.set_low();
        }
    }
}
