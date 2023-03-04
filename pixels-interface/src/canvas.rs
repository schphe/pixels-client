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
            .update_pixels()
            .expect("couldn't update canvas pixels");
    }
}

pub fn draw(container: Res<CanvasContainer>) {
    for y in 0..container.canvas.height() {
        for x in 0..container.canvas.width() {
            draw_rectangle(
                x as f32,
                y as f32,
                1.0,
                1.0,
                convert_color(
                    container
                        .canvas
                        .pixel(x as usize, y as usize)
                        .expect(format!("Unexpected index: (x: {}, y: {})", x, y).as_str()),
                ),
            );
        }
    }
}

pub fn convert_color(color: Color) -> macroquad::color::Color {
    macroquad::color::Color::new(color.r, color.g, color.b, 255.0)
}
