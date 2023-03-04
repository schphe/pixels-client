use client::Client;
use pixels_util::*;
use prelude::*;

mod client;
pub mod error;
pub mod prelude {
    pub use super::{
        error::{
            CanvasResult,
            CanvasError
        },
        Canvas
    };
}

pub struct Canvas {
    data: Vec<u8>,
    size: (u64, u64),
    client: Client,
}

impl Canvas {
    pub fn new(refresh: String) -> Canvas {
        let mut client = Client::new();
        client.auth(refresh).expect("couldn't get access token");

        let mut canvas = Canvas{
            data: vec![],
            size: (0, 0),
            client,
        };

        canvas.update_size().expect("couldn't update size");
        canvas.update_pixels().expect("couldn't update canvas pixels");

        return canvas;
    }

    pub fn width(&self) -> u64 {
        self.size.0
    }

    pub fn height(&self) -> u64 {
        self.size.1
    }

    pub fn size(&self) -> (u64, u64) {
        self.size
    }

    pub fn set_size(&mut self, size: (u64, u64)) {
        self.size = size
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn update_size(&mut self) -> CanvasResult {
        self.set_size(self.client.canvas_size()?);
        Ok(())
    }

    pub fn update_pixels(&mut self) -> CanvasResult {
        self.set_data(self.client.canvas_pixels()?);
        Ok(())
    }

    pub fn pixel(&self, x: u64, y: u64) -> Color {
        let pos = self.array_position(x, y);

        Color::from_rgb(
            self.data[pos],
            self.data[pos+1],
            self.data[pos+2],
        )
    }

    pub fn set_pixel(&mut self, x: u64, y: u64, color: Color) -> CanvasResult {
        // todo: cooldown
        self.client.canvas_set_pixel(x, y, color.clone())?;

        let pos = self.array_position(x, y);
        let color = color.to_rgb();

        self.data[pos] = color.0;
        self.data[pos+1] = color.1;
        self.data[pos+2] = color.2;

        Ok(())
    }

    fn array_position(&self, x: u64, y: u64) -> usize {
        ((y * self.width() + x) * 3) as usize
    }
}
