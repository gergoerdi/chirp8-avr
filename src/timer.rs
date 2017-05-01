use core::prelude::v1::*;
use core::intrinsics::{volatile_load, volatile_store};

use avr::*;
use spi;

#[allow(non_upper_case_globals)]
pub static mut countdown: u8 = 0;

pub unsafe fn setup() {
    // Configure timer 1 for CTC mode, with divider of 64
    volatile_store(TCCR1B, volatile_load(TCCR1B) | 0b_0000_1011);

    volatile_store(OCR1A, 4167); // 60 Hz

    // Enable CTC interrupt
    volatile_store(TIMSK1, volatile_load(TIMSK1) | 1 << 1);

    // Good to go!
    unsafe { asm!("SEI") }
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_11() {
    if countdown > 0 { countdown -= 1; }

    super::redraw();
}
