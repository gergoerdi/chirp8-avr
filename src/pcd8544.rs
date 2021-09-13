use core::intrinsics::{volatile_load, volatile_store};

use avr::*;
use spi;

pub fn setup() {
    unsafe {
        // LCD SCE = PD3
        // LCD RST = PD4
        // LCD DC = PD5
        volatile_store(DDRD, volatile_load(DDRD) | 1 << 3 | 1 << 4 | 1 << 5);

        // Reset
        volatile_store(PORTD, volatile_load(PORTD) & !(1 << 3 | 1 << 4));
        // delay_ms(500);
        volatile_store(PORTD, volatile_load(PORTD) | 1 << 4);

        // Initialize
        volatile_store(PORTD, volatile_load(PORTD) & !(1<<3)); // Chip select LCD
        volatile_store(PORTD, volatile_load(PORTD) & !(1<<5)); // Set Command mode

        spi::sync(0x20 | 0x1); // Turn on chip, set extended command set
        spi::sync(0x10 | 4); // Set bias to 4 (???)
        spi::sync(0x80 | 60); // Set contrast to 60
        spi::sync(0x20 | 0x2); // Set non-extended command set
        spi::sync(0x08 | 0x4); // Set display mode to normal

        volatile_store(PORTD, volatile_load(PORTD) | 1 << 3); // Unselect LCD
    }
}

use chirp8::peripherals::{SCREEN_WIDTH, SCREEN_HEIGHT};

#[inline(never)]
pub fn send(pixels: &[[u8; (SCREEN_HEIGHT / 8) as usize]; SCREEN_WIDTH as usize]) {
    unsafe {
        volatile_store(PORTD, volatile_load(PORTD) & !(1 << 3)); // Chip select LCD
        volatile_store(PORTD, volatile_load(PORTD) & !(1 << 5)); // Set Command mode

        spi::sync(0x20 | 0x2); // Vertical addressing
        spi::sync(0x80 | 0);   // Set X address
        spi::sync(0x40 | 0);   // Set Y address

        volatile_store(PORTD, volatile_load(PORTD) | 1 << 5); // Set Data mode
        for col in pixels.iter() {
            for &pixel in col.iter() {
                spi::sync(pixel);
            }
        }

        volatile_store(PORTD, volatile_load(PORTD) | 1 << 3); // Unselect LCD
    }
}
