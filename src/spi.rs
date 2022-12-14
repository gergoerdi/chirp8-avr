use ruduino::Pin;
use ruduino::cores::current::*;
use ruduino::modules::HardwareSpi;

pub fn setup() {
    // Set up SPI pin directions
    <Spi as HardwareSpi>::MasterInSlaveOut::set_input();
    <Spi as HardwareSpi>::MasterOutSlaveIn::set_output();
    <Spi as HardwareSpi>::Clock::set_output();

    // Turn on SPI
    <Spi as HardwareSpi>::set_master();
    <Spi as HardwareSpi>::enable();
}

pub fn sync(out: u8) -> u8 {
    <Spi as HardwareSpi>::send_byte(out);
    <Spi as HardwareSpi>::receive_byte()
}
