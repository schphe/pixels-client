use pixels_util::prelude::*;

use crate::prelude::Element;

pub struct Layer {
    pixels: Pixels,
    opacity: f32
}

impl Layer {
    pub fn new(size: (u32, u32), opacity: f32) -> Layer {
        Self {
            pixels: Pixels::new(size),
            opacity,
        }
    }

    pub fn from_vec(size: (u32, u32), buffer: Vec<u8>, opacity: f32) -> Self {
        Self::from_pixels(
            Pixels::from_buffer(
                size, buffer, ColorMode::RGB
            ),
            opacity
        )
    }

    pub fn from_pixels(pixels: Pixels, opacity: f32) -> Self {
        Self {
            pixels,
            opacity,
        }
    }

    pub fn get_opacity(&self) -> u8 {
        (self.opacity * 255.0) as u8
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        self.pixels.get(x, y)
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels.set(x, y, color)
    }

    pub fn set_pixels(&mut self, pixels: Pixels) {
        self.pixels = pixels
    }

    pub fn draw(&mut self, element: Element) {
        for ((x, y), color) in element.iter() {
            let pos = element.get_position();
            self.pixels.set(x + pos.0, y + pos.1, color);
        }
    }

    pub fn overlay(&self, other: &Layer) -> Layer {
        Self::from_pixels(
            self.pixels.overlay(
                &other.pixels, other.opacity
            ),
            self.opacity
        )
    }

    pub fn clean(&mut self) {
        self.pixels = Pixels::new(self.pixels.size())
    }

    pub fn iter(&self) -> PixelsIterator {
        self.pixels.iter()
    }
}
