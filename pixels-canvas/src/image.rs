use std::path::Path;

use pixels_util::color::Color;

pub struct Image {
    data: Vec<Vec<Color>>,
    size: (u64, u64)
}

impl Image {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let image = image::io::Reader::open(path).unwrap().decode().unwrap();
        let size = (image.width() as u64, image.height() as u64);

        let mut y_axis = Vec::with_capacity(size.1 as usize);
        let mut x_axis = Vec::with_capacity(size.0 as usize);

        for (i, pixel) in image.as_rgb8().unwrap().pixels().into_iter().enumerate() {
            if i % size.0 as usize == 0 && i >= size.0 as usize {
                y_axis.push(x_axis.clone());
                x_axis.clear()
            }
            let [r, g, b] = pixel.0;
            x_axis.push(Color::from_rgb(r, g, b));
        }
        y_axis.push(x_axis);
        Image {
            data: y_axis,
            size
        }
    }

    pub fn from_vec(data: Vec<u8>, size: (u64, u64)) -> Self {
        let mut y_axis = Vec::with_capacity(size.1 as usize);
        let mut x_axis = Vec::with_capacity(size.0 as usize);

        for (i, raw_color) in data.chunks(3).enumerate(){
            if let [r, g, b] = raw_color{
                if i % size.0 as usize == 0 && i >= size.0 as usize{
                    y_axis.push(x_axis.clone());
                    x_axis.clear()
                }
                x_axis.push(Color::from_rgb(*r, *g, *b));
            }
            
        }
        y_axis.push(x_axis.clone());
        Image { data: y_axis, size }
    }

    pub fn width(&self) -> u64 {
        self.size.0
    }

    pub fn height(&self) -> u64 {
        self.size.1
    }

    pub fn get_pixel_color(&self, x: usize, y: usize) -> Option<&Color> {
        
        self.data.get(y as usize).and_then(|x_axis| x_axis.get(x as usize))
    }

    pub fn set_pixel_color(&mut self, x: usize, y: usize, color: Color) {
        if let Some(pixel) = self.data.get_mut(y as usize).and_then(|x_axis| x_axis.get_mut(x)) {
            *pixel = color;
        }
    }
}
