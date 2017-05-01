#![feature(no_core)]
#![feature(lang_items)]
#![feature(fundamental)]
#![feature(intrinsics)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(unboxed_closures)]
#![feature(associated_type_defaults)]
#![feature(asm)]
#![feature(abi_avr_interrupt)]
#![feature(unwind_attributes)]

#![no_core]
#![no_main]

#![feature(never_type)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]
#![feature(prelude_import)]
#![feature(staged_api)]

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_unsafe)]

extern crate libcore_mini as core;
use core::prelude::v1::*;
use core::intrinsics::{volatile_load, volatile_store};
use core::option;
use core::iter;
use core::ops;

extern crate chip8_engine as chip8;

pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(state: (), exception_object: *mut (), context: *mut ()) -> () {
    }

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(msg: (), file: &'static str, line: u32) -> ! {
        loop{}
    }
}

mod avr;
use avr::*;
mod spi;
mod pcd8544;
pub mod timer;
use timer::sleep_ms;
mod keypad;
mod serial_ram;

use chip8::*;
use chip8::prelude::*;
use chip8::peripherals::*;

struct Board {
}

static BOARD: Board = Board{};

static mut FB_PIXELS: [[u8; (SCREEN_HEIGHT / 8) as usize]; SCREEN_WIDTH as usize] = [[0; (SCREEN_HEIGHT / 8) as usize]; SCREEN_WIDTH as usize];

static mut FB_DIRTY: bool = false;

impl Peripherals for Board {
    fn keep_running(&self) -> bool { true }

    fn clear_pixels(&self) {
        unsafe {
            for mut col in FB_PIXELS.iter_mut() {
                for mut pixel in col.iter_mut() {
                    *pixel = 0;
                }
            }
            FB_DIRTY = true;
        }
    }

    fn set_pixel(&self, x: Byte, y: Byte, v: bool) {
        let row = y >> 3;
        let offset = y - (row << 3);
        let mask = 1 << offset;
        let bit = (if v {1} else {0}) << offset;

        unsafe {
            FB_PIXELS[x as usize][row as usize] = (FB_PIXELS[x as usize][row as usize] & !mask) | bit;
            FB_DIRTY = true;
        }
    }

    fn get_pixel(&self, x: Byte, y: Byte) -> bool {
        let row = y >> 3;
        let offset = y - (row << 3);
        let mask = 1 << offset;

        unsafe{
            FB_PIXELS[x as usize][row as usize] & mask != 0
        }
    }

    fn redraw(&self) {
        // Not really needed? timer will take care of it?
    }

    fn scan_key_row(&self, row: Byte) -> Byte {
        keypad::scan_key_row(row)
    }

    fn set_timer(&self, v: Byte) {
        timer::set_countdown(v)
    }

    fn get_timer(&self) -> Byte {
        timer::get_countdown()
    }


    fn set_sound(&self, v: Byte) {
        // Not implemented on this board
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        serial_ram::read_ram(addr)
    }

    fn write_ram(&self, addr: Addr, v: Byte) {
        serial_ram::write_ram(addr, v)
    }

    fn get_random(&self) -> Byte { 0x42 }
}

pub unsafe fn redraw() {
    if FB_DIRTY {
        pcd8544::send(&FB_PIXELS);
        FB_DIRTY = false;
    }
}

fn draw_test_pattern() {
    for i in 0..10 {
        BOARD.set_pixel(i, SCREEN_HEIGHT - (i + 1), false);
        BOARD.set_pixel(i, i, true);
    }
    sleep_ms(500);

    for i in 0..10 {
        BOARD.set_pixel(i, SCREEN_HEIGHT - (i + 1), true);
        BOARD.set_pixel(i, i, false);
    }
    sleep_ms(500);
    BOARD.clear_pixels();
}

const FONT_ROM: [u8; 16 * 8] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x00, 0x00, 0x00,
    0x20, 0x60, 0x20, 0x20, 0x70, 0x00, 0x00, 0x00,
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00,
    0x90, 0x90, 0xF0, 0x10, 0x10, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x10, 0x20, 0x40, 0x40, 0x00, 0x00, 0x00,
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x90, 0xF0, 0x10, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x90, 0xF0, 0x90, 0x90, 0x00, 0x00, 0x00,
    0xE0, 0x90, 0xE0, 0x90, 0xE0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0x80, 0x80, 0xF0, 0x00, 0x00, 0x00,
    0xE0, 0x90, 0x90, 0x90, 0xE0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x80, 0xF0, 0x00, 0x00, 0x00,
    0xF0, 0x80, 0xF0, 0x80, 0x80, 0x00, 0x00, 0x00
];

fn upload_font(board: &Board) {
    for (addr, &b) in FONT_ROM.iter().enumerate() {
        board.write_ram(addr as Addr, b);
    }
}

const PROG_ROM: [u8; 94] = [
  0xf5, 0x0a, 0xa2, 0x5e, 0xf5, 0x1e, 0xf0, 0x65, 0x6e, 0x01, 0x80, 0xe3,
  0xa2, 0x5e, 0xf5, 0x1e, 0xf0, 0x55, 0x67, 0x00, 0xf5, 0x29, 0x00, 0xe0,
  0xd7, 0x75, 0x22, 0x1e, 0x12, 0x00, 0x61, 0x00, 0x62, 0x17, 0x63, 0x04,
  0x41, 0x10, 0x00, 0xee, 0xa2, 0x4e, 0xf1, 0x1e, 0xf0, 0x65, 0xa2, 0x5e,
  0xf0, 0x1e, 0xf0, 0x65, 0x40, 0x00, 0x12, 0x3c, 0xf1, 0x29, 0xd2, 0x35,
  0x71, 0x01, 0x72, 0x05, 0x64, 0x03, 0x84, 0x12, 0x34, 0x00, 0x12, 0x24,
  0x62, 0x17, 0x73, 0x06, 0x12, 0x24, 0x01, 0x02, 0x03, 0x0c, 0x04, 0x05,
  0x06, 0x0d, 0x07, 0x08, 0x09, 0x0e, 0x0a, 0x00, 0x0b, 0x0f
];


fn upload_prog(board: &Board) {
    let base = 0x0200;

    for (addr, &b) in PROG_ROM.iter().enumerate() {
        board.write_ram(base + addr as Addr, b);
    }
}

#[no_mangle]
pub extern fn main() {
    let io: &Board = &BOARD;

    spi::setup();
    pcd8544::setup();
    keypad::setup();
    timer::setup();
    serial_ram::setup();

    draw_test_pattern();

    upload_font(io);
    upload_prog(io);

    let mut machine = chip8::machine::Machine::new();

    while io.keep_running() {
        machine.step(io);
    }
}
