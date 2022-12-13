use core::arch::asm;

use ruduino::interrupt::*;
use ruduino::Register;
use ruduino::cores::current::{TCCR1B,OCR1A,TIMSK1};

static mut COUNTDOWN: u8 = 0;

pub fn setup() {
    without_interrupts(|| {
        // Configure timer 1 for CTC mode, with divider of 64
        TCCR1B::write(TCCR1B::read() | 0b_0000_1011);
        OCR1A::write(4167u16); // 60 Hz

        // Enable CTC interrupt
        TIMSK1::set(TIMSK1::OCIE1A);
    })
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

pub fn sleep_ms(duration_ms: u16) {
    const FREQUENCY_HZ: u32 = 12_000_000;
    const CYCLES_PER_MS: u16 = (FREQUENCY_HZ / 1000) as u16;
    const CYCLES_PER_INNER_LOOP: u16 = 6;
    const INNER_LOOP_ITERATIONS: u16 = CYCLES_PER_MS / CYCLES_PER_INNER_LOOP;

    let mut outer = 0;
    while outer < duration_ms {
        let mut inner = 0;
        while inner < INNER_LOOP_ITERATIONS {
            unsafe { asm!(""); }
            inner += 1;
        }
        outer += 1;
    }
}
