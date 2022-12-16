#![feature(abi_avr_interrupt)]
#![feature(core_intrinsics)]
#![feature(asm_experimental_arch)]

#![no_std]
#![no_main]

#![allow(unused_variables)]
#![allow(dead_code)]

extern crate chirp8_engine as chirp8;
extern crate avr_std_stub;
extern crate ruduino;
extern crate avr_progmem;
extern crate progmem_include_bytes;

mod spi;
mod pcd8544;
mod timer;
mod keypad;
mod serial_ram;
mod rom;

use rom::*;
use timer::sleep_ms;

use chirp8::prelude::*;
use chirp8::peripherals::*;

struct Board {
}

static mut BOARD: Board = Board{};

static mut FB_PIXELS: [[u8; (pcd8544::SCREEN_HEIGHT / 8) as usize]; pcd8544::SCREEN_WIDTH as usize] = [[0; (pcd8544::SCREEN_HEIGHT / 8) as usize]; pcd8544::SCREEN_WIDTH as usize];

static mut FB_DIRTY: bool = false;

fn xy_from_chirp8(x: u8, y: u8) -> (u8, u8) {
    let dx = (pcd8544::SCREEN_WIDTH - SCREEN_WIDTH) / 2;
    let dy = (pcd8544::SCREEN_HEIGHT - SCREEN_HEIGHT) / 2;
    (x + dx, y + dy)
}

impl Peripherals for Board {
    fn keep_running(&self) -> bool { true }

    fn clear_pixels(&mut self) {
        unsafe {
            for col in FB_PIXELS.iter_mut() {
                for pixel in col.iter_mut() {
                    *pixel = 0;
                }
            }
            FB_DIRTY = true;
        }
    }

    #[inline(never)]
    fn set_pixel(&mut self, x: Byte, y: Byte, v: bool) {
        let (x, y) = xy_from_chirp8(x, y);

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
        let (x, y) = xy_from_chirp8(x, y);

        let row = y >> 3;
        let offset = y - (row << 3);
        let mask = 1 << offset;

        unsafe{
            FB_PIXELS[x as usize][row as usize] & mask != 0
        }
    }

    fn redraw(&mut self) {
        // Not really needed? timer will take care of it?
    }

    fn scan_key_row(&self, row: Byte) -> Byte {
        keypad::scan_key_row(row)
    }

    fn set_timer(&mut self, v: Byte) {
        timer::set_countdown(v)
    }

    fn get_timer(&self) -> Byte {
        timer::get_countdown()
    }


    fn set_sound(&mut self, v: Byte) {
        // Not implemented on this board
    }

    fn read_ram(&self, addr: Addr) -> Byte {
        serial_ram::read_ram(addr)
    }

    fn write_ram(&mut self, addr: Addr, v: Byte) {
        serial_ram::write_ram(addr, v)
    }

    fn get_random(&mut self) -> Byte { 0x42 }
}

#[inline(never)]
pub unsafe fn redraw() {
    if FB_DIRTY {
        pcd8544::send(&FB_PIXELS);
        FB_DIRTY = false;
    }
}

fn draw_test_pattern(board: &mut Board) {
    for i in 0..SCREEN_HEIGHT {
        board.set_pixel(i, SCREEN_HEIGHT - (i + 1), false);
        board.set_pixel(i, i, true);
    }

    sleep_ms(500);

    for i in 0..SCREEN_HEIGHT {
        board.set_pixel(i, SCREEN_HEIGHT - (i + 1), true);
        board.set_pixel(i, i, false);
    }
    sleep_ms(500);
    board.clear_pixels();
}

#[no_mangle]
pub extern fn main() {
    let io: &mut Board = unsafe{ &mut BOARD };

    spi::setup();
    pcd8544::setup();
    keypad::setup();
    serial_ram::setup();
    timer::setup();

    draw_test_pattern(unsafe{ &mut BOARD });

    upload_font(io);
    upload_prog(io);

    let mut cpu = chirp8::cpu::CPU::new();

    loop {
        cpu.step(io);
    }
}
