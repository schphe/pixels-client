use crate::image::{ColorMode, Image};
use client::Client;
use layer::Layer;
use pixels_util::{color::Color, cooldown::Cooldown};
use prelude::*;

mod client;
pub mod error;
pub mod image;
mod layer;
pub mod prelude {
    pub use super::{
        Canvas,
        error::CanvasError,
        image::{Image, ColorMode},
        layer::{Layer, LayerElement},
    };
}

pub struct Canvas {
    layers: Vec<Layer>,
    client: Client,
    cooldown: Cooldown,
}

impl Canvas {
    pub fn new(refresh: String) -> Canvas {
        let mut client = Client::new();
        client.auth(refresh).expect("couldn't get access token");

        let data = client.canvas_pixels().expect("couldn't get canvas pixels");
        let size = client.canvas_size().expect("couldn't get canvas size");

        let canvas_layer = Layer::from(Image::from_vec(data, size, ColorMode::RGB));
        let image_layer = Layer::new(size);

        let mut canvas = Canvas {
            layers: vec![canvas_layer, image_layer],
            client,
            cooldown: Cooldown::default(),
        };

        canvas
            .update_main_pixels()
            .expect("couldn't update canvas pixels");

        canvas
    }

    pub fn width(&self) -> u64 {
        self.layers[0].get_size().0
    }

    pub fn height(&self) -> u64 {
        self.layers[0].get_size().1
    }

    pub fn size(&self) -> (u64, u64) {
        (self.width(), self.height())
    }

    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn get_mut_layers(&mut self) -> &mut Vec<Layer> {
        &mut self.layers
    }

    pub fn set_main_data(&mut self, raw_data: Vec<u8>, size: (u64, u64)) {
        self.layers[0]
            .get_mut_layer_element(0)
            .unwrap()
            .set_raw_data(raw_data, size);
    }

    pub fn update_main_pixels(&mut self) -> Result<(), CanvasError> {
        self.set_main_data(self.client.canvas_pixels()?, self.size());
        Ok(())
    }

    pub fn get_main_pixel(&self, x: usize, y: usize) -> Option<&Color> {
        self.layers[0].get_layer_element(0).unwrap().get_pixel(x, y)
    }

    pub fn get_cooldown(&self) -> &Cooldown {
        &self.cooldown
    }

    pub fn set_main_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), CanvasError> {
        if !self.cooldown.is_ended() {
            return Err(CanvasError::Cooldown(self.cooldown));
        }

        let (remain, cooldown) = self.client.canvas_set_pixel(x, y, color)?;
        if remain == 0 {
            self.cooldown.set(cooldown);
            self.cooldown.set(cooldown);
        }

        *self.layers[0]
            .get_mut_layer_element(0)
            .unwrap()
            .get_mut_pixel(x, y)
            .unwrap() = color;

        Ok(())
    }

    pub fn replace_part_with_image(
        &mut self,
        part_location_x: u64,
        part_location_y: u64,
        part_image: &Image,
    ) {
        if let Some(layer_element) = self.layers[1].get_mut_layer_element(0) {
            layer_element.set_data(part_image.clone());
            layer_element.set_position((part_location_x, part_location_y));
        } else {
            self.layers[1]
                .add_layer_element((part_location_x, part_location_y), part_image.clone());
        }
    }

    pub fn remove_part_image(&mut self) {
        if let Some(layer_element) = self.layers[1].get_mut_layer_element(0) {
            layer_element.set_data(Image::from_vec(Vec::new(), (0, 0), ColorMode::RGB));
        }
    }
}
