use core::arch::asm;

use ruduino::Pin;
use ruduino::Register;
use ruduino::cores::current::{port,PINC};
use timer;

pub fn setup() {
    port::B1::set_output(); port::B1::set_high();
    port::B2::set_output(); port::B2::set_high();

    port::C0::set_output(); port::C0::set_high();
    port::C1::set_output(); port::C1::set_high();

    // Set up input pins (with pull-up resistors enabled)
    port::C2::set_input();
    port::C3::set_input();
    port::C4::set_input();
    port::C5::set_input();
    // port::C2::set_high();
    // port::C3::set_high();
    // port::C4::set_high();
    // port::C5::set_high();
}

pub fn scan_key_row(row: u8) -> u8 {
    match row {
        0 => { port::C1::set_low() },
        1 => { port::C0::set_low() },
        2 => { port::B2::set_low() },
        3 => { port::B1::set_low() },
        _ => {}
    };

    unsafe { asm!("NOP"); }

    let mut result = 0;
    let mut buf = 0;
    let mut success_count = 0;

    while success_count < 20 {
        result = PINC::read();
        success_count = if buf == result { success_count + 1} else { 0 };
        buf = result;
            timer::sleep_ms(1);
        }

    port::C0::set_high();
    port::C1::set_high();
    port::B1::set_high();
    port::B2::set_high();

    (if !result & 1 << 5 == 0 {1} else {0}) << 0 |
    (if !result & 1 << 4 == 0 {1} else {0}) << 1 |
    (if !result & 1 << 3 == 0 {1} else {0}) << 2 |
    (if !result & 1 << 2 == 0 {1} else {0}) << 3
}
