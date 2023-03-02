use bevy_time::{Time, Timer, TimerMode};
use std::time::Duration;

use macroquad::prelude::*;
use bevy_ecs::prelude::*;

use crate::client::Client;

#[derive(Resource)]
pub struct Canvas {
    data: Vec<u8>,
    size: (u64, u64)
}

#[derive(Resource)]
pub struct CanvasTimer(pub Timer);

impl Canvas {
    pub fn new() -> Canvas {
        return Canvas{
            data: vec![],
            size: (0, 0)
        };
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

    pub fn size_vec2(&self) -> Vec2 {
        vec2(self.width() as f32, self.height() as f32)
    }

    pub fn set_size(&mut self, size: (u64, u64)) {
        self.size = size
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn pixel(&self, x: u64, y: u64) -> Color {
        let pos = self.array_position(x, y);
        color_u8!(
            self.data[pos],
            self.data[pos+1],
            self.data[pos+2],
            255
        )
    }

    pub fn set_pixel(&mut self, x: u64, y: u64, color: Color) {
        let pos = self.array_position(x, y);
        self.data[pos] = (color.r * 255.0) as u8;
        self.data[pos+1] = (color.g * 255.0) as u8;
        self.data[pos+2] = (color.b * 255.0) as u8;
    }

    fn array_position(&self, x: u64, y: u64) -> usize {
        ((y * self.width() + x) * 3) as usize
    }
}

pub fn update(time: Res<Time>, mut timer: ResMut<CanvasTimer>, mut canvas: ResMut<Canvas>, client: Res<Client>) {
    if timer.0.tick(time.delta()).finished() {
        canvas.set_data(client.canvas_pixels().expect("couldn't get canvas pixels"))
    }
}

pub fn draw(canvas: Res<Canvas>) {
    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            draw_rectangle(
                x as f32, y as f32, 1.0, 1.0,
                canvas.pixel(x, y)
            );
        }
    }
}
