use bevy_time::{Time, Timer};

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use pixels_util::{color::Color, cooldown::Cooldown};

#[derive(Resource)]
pub struct CanvasContainer {
    pub canvas: Canvas,
}

impl CanvasContainer {
    pub fn new(canvas: Canvas) -> Self {
        Self { canvas }
    }

    pub fn get_cooldown(&self) -> &Cooldown {
        self.canvas.get_cooldown()
    }
}

#[derive(Resource)]
pub struct CanvasTimer {
    pub instance: Timer,
}

impl CanvasTimer {
    pub fn new(timer: Timer) -> Self {
        Self { instance: timer }
    }
}

pub fn update(
    time: Res<Time>,
    mut timer: ResMut<CanvasTimer>,
    mut container: ResMut<CanvasContainer>,
) {
    if timer.instance.tick(time.delta()).finished() {
        container
            .canvas
            .update_main_pixels()
            .expect("couldn't update canvas pixels");
    }
}

pub fn draw(container: Res<CanvasContainer>) {
    for layer in container.canvas.get_layers() {
        for layer_element in layer.get_layer_elements() {
            let (x_position, y_position) = layer_element.get_position();
            for (y, y_pixels) in layer_element.get_pixels().iter().enumerate() {
                for (x, x_pixel) in y_pixels.iter().enumerate() {
                    let (x_position, y_position) = (x_position + x as u64, y_position + y as u64);
                    if x_position < container.canvas.width()
                        && y_position < container.canvas.height()
                    {
                        draw_rectangle(
                            x_position as f32,
                            y_position as f32,
                            1.0,
                            1.0,
                            convert_color(x_pixel),
                        );
                    }
                }
            }
        }
    }
}

pub fn convert_color(color: &Color) -> macroquad::color::Color {
    macroquad::color::Color::from(color.to_rgba_array())
}
