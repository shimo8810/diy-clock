use attiny_hal as hal;

use hal::port::mode::{Floating, Input, Output};
use hal::port::Pin;
use hal::port::{PB1, PB2, PB3};

const MAP: [usize; 8] = [2, 3, 4, 5, 7, 6, 1, 0];
const NUM_REP: usize = 10;
const NUM_SWITCHES: usize = 8;

pub enum Edge {
    No,
    Rising,
    Falling,
}

pub struct Switches {
    clock: Pin<Output, PB1>,
    latch: Pin<Output, PB2>,
    data: Pin<Input<Floating>, PB3>,
    prev: u8,
}

#[allow(clippy::too_many_arguments)]
impl Switches {
    pub fn new(
        mut clock: Pin<Output, PB1>,
        mut latch: Pin<Output, PB2>,
        data: Pin<Input<Floating>, PB3>,
    ) -> Self {
        latch.set_high();
        clock.set_low();
        Self {
            clock,
            latch,
            data,
            prev: 0,
        }
    }

    pub fn shiftin(&mut self) -> u8 {
        self.latch.set_low();
        self.latch.set_high();

        let mut v = if self.data.is_high() { 1 } else { 0 } << MAP[0];

        for &s in MAP.iter().skip(1) {
            self.clock.set_high();
            self.clock.set_low();
            v += if self.data.is_high() { 1 } else { 0 } << s;
        }

        v
    }

    pub fn state(&mut self, edge: Edge) -> [bool; NUM_SWITCHES] {
        // prevent chattering
        let state = (0..NUM_REP)
            .map(|_| self.shiftin())
            .fold(0xffu8, |x, y| x & y);

        let mut res = [false; NUM_SWITCHES];

        for (i, r) in res.iter_mut().enumerate() {
            let s = (state >> i) & 0x1;
            let p = (self.prev >> i) & 0x1;
            *r = match edge {
                Edge::No => s == 1,
                Edge::Rising => (s != p) && (s == 1),
                Edge::Falling => (s != p) && (s == 0),
            };
        }

        self.prev = state;

        res
    }
}

pub fn state2number(state: &[bool; NUM_SWITCHES]) -> u8 {
    state
        .iter()
        .enumerate()
        .fold(0u8, |s, (i, &b)| s + ((b as u8) << i))
}
