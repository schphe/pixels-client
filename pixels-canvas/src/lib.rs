use crate::image::Image;
use client::Client;
use pixels_util::{color::Color, cooldown::Cooldown};
use prelude::*;

mod client;
pub mod error;
pub mod image;
pub mod prelude {
    pub use super::{
        error::{CanvasError, CanvasResult},
        Canvas,
    };
}

pub struct Canvas {
    image: Image,
    client: Client,
    cooldown: Cooldown,
}

impl Canvas {
    pub fn new(refresh: String) -> Canvas {
        let mut client = Client::new();
        client.auth(refresh).expect("couldn't get access token");

        let data = client.canvas_pixels().expect("couldn't get canvas pixels");
        let size = client.canvas_size().expect("couldn't get canvas size");
        let image = Image::from_vec(data, size);

        let mut canvas = Canvas {
            image,
            client,
            cooldown: Cooldown::default(),
        };

        canvas
            .update_pixels()
            .expect("couldn't update canvas pixels");

        canvas
    }

    pub fn width(&self) -> u64 {
        self.image.width()
    }

    pub fn height(&self) -> u64 {
        self.image.height()
    }

    pub fn size(&self) -> (u64, u64) {
        (self.width(), self.height())
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.image = Image::from_vec(data, self.size());
    }

    pub fn update_pixels(&mut self) -> CanvasResult {
        self.set_data(self.client.canvas_pixels()?);
        Ok(())
    }

    pub fn pixel(&self, x: usize, y: usize) -> Option<Color> {
        self.image.get_pixel_color(x, y).map(|color| color.clone())
    }

    pub fn get_cooldown(&self) -> &Cooldown {
        &self.cooldown
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> CanvasResult {
        if !self.cooldown.is_ended() {
            return Err(CanvasError::Cooldown(self.cooldown));
        }

        let (remain, cooldown) = self.client.canvas_set_pixel(x, y, color)?;
        if remain == 0 {
            self.cooldown.set(cooldown);
            self.cooldown.set(cooldown);
        }

        self.image.set_pixel_color(x, y, color);

        Ok(())
    }

    pub fn replace_part_with_image(&mut self, part_location_x: usize, part_location_y: usize, part_image: &Image){
        self.image.replace_part_with_image(part_location_x, part_location_y, part_image);
    }
}
