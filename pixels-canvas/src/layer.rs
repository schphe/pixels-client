use pixels_util::color::Color;

use crate::image::{ColorMode, Image};

pub struct Layer {
    elements: Vec<LayerElement>,
    //Size may not be necessary
    size: (u64, u64),
}

impl From<Image> for Layer {
    fn from(value: Image) -> Self {
        let size = (value.width(), value.height());
        let layer_element = LayerElement {
            position: (0, 0),
            image: value,
        };
        Layer {
            elements: vec![layer_element],
            size,
        }
    }
}

impl Layer {
    pub fn new(size: (u64, u64)) -> Self {
        Layer {
            elements: Vec::new(),
            size,
        }
    }

    pub fn get_size(&self) -> (u64, u64) {
        self.size
    }

    pub fn get_mut_layer_element(&mut self, idx: usize) -> Option<&mut LayerElement> {
        self.elements.get_mut(idx)
    }

    pub fn get_layer_element(&self, idx: usize) -> Option<&LayerElement> {
        self.elements.get(idx)
    }

    pub fn add_layer_element(&mut self, position: (u64, u64), image: Image) {
        self.elements.push(LayerElement::new(position, image))
    }

    pub fn get_layer_elements(&self) -> &Vec<LayerElement> {
        &self.elements
    }
}

pub struct LayerElement {
    position: (u64, u64),
    image: Image,
}

impl LayerElement {
    pub fn new(position: (u64, u64), image: Image) -> Self {
        LayerElement { position, image }
    }

    pub fn set_raw_data(&mut self, raw_data: Vec<u8>, size: (u64, u64)) {
        self.image = Image::from_vec(raw_data, size, ColorMode::RGB);
    }

    pub fn set_data(&mut self, image: Image) {
        self.image = image;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Color> {
        self.image.get_pixel_color(x, y)
    }

    pub fn get_mut_pixel(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        self.image.get_mut_pixel_color(x, y)
    }

    pub fn get_pixels(&self) -> &Vec<Vec<Color>> {
        self.image.get_pixels()
    }

    pub fn get_position(&self) -> (u64, u64) {
        self.position
    }

    pub fn set_position(&mut self, position: (u64, u64)) {
        self.position = position;
    }
}
