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
mod keypad;

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

    fn get_pixel(&self, x: Byte, y: Byte) -> bool { false }
    fn redraw(&self) {}


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

    fn read_ram(&self, addr: Addr) -> Byte { 0x00 }
    fn write_ram(&self, addr: Addr, v: Byte) {}

    fn get_random(&self) -> Byte { 0x42 }
}

pub unsafe fn redraw() {
    if FB_DIRTY {
        pcd8544::send(&FB_PIXELS);
        FB_DIRTY = false;
    }
}

#[no_mangle]
pub extern fn main() {
    unsafe {
        spi::setup();
        pcd8544::setup();
        keypad::setup();
        timer::setup();

        BOARD.set_pixel(10, 10, true);

        loop {}
    }
}
