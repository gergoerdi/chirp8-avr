use core::prelude::v1::*;
use core::intrinsics::{volatile_load, volatile_store};

use avr::*;
use spi;

pub fn setup() {
    unsafe {
        volatile_store(DDRD, volatile_load(DDRD) | 1 << 6);
        volatile_store(PORTD, volatile_load(PORTD) | 1 << 6);

        volatile_store(DDRB, volatile_load(DDRB) | 1 << 0);
        volatile_store(PORTB, volatile_load(PORTB) | 1 << 0);

        volatile_store(PORTD, volatile_load(PORTD) & !(1 << 6));
        spi::sync(0x01);
        spi::sync(0x00);
        volatile_store(PORTD, volatile_load(PORTD) | 1 << 6);
    }
}

#[inline(never)]
pub fn write_ram(addr: u16, value: u8) {
    unsafe {
        asm!("CLI");

        volatile_store(PORTD, volatile_load(PORTD) & !(1 << 6));
        spi::sync(0x02);
        spi::sync((addr >> 8) as u8);
        spi::sync(addr as u8);
        spi::sync(value);
        volatile_store(PORTD, volatile_load(PORTD) | (1 << 6));

        asm!("SEI");
    }
}

#[inline(never)]
pub fn read_ram(addr: u16) -> u8 {
    unsafe {
        asm!("CLI");

        volatile_store(PORTD, volatile_load(PORTD) & !(1 << 6));
        spi::sync(0x03);
        spi::sync((addr >> 8) as u8);
        spi::sync(addr as u8);
        let value = spi::sync(0);
        volatile_store(PORTD, volatile_load(PORTD) | (1 << 6));

        asm!("SEI");

        value
    }
}
