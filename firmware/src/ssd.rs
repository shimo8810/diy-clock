use attiny_hal as hal;
use hal::port::mode::*;
use hal::port::*;

use crate::delay::delay_ms;

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

const DOT: u8 = 0xC0;
const E: u8 = 0x9E;
const R: u8 = 0x8C;

pub struct Ssd {
    rclk: Pin<Output, PD0>,
    ser: Pin<Output, PD1>,
    srclk: Pin<Output, PD2>,
    d1: Pin<Output, PD3>,
    d2: Pin<Output, PD4>,
    d3: Pin<Output, PD5>,
    d4: Pin<Output, PD6>,
    dd: Pin<Output, PB0>,
}

#[allow(clippy::too_many_arguments)]
impl Ssd {
    pub fn new(
        rclk: Pin<Output, PD0>,
        ser: Pin<Output, PD1>,
        srclk: Pin<Output, PD2>,
        d1: Pin<Output, PD3>,
        d2: Pin<Output, PD4>,
        d3: Pin<Output, PD5>,
        d4: Pin<Output, PD6>,
        dd: Pin<Output, PB0>,
    ) -> Self {
        Self {
            rclk,
            ser,
            srclk,
            d1,
            d2,
            d3,
            d4,
            dd,
        }
    }

    pub fn light_off(&mut self) {
        self.d1.set_low();
        self.d2.set_low();
        self.d3.set_low();
        self.d4.set_low();
        self.dd.set_low();

        self.rclk.set_low();
        for _ in 0u8..8 {
            self.srclk.set_low();
            self.ser.set_low();
            self.srclk.set_high();
        }
        self.rclk.set_high();
    }

    pub fn set_number(&mut self, n: usize) {
        self.rclk.set_low();
        for s in 0u8..8 {
            self.srclk.set_low();
            if ((NUMS[n] >> s) & 0x1) == 0 {
                self.ser.set_low();
            } else {
                self.ser.set_high();
            }
            self.srclk.set_high();
        }
        self.rclk.set_high();
    }

    pub fn on_dig(&mut self, n: usize) {
        self.d1.set_low();
        self.d2.set_low();
        self.d3.set_low();
        self.d4.set_low();
        self.dd.set_low();

        match n {
            0 => self.d1.set_high(),
            1 => self.d2.set_high(),
            2 => self.d3.set_high(),
            3 => self.d4.set_high(),
            4 => self.dd.set_high(),
            _ => panic!(),
        }
    }

    pub fn dot_on(&mut self) {
        self.rclk.set_low();
        for s in 0u8..8 {
            self.srclk.set_low();
            if ((DOT >> s) & 0x1) == 0 {
                self.ser.set_low();
            } else {
                self.ser.set_high();
            }
            self.srclk.set_high();
        }
        self.rclk.set_high();
    }

    pub fn display_number(&mut self, mut n: usize, dot: bool) {
        let ms = 1;
        self.set_number(n % 10);
        self.on_dig(3);
        delay_ms(ms);
        self.light_off();

        n /= 10;
        self.set_number(n % 10);
        self.on_dig(2);
        delay_ms(ms);
        self.light_off();

        n /= 10;
        self.set_number(n % 10);
        self.on_dig(1);
        delay_ms(ms);
        self.light_off();

        n /= 10;
        self.set_number(n % 10);
        self.on_dig(0);
        delay_ms(ms);
        self.light_off();

        if dot {
            self.dot_on();
            self.on_dig(4);
            delay_ms(ms);
            self.light_off();
        }
    }

    pub fn display_panic(&mut self) {
        let ms = 1;

        self.rclk.set_low();
        for s in 0u8..8 {
            self.srclk.set_low();
            if ((E >> s) & 0x1) == 0 {
                self.ser.set_low();
            } else {
                self.ser.set_high();
            }
            self.srclk.set_high();
        }
        self.rclk.set_high();
        self.on_dig(0);
        delay_ms(ms);
        self.light_off();

        self.rclk.set_low();
        for s in 0u8..8 {
            self.srclk.set_low();
            if ((R >> s) & 0x1) == 0 {
                self.ser.set_low();
            } else {
                self.ser.set_high();
            }
            self.srclk.set_high();
        }
        self.rclk.set_high();
        self.on_dig(1);
        delay_ms(ms);
        self.light_off();

        self.rclk.set_low();
        for s in 0u8..8 {
            self.srclk.set_low();
            if ((R >> s) & 0x1) == 0 {
                self.ser.set_low();
            } else {
                self.ser.set_high();
            }
            self.srclk.set_high();
        }
        self.rclk.set_high();
        self.on_dig(2);
        delay_ms(ms);
        self.light_off();
    }
}
