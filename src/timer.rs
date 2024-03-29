use core::arch::asm;
use avr_config::CPU_FREQUENCY_HZ;

use ruduino::interrupt::*;
use ruduino::cores::current::Timer16;
#[allow(unused_imports)] use ruduino::modules::Timer16::{};
use ruduino::modules::{ClockSource16,WaveformGenerationMode16};

pub fn setup() {
    without_interrupts(|| {
        // Configure timer 1 for CTC mode, with divider of 64
        Timer16::setup()
            .waveform_generation_mode(WaveformGenerationMode16::ClearOnTimerMatchOutputCompare)
            .clock_source(ClockSource16::Prescale64)
            .output_compare_1(Some((CPU_FREQUENCY_HZ / 64 / 60) as u16)) // 60 Hz
            .configure();
    })
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_11() {
    super::tick();
}

pub fn sleep_ms(duration_ms: u16) {
    const CYCLES_PER_MS: u16 = (CPU_FREQUENCY_HZ / 1000) as u16;
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
