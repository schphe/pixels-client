use paste::item;

use pixels_util::prelude::*;
use prelude::*;

mod client;
mod layer;
mod error;
mod elem;

pub mod prelude {
    pub use crate::{
        Canvas,
        layer::{
            Layer,

        },
        elem::Element,
        client::Client,
        error::CanvasError,
    };
}

pub struct Canvas {
    client: Client,
    layers: Vec<Layer>,
    cooldown: Cooldown,
    size: (u32, u32),
}

impl Canvas {
    pub fn new(refresh: String) -> Result<Self, CanvasError> {
        let mut client = Client::new();

        client.auth(refresh)?;

        let mut canvas = Self {
            size: client.canvas_size()?,
            cooldown: Cooldown::new(),
            layers: vec![],
            client,
        };

        canvas.add_layer(Layer::new(canvas.size(), 1.0));
        canvas.add_layer(Layer::new(canvas.size(), 0.5));

        canvas.update_main_layer()?;

        Ok(canvas)
    }

    pub fn width(&self) -> u32 {
        self.size.0
    }

    pub fn height(&self) -> u32 {
        self.size.1
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width(), self.height())
    }

    pub fn get_cooldown(&self) -> f32 {
        self.cooldown.remaining()
    }

    fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer)
    }

    pub fn get_layer(&self, id: usize) -> Option<&Layer> {
        self.layers.get(id)
    }

    pub fn get_layer_mut(&mut self, id: usize) -> Option<&mut Layer> {
        self.layers.get_mut(id)
    }

    layer_accessors!(main, 0);
    layer_accessors!(image, 1);

    pub fn get_layers_merged(&self) -> Layer {
        self.layers.iter().fold(
            Layer::new(self.size, 1.0),
            |l, o| l.overlay(o),
        )
    }

    pub fn update_main_layer(&mut self) -> Result<(), CanvasError> {
        let pixels = self.client.canvas_pixels()?;
        let size = self.size;

        self.get_main_layer_mut().set_pixels(
            Pixels::from_buffer(
                size,
                pixels,
                ColorMode::RGB,
            )
        );
        Ok(())
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), CanvasError> {
        if !self.cooldown.is_ended() {
            return Err(CanvasError::Cooldown(self.get_cooldown()))
        }

        self.get_main_layer_mut().set_pixel(x, y, color);
        let (remain, delay) = self.client.canvas_set_pixel(x, y, color)?;

        if remain == 0 {
            self.cooldown.set(delay)
        }

        Ok(())
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        self.get_main_layer().get_pixel(x, y)
    }
}

#[macro_export]
macro_rules! layer_accessors {
    ($name:ident, $id:expr) => {
        item! {
            pub fn [<get_ $name _layer>](&self) -> &Layer {
                self.layers.get($id).unwrap()
            }

            pub fn [<get_ $name _layer_mut>](&mut self) -> &mut Layer {
                self.layers.get_mut($id).unwrap()
            }
        }
    };
}
