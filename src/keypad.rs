use core::prelude::v1::*;
use core::intrinsics::{volatile_load, volatile_store};

use avr::*;
use timer;

pub fn setup() {
    unsafe {
        volatile_store(DDRB, volatile_load(DDRB) | 1 << 1 | 1 << 2);
        volatile_store(PORTB, volatile_load(PORTB) | 1 << 1 | 1 << 2);

        volatile_store(DDRC, volatile_load(DDRC) | 1 << 0 | 1 << 1);
        volatile_store(PORTC, volatile_load(PORTC) | 1 << 0 | 1 << 1);

        // Set up input pins (with pull-up resistors enabled)
        volatile_store(DDRC, volatile_load(DDRC) & !(1 << 2 | 1 << 3 | 1 << 4 | 1 << 5));
        // volatile_store(PORTC, volatile_load(DDRC) | (1 << 2 | 1 << 3 | 1 << 4 | 1 << 5));
    }
}

pub fn scan_key_row(row: u8) -> u8 {
    unsafe {
        match row {
            0 => { volatile_store(PORTC, volatile_load(PORTC) & !(1 << 1)) },
            1 => { volatile_store(PORTC, volatile_load(PORTC) & !(1 << 0)) },
            2 => { volatile_store(PORTB, volatile_load(PORTB) & !(1 << 2)) },
            3 => { volatile_store(PORTB, volatile_load(PORTB) & !(1 << 1)) },
            _ => {}
        };

        asm!("NOP");

        let mut result = 0;
        let mut buf = 0;
        let mut success_count = 0;

        while success_count < 20 {
            result = volatile_load(PINC);
            success_count = if buf == result { success_count + 1} else { 0 };
            buf = result;
            timer::sleep_ms(1);
        }

        volatile_store(PORTC, volatile_load(PORTC) | 1 << 0 | 1 << 1);
        volatile_store(PORTB, volatile_load(PORTB) | 1 << 1 | 1 << 2);

        (if !result & 1 << 5 == 0 {1} else {0}) << 0 |
        (if !result & 1 << 4 == 0 {1} else {0}) << 1 |
        (if !result & 1 << 3 == 0 {1} else {0}) << 2 |
        (if !result & 1 << 2 == 0 {1} else {0}) << 3
    }
}
