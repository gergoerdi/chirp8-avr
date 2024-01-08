use avr_progmem::progmem;
use progmem_include_bytes::*;
use chirp8::quirks::*;

progmem! {
    pub static progmem FONT_ROM: [u8;8 * 16] = chirp8::font::FONT_HEX;
}

progmem_include_bytes!(pub PROG_ROM = "image/hidden.ch8");

pub static QUIRKS: Quirks = Quirks {
    shift_vy: false,
    reset_vf: false,
    increment_ptr: false,
    video_wait: true,
    clip_sprites: true,
};
