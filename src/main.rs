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

    #[inline(never)]
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

    #[inline(never)]
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

extern {
    fn font_rom_size() -> u16;
    fn read_font_rom(offset: u16) -> u8;

    fn prog_rom_size() -> u16;
    fn read_prog_rom(offset: u16) -> u8;
}

fn upload_font(board: &Board) {
    for offset in 0..unsafe{font_rom_size()} {
        board.write_ram(offset, unsafe{ read_font_rom(offset) });
    }
}

fn upload_prog(board: &Board) {
    let base = 0x0200;

    for offset in 0..unsafe{prog_rom_size()} {
        board.write_ram(base + offset, unsafe{ read_prog_rom(offset) });
    }
}

#[no_mangle]
pub extern fn main() {
    let io: &Board = &BOARD;

    spi::setup();
    pcd8544::setup();
    keypad::setup();
    serial_ram::setup();
    // timer::setup();

    draw_test_pattern();

    upload_font(io);
    upload_prog(io);

    let mut machine = chip8::machine::Machine::new();

    loop {
        machine.step(io);
    }
}
