use bevy_time::{Time, Timer, TimerMode};

use pixels_canvas::prelude::*;
use macroquad::prelude::*;
use bevy_ecs::prelude::*;

use pixels_util::Color;
use super::State;

#[derive(Resource)]
pub struct CanvasContainer {
    pub canvas: Canvas
}

impl CanvasContainer {
    pub fn new(canvas: Canvas) -> Self {
        Self { canvas }
    }
}

#[derive(Resource)]
pub struct CanvasTimer {
    pub instance: Timer
}

impl CanvasTimer {
    pub fn new(timer: Timer) -> Self {
        Self { instance: timer }
    }
}

pub fn update(time: Res<Time>, mut timer: ResMut<CanvasTimer>, mut container: ResMut<CanvasContainer>) {
    if timer.instance.tick(time.delta()).finished() {
        container.canvas.update_pixels().expect("couldn't update canvas pixels");
    }
}

pub fn draw(state: Res<State>, container: Res<CanvasContainer>) {
    for x in 0..container.canvas.width() {
        for y in 0..container.canvas.height() {
            draw_rectangle(
                x as f32, y as f32, 1.0, 1.0,
                convert_color(if state.focus {
                    container.canvas.pixel(x, y)
                } else {
                    dim_color(container.canvas.pixel(x, y))
                })
            );
        }
    }
}

pub fn convert_color(color: Color) -> macroquad::color::Color {
    macroquad::color::Color::new(
        color.r, color.g, color.b, 255.0
    )
}

fn dim_color(color: Color) -> Color {
    Color::new(color.r * 0.5, color.g * 0.5, color.b * 0.5)
}
