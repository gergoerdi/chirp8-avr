use ruduino::Pin;
use ruduino::cores::current::{port};
use spi;

pub const SCREEN_WIDTH: u8 = 84;
pub const SCREEN_HEIGHT: u8 = 48;

pub fn setup() {
    // LCD SCE = PD3
    // LCD RST = PD4
    // LCD DC = PD5
    port::D3::set_output();
    port::D4::set_output();
    port::D5::set_output();

    // Reset
    port::D3::set_low();
    port::D4::set_low();
    // delay_ms(500);
    port::D4::set_high();

    // Initialize
    port::D3::set_low(); // Chip select LCD
    port::D5::set_low(); // Set Command mode

    spi::sync(0x20 | 0x1); // Turn on chip, set extended command set
    spi::sync(0x10 | 4); // Set bias to 4 (???)
    spi::sync(0x80 | 60); // Set contrast to 60
    spi::sync(0x20 | 0x2); // Set non-extended command set
    spi::sync(0x08 | 0x4); // Set display mode to normal

    port::D3::set_high(); // Unselect LCD
}

#[inline(never)]
pub fn send<I>(pixels: I) where I: IntoIterator<Item = u8> {
    port::D3::set_low(); // Chip select LCD
    port::D5::set_low(); // Set Command mode

    spi::sync(0x20 | 0x2); // Vertical addressing
    spi::sync(0x80 | 0);   // Set X address
    spi::sync(0x40 | 0);   // Set Y address

    port::D5::set_high(); // Set Data mode
    for stripe in pixels.into_iter() {
        spi::sync(stripe);
    }

    port::D3::set_high(); // Unselect LCD
}
