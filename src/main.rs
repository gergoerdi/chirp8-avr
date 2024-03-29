#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

#![no_std]
#![no_main]

#![allow(unused_variables)]
#![allow(dead_code)]

extern crate chirp8_engine as chirp8;
extern crate avr_std_stub;
extern crate avr_config;
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

use chirp8::cpu::*;

struct Board {
    fb_dirty: bool,
    fb_rows: [u64; SCREEN_HEIGHT as usize],
}

impl Board {
    pub const fn new() -> Board {
        Board {
            fb_dirty: false,
            fb_rows: [0; SCREEN_HEIGHT as usize],
        }
    }
}
static mut BOARD: Board = Board::new();
static mut CPU: CPU = CPU::new(QUIRKS);

impl Peripherals for Board {
    fn get_pixel_row(&self, y: u8) -> u64 {
        self.fb_rows[y as usize]
    }

    fn set_pixel_row(&mut self, y: u8, row: u64) {
        self.fb_rows[y as usize] = row;
        self.fb_dirty = true;
    }

    fn get_keys(&self) -> u16 {
        let row0 = keypad::scan_key_row(0);
        let row1 = keypad::scan_key_row(1);
        let row2 = keypad::scan_key_row(2);
        let row3 = keypad::scan_key_row(3);

        (if (row3 & 1 << 1) == 0 {1} else {0}) << 0x0 |
        (if (row0 & 1 << 0) == 0 {1} else {0}) << 0x1 |
        (if (row0 & 1 << 1) == 0 {1} else {0}) << 0x2 |
        (if (row0 & 1 << 2) == 0 {1} else {0}) << 0x3 |
        (if (row1 & 1 << 0) == 0 {1} else {0}) << 0x4 |
        (if (row1 & 1 << 1) == 0 {1} else {0}) << 0x5 |
        (if (row1 & 1 << 2) == 0 {1} else {0}) << 0x6 |
        (if (row2 & 1 << 0) == 0 {1} else {0}) << 0x7 |
        (if (row2 & 1 << 1) == 0 {1} else {0}) << 0x8 |
        (if (row2 & 1 << 2) == 0 {1} else {0}) << 0x9 |

        (if (row3 & 1 << 0) == 0 {1} else {0}) << 0xa |
        (if (row3 & 1 << 2) == 0 {1} else {0}) << 0xb |
        (if (row0 & 1 << 3) == 0 {1} else {0}) << 0xc |
        (if (row1 & 1 << 3) == 0 {1} else {0}) << 0xd |
        (if (row2 & 1 << 3) == 0 {1} else {0}) << 0xe |
        (if (row3 & 1 << 3) == 0 {1} else {0}) << 0xf
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
}

fn redraw(board: &mut Board) {
    if board.fb_dirty {
        pcd8544::send(FBIter::new(&board.fb_rows));
        board.fb_dirty = false;
    }
}

pub fn tick() {
    let board = unsafe{ &mut BOARD };
    redraw(board);

    let cpu = unsafe{ &mut CPU };
    cpu.tick_frame();
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

    let cpu = unsafe{ &mut CPU };
    loop {
        cpu.step(board);
    }
}
