use ruduino::Pin;
use ruduino::Register;
use ruduino::cores::current::{port,SPCR,SPDR,SPSR};

pub fn setup() {
    // Set up SPI pin directions
    port::B3::set_output();
    port::B5::set_output();
    port::B4::set_input();

    // SS is used for unrelated output
    port::B2::set_output();
    port::B2::set_high();

    // Turn on SPI
    SPCR::set(SPCR::SPE | SPCR::MSTR);
}

pub fn sync(out: u8) -> u8 {
    SPDR::write(out);
    SPSR::wait_until_set(SPSR::SPIF);
    SPDR::read()
}
