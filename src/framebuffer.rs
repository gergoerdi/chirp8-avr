use pcd8544;
use chirp8::prelude::*;

const FB_X_START: u8 = (pcd8544::SCREEN_WIDTH - SCREEN_WIDTH) / 2;
const FB_X_END: u8 = FB_X_START + SCREEN_WIDTH;
const FB_Y_START: u8 = (pcd8544::SCREEN_HEIGHT - SCREEN_HEIGHT) / 2;
const FB_Y_END: u8 = FB_Y_START + SCREEN_HEIGHT;

pub struct FBIter<'a> {
    data: &'a [u64; SCREEN_HEIGHT as usize],
    x: u8,
    y: u8,
    mask: u64
}

impl<'a> FBIter<'a> {
    pub fn new(data: &'a [u64; SCREEN_HEIGHT as usize]) -> FBIter<'a> {
        FBIter {
            data: data,
            x: 0,
            y: 0,
            mask: 1 << 63
        }
    }

    fn stripe(&self) -> u8 {
        if self.x < FB_X_START || self.x >= FB_X_END { return 0x00 };
        let x = self.x - FB_X_START;
        if self.y < FB_Y_START || self.y >= FB_Y_END { return 0x00 };
        let y = self.y - FB_Y_START;

        let mut stripe = 0x00;

        for i in (y..y+8).rev() {
            let row: u64 = self.data[i as usize];
            let pixel = if row & self.mask == 0 { 0 } else { 1 };

            stripe = (stripe << 1) | pixel;
        }

        stripe
    }
}

impl<'a> Iterator for FBIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= pcd8544::SCREEN_WIDTH {
            return None
        }

        let result = self.stripe();

        self.y += 8;
        if self.y >= pcd8544::SCREEN_HEIGHT {
            self.y = 0;
            if self.x >= FB_X_START { self.mask >>= 1 }
            self.x += 1;
        }

        Some(result)
    }
}
