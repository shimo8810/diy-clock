use attiny_hal as hal;

use hal::port::mode::*;
use hal::port::*;

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

pub struct Ssd {
    pub rclk: Pin<Output, PD0>,
    pub ser: Pin<Output, PD1>,
    pub srclk: Pin<Output, PD2>,
    pub d1: Pin<Output, PD3>,
    pub d2: Pin<Output, PD4>,
    pub d3: Pin<Output, PD5>,
    pub d4: Pin<Output, PD6>,
    pub dd: Pin<Output, PB0>,
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
}
