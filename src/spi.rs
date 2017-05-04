use avr::*;
use core::intrinsics::{volatile_load, volatile_store};

pub fn setup() {
    unsafe {
        // Set SS to HIGH
        volatile_store(PORTB, volatile_load(PORTB) | (1 << 2));

        // Set up SPI pin directions
        volatile_store(DDRB, volatile_load(DDRB) | (1 << 2) | (1 << 3) | (1 << 5));
        volatile_store(DDRB, volatile_load(DDRB) & !(1 << 4));

        // Turn on SPI
        volatile_store(SPCR, (1 << 6) | (1 << 4));
    }
}

pub fn sync(out: u8) -> u8 {
    unsafe {
        volatile_store(SPDR, out);
        while volatile_load(SPSR) & 1 << 7 == 0 {}
        volatile_load(SPDR)
    }
}
