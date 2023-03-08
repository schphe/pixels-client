use std::path::Path;
use image::io::Reader;
use image::GenericImageView;

use super::prelude::*;

#[derive(Clone)]
pub struct Pixels {
    size: (u32, u32),
    data: Vec<Vec<Color>>,
}

impl Pixels {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            data: vec![
                vec![
                    Color::default();
                    size.0 as usize
                ]; size.1 as usize
            ],
        }
    }

    pub fn from_buffer(size: (u32, u32), buffer: Vec<u8>, mode: ColorMode) -> Self {
        let mut instance = Self::new(size);

        for (i, pixel) in instance.data.iter_mut().flatten().enumerate() {
            *pixel = Color::from_slice(&buffer[i*mode.size()..], mode);
        }

        instance
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let image = Reader::open(path)
            .unwrap().decode().unwrap();

        Self::from_buffer(
            image.dimensions(),
            image.clone().into_bytes(),
            ColorMode::from(image.color()).expect("unsupported format")
        )
    }

    pub fn width(&self) -> u32 {
        self.size.0
    }

    pub fn height(&self) -> u32 {
        self.size.1
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Color> {
        Some(*self.data
            .get(y as usize)?
            .get(x as usize)?)
    }

    pub fn set(&mut self, x: u32, y: u32, color: Color) {
        if let Some(item) = self.data
            .get_mut(y as usize)
            .and_then(|list| {
                list.get_mut(x as usize)
            }) {
            *item = color;
        }
    }

    pub fn overlay(&self, other: &Pixels, alpha: f32) -> Pixels {
        let mut result = Pixels::new(self.size);

        for ((x, y), src) in self.iter() {
            let dst = other.get(x, y).unwrap();
            result.set(x, y, src.merge_alpha(dst, alpha));
        }

        result
    }

    pub fn iter(&self) -> PixelsIterator {
        PixelsIterator::new(self.clone())
    }
}

pub struct PixelsIterator {
    pixels: Pixels,
    x: u32,
    y: u32,
}

impl PixelsIterator {
    fn new(pixels: Pixels) -> Self {
        Self {
            pixels,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for PixelsIterator {
    type Item = ((u32, u32), Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.pixels.height() {
            return None;
        }

        let result = Some((
            (self.x, self.y),
            self.pixels.get(self.x, self.y).unwrap_or_default(),
        ));

        self.x += 1;
        if self.x >= self.pixels.width() {
            self.x = 0;
            self.y += 1;
        }

        result
    }
}
