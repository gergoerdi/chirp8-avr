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
use core::prelude::*;
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

use chip8::*;
use chip8::prelude::*;
use chip8::peripherals::*;

struct Board {
}

impl Peripherals for Board {
    fn keep_running(&self) -> bool { true }

    fn clear_pixels(&self) {}

    fn set_pixel(&self, x: Byte, y: Byte, v: bool) {}
    fn get_pixel(&self, x: Byte, y: Byte) -> bool { false }
    fn redraw(&self) {}

    fn scan_key_row(&self, row: Byte) -> Byte { 0x0 }

    fn set_timer(&self, v: Byte) {}
    fn get_timer(&self) -> Byte { 0x00 }
    fn set_sound(&self, v: Byte) {}

    fn read_ram(&self, addr: Addr) -> Byte { 0x00 }
    fn write_ram(&self, addr: Addr, v: Byte) {}

    fn get_random(&self) -> Byte { 0x42 }
}

#[no_mangle]
pub extern fn main() {
    unsafe {
        spi::setup();
        pcd8544::setup();

        let mut pixels = [[0; (SCREEN_HEIGHT / 8) as usize]; SCREEN_WIDTH as usize];
        pixels[2][2] = 0xff;
        pcd8544::send(&pixels);

        loop {}
    }
}
