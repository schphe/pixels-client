use std::path::Path;

use pixels_util::color::Color;

pub enum ColorMode {
    RGB,
    RGBA,
}

impl From<ColorMode> for usize {
    fn from(value: ColorMode) -> Self {
        match value {
            ColorMode::RGB => 3,
            ColorMode::RGBA => 4,
        }
    }
}

#[derive(Clone)]
pub struct Image {
    data: Vec<Vec<Color>>,
    size: (u64, u64),
}

impl Image {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let image = image::io::Reader::open(path).unwrap().decode().unwrap();
        let size = (image.width() as u64, image.height() as u64);
        let color_mode = image.as_rgb8().map_or(ColorMode::RGBA, |_| ColorMode::RGB);
        Image::from_vec(image.into_bytes(), size, color_mode)
    }

    pub fn from_vec(raw_data: Vec<u8>, size: (u64, u64), color_mode: ColorMode) -> Self {
        let mut data = Vec::with_capacity(size.1 as usize);
        let mut row = Vec::with_capacity(size.0 as usize);

        for chunk in raw_data.chunks_exact(color_mode.into()) {
            row.push(
                Color::try_from(chunk).expect("Unexpected image format, expected: RGB and RGBA"),
            );
            if row.len() == (size.0 as usize) {
                data.push(row.clone());
                row.clear();
            }
        }

        Image { data, size }
    }

    pub fn width(&self) -> u64 {
        self.size.0
    }

    pub fn height(&self) -> u64 {
        self.size.1
    }

    pub fn get_pixel_color(&self, x: usize, y: usize) -> Option<&Color> {
        self.data.get(y).and_then(|x_axis| x_axis.get(x))
    }

    pub fn get_mut_pixel_color(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.data.get_mut(y).and_then(|x_axis| x_axis.get_mut(x))
    }

    pub fn set_pixel_color(&mut self, x: usize, y: usize, color: Color) {
        if let Some(pixel) = self.data.get_mut(y).and_then(|x_axis| x_axis.get_mut(x)) {
            *pixel = color;
        }
    }

    pub fn get_pixels(&self) -> &Vec<Vec<Color>> {
        &self.data
    }

    pub fn replace_part_with_image(&mut self, part_x: usize, part_y: usize, image: &Image) {
        let (width, height) = (self.size.0 as usize, self.size.1 as usize);

        let (x1, y1) = (part_x.min(width), part_y.min(height));

        let (x2, y2) = (
            (x1 + image.size.0 as usize).min(width),
            (y1 + image.size.1 as usize).min(height),
        );

        for (y, row) in self.data.get_mut(y1..y2).unwrap().iter_mut().enumerate() {
            for (x, pixel) in row.get_mut(x1..x2).unwrap().iter_mut().enumerate() {
                *pixel = *image.get_pixel_color(x, y).unwrap();
            }
        }
    }
}
