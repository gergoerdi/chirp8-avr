pub const DDRB:   *mut u8  = 0x24 as *mut u8;
pub const PORTB:  *mut u8  = 0x25 as *mut u8;

pub const DDRD:   *mut u8  = 0x2a as *mut u8;
pub const PORTD:  *mut u8  = 0x2b as *mut u8;

pub const TCCR1B: *mut u8  = 0x81 as *mut u8;
pub const TIMSK1: *mut u8  = 0x6f as *mut u8;
pub const OCR1A:  *mut u16 = 0x88 as *mut u16;

// SPI
pub const SPCR:   *mut u8  = 0x4c as *mut u8;
pub const SPSR:   *mut u8  = 0x4d as *mut u8;
pub const SPDR:   *mut u8  = 0x4e as *mut u8;
