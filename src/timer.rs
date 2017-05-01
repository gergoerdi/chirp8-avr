use core::prelude::v1::*;
use core::intrinsics::{volatile_load, volatile_store};

use avr::*;
use spi;

static mut COUNTDOWN: u8 = 0;

pub fn setup() {
    unsafe{
        // Configure timer 1 for CTC mode, with divider of 64
        volatile_store(TCCR1B, volatile_load(TCCR1B) | 0b_0000_1011);

        volatile_store(OCR1A, 4167); // 60 Hz

        // Enable CTC interrupt
        volatile_store(TIMSK1, volatile_load(TIMSK1) | 1 << 1);

        // Good to go!
        asm!("SEI")
    }
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_11() {
    if COUNTDOWN > 0 { COUNTDOWN -= 1; }

    super::redraw();
}

pub fn get_countdown() -> u8 {
    unsafe{ COUNTDOWN }
}

pub fn set_countdown(countdown: u8) {
    unsafe{ COUNTDOWN = countdown }
}
