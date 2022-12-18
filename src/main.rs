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
mod framebuffer;

use rom::*;
use timer::sleep_ms;
use framebuffer::FBIter;

use chirp8::prelude::*;
use chirp8::peripherals::*;

struct Board {
    fb_dirty: bool,
    countdown: u8
}

impl Board {
    pub const fn new() -> Board {
        Board {
            fb_dirty: false,
            countdown: 0
        }
    }
}

static mut BOARD: Board = Board::new();
static mut FB_ROWS: [u64; SCREEN_HEIGHT as usize] = [0; SCREEN_HEIGHT as usize];

impl Peripherals for Board {
    fn keep_running(&self) -> bool { true }

    #[inline(never)]
    fn get_pixel_row(&self, y: u8) -> u64 {
        unsafe{ FB_ROWS[y as usize] }
    }

    #[inline(never)]
    fn set_pixel_row(&mut self, y: u8, row: u64) {
        unsafe {
            FB_ROWS[y as usize] = row;
        }
        self.fb_dirty = true;
    }

    fn redraw(&mut self) {
        // Not really needed? timer will take care of it?
    }

    fn get_keys(&self) -> u16 {
        let row0 = keypad::scan_key_row(0);
        let row1 = keypad::scan_key_row(1);
        let row2 = keypad::scan_key_row(2);
        let row3 = keypad::scan_key_row(3);

        (if (row3 & 1 << 1) == 0 {0} else {1}) << 0x0 |
        (if (row0 & 1 << 0) == 0 {0} else {1}) << 0x1 |
        (if (row0 & 1 << 1) == 0 {0} else {1}) << 0x2 |
        (if (row0 & 1 << 2) == 0 {0} else {1}) << 0x3 |
        (if (row1 & 1 << 0) == 0 {0} else {1}) << 0x4 |
        (if (row1 & 1 << 1) == 0 {0} else {1}) << 0x5 |
        (if (row1 & 1 << 2) == 0 {0} else {1}) << 0x6 |
        (if (row2 & 1 << 0) == 0 {0} else {1}) << 0x7 |
        (if (row2 & 1 << 1) == 0 {0} else {1}) << 0x8 |
        (if (row2 & 1 << 2) == 0 {0} else {1}) << 0x9 |

        (if (row3 & 1 << 0) == 0 {0} else {1}) << 0xa |
        (if (row3 & 1 << 2) == 0 {0} else {1}) << 0xb |
        (if (row0 & 1 << 3) == 0 {0} else {1}) << 0xc |
        (if (row1 & 1 << 3) == 0 {0} else {1}) << 0xd |
        (if (row2 & 1 << 3) == 0 {0} else {1}) << 0xe |
        (if (row3 & 1 << 3) == 0 {0} else {1}) << 0xf
    }

    fn set_timer(&mut self, v: Byte) {
        self.countdown = v
    }

    fn get_timer(&self) -> Byte {
        self.countdown
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
pub fn redraw() {
    let board = unsafe{ &mut BOARD };
    if board.fb_dirty {
        pcd8544::send(FBIter::new(unsafe{ &FB_ROWS }));
        board.fb_dirty = false;
    }
}

pub fn tick() {
    let board = unsafe{ &mut BOARD };
    if board.countdown > 0 { board.countdown -= 1; };
}

fn draw_test_pattern(board: &mut Board) {
    let dx = (SCREEN_WIDTH - SCREEN_HEIGHT) / 2;

    let mut row: u64 = 1 << 47;
    for i in 0..32 {
        board.set_pixel_row(i, row);
        row >>= 1;
    }
    sleep_ms(500);

    let mut row: u64 = 1 << 47;
    for i in (0..32).rev() {
        board.set_pixel_row(i, row);
        row >>= 1;
    }
    sleep_ms(500);

    for i in 0..SCREEN_HEIGHT {
        board.set_pixel_row(i, 0)
    }
}

#[no_mangle]
pub extern fn main() {
    let board = unsafe{ &mut BOARD };

    spi::setup();
    pcd8544::setup();
    keypad::setup();
    serial_ram::setup();
    timer::setup();

    draw_test_pattern(board);

    for offset in 0..FONT_ROM.len() {
        board.write_ram(offset as u16, FONT_ROM.load_at(offset));
    }
    for offset in 0..PROG_ROM.len() {
        board.write_ram(0x0200 + offset as u16, PROG_ROM.load_at(offset));
    }

    let mut cpu = chirp8::cpu::CPU::new();

    loop {
        cpu.step(board);
    }
}
