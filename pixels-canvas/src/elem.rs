use std::path::Path;
use pixels_util::pixels::PixelsIterator;

use pixels_util::prelude::*;

#[derive(Clone)]
pub struct Element {
    pixels: Pixels,
    position: (u32, u32),
}

impl Element {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            pixels: Pixels::from_path(path),
            position: (0, 0),
        }
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position = (x, y)
    }

    pub fn get_position(&self) -> (u32, u32) {
        self.position
    }

    pub fn iter(&self) -> PixelsIterator {
        self.pixels.iter()
    }
}
