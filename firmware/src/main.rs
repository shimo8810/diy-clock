#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod delay;
mod ssd;

use attiny_hal as hal;
use avr_device::attiny2313::tc1::tccr1b::CS1_A;
use avr_device::attiny2313::TC1;
use core::mem;
use delay::delay_ms;
use panic_halt as _;

struct InterruptState {
    h: u8,
    m: u8,
    s: u8,
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

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

    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        INTERRUPT_STATE = mem::MaybeUninit::new(InterruptState { h: 10, m: 48, s: 0 });
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

    let timer1: avr_device::attiny2313::TC1 = dp.TC1;

    rig_timer(&timer1);
    unsafe {
        // SAFETY: Not inside a critical section and any non-atomic operations have been completed
        // at this point.
        avr_device::interrupt::enable();
    }

    let mut buzzer = pins.pb4.into_output();
    let ms = 1;

    let mut psec = 0;
    let mut doton = true;

    loop {
        // buzzer.set_high();
        // delay_ms(ms);
        // buzzer.set_low();
        // delay_ms(ms);

        // for n in 0usize..10000 {
        let state = unsafe {
            // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
            // initialized so this ISR will never run when LED is uninitialized.
            &mut *INTERRUPT_STATE.as_mut_ptr()
        };

        let number = state.h as usize * 100 + state.m as usize;
        if state.s != psec {
            // for _ in 0..5 {
            //     buzzer.set_high();
            //     delay_ms(ms);
            //     buzzer.set_low();
            //     delay_ms(ms);
            // }
            doton = !doton;
            psec = state.s;
        }
        ssd.display_number(number, doton);
        // }
    }
}
pub const fn calc_overflow(clock_hz: u32, target_hz: u32, prescale: u32) -> u32 {
    /*
    https://github.com/Rahix/avr-hal/issues/75
    reversing the formula F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
     */
    clock_hz / target_hz / prescale - 1
}

pub fn rig_timer(timer1: &TC1) {
    use hal::clock::Clock;
    const CLOCK_FREQUENCY_HZ: u32 = hal::clock::MHz16::FREQ;
    const CLOCK_SOURCE: CS1_A = CS1_A::PRESCALE_256;
    let clock_divisor: u32 = match CLOCK_SOURCE {
        CS1_A::DIRECT => 1,
        CS1_A::PRESCALE_8 => 8,
        CS1_A::PRESCALE_64 => 64,
        CS1_A::PRESCALE_256 => 256,
        CS1_A::PRESCALE_1024 => 1024,
        CS1_A::NO_CLOCK | CS1_A::EXT_FALLING | CS1_A::EXT_RISING => {
            panic!()
        }
    };

    let ticks = calc_overflow(CLOCK_FREQUENCY_HZ, 1, clock_divisor) as u16;

    timer1.tccr1a.write(|w| w.wgm1().bits(0b00));
    timer1.tccr1b.write(|w| {
        w.cs1()
            //.prescale_256()
            .variant(CLOCK_SOURCE)
            .wgm1()
            .bits(0b01)
    });

    timer1.ocr1a.write(|w| w.bits(ticks));
    timer1.timsk.write(|w| w.ocie1a().set_bit()); //enable this specific interrupt
}

#[avr_device::interrupt(attiny2313)]
fn TIMER1_COMPA() {
    let state = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
        // initialized so this ISR will never run when LED is uninitialized.
        &mut *INTERRUPT_STATE.as_mut_ptr()
    };
    state.s += 1;
    if state.s >= 60 {
        state.s = 0;
        state.m += 1;
    }

    if state.m >= 60 {
        state.m = 0;
        state.h += 1;
    }

    if state.h >= 24 {
        state.h = 0;
    }
    // state.blinker.toggle();
}
