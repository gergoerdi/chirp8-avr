use ruduino::Pin;
use ruduino::cores::current::{port};
use ruduino::interrupt::*;
use spi;

pub fn setup() {
    // RAM chip select
    port::D6::set_output(); port::D6::set_high();

    port::D6::set_low();
    spi::sync(0x01);
    spi::sync(0x00);
    port::D6::set_high();
}

pub fn write_ram(addr: u16, value: u8) {
    without_interrupts(|| {
        port::D6::set_low();
        spi::sync(0x02);
        spi::sync((addr >> 8) as u8);
        spi::sync(addr as u8);
        spi::sync(value);
        port::D6::set_high();
    })
}

pub fn read_ram(addr: u16) -> u8 {
    without_interrupts(|| {
        port::D6::set_low();
        spi::sync(0x03);
        spi::sync((addr >> 8) as u8);
        spi::sync(addr as u8);
        let value = spi::sync(0);
        port::D6::set_high();
        value
    })
}
