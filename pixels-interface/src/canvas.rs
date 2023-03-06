use std::time::Duration;

use bevy_time::{Time, Timer, TimerMode};

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use pixels_util::color::Color;

use super::State;

#[derive(Resource)]
pub struct CanvasContainer {
    pub canvas: Canvas,
}

impl CanvasContainer {
    pub fn new(canvas: Canvas) -> Self {
        Self { canvas }
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

pub fn register_systems(
    world: &mut World,
    _update_schedule: &mut Schedule,
    draw_schedule: &mut Schedule,
) {
    draw_schedule.add_stage(
        "draw_canvas",
        SystemStage::single_threaded()
            .with_system(draw.label("canvas_draw"))
            .with_system(draw_image),
    );

    draw_schedule.add_stage(
        "update_canvas",
        SystemStage::single_threaded()
            .with_system(update_cooldown)
            .with_system(update),
    );

    world.insert_resource(CanvasTimer::new(Timer::new(
        Duration::from_secs(5),
        TimerMode::Repeating,
    )));
}

pub fn update_cooldown(mut state: ResMut<State>, container: ResMut<CanvasContainer>) {
    state.cooldown = container.canvas.get_cooldown().remaining();
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

pub fn draw(state: Res<State>, container: Res<CanvasContainer>) {
    for layer in container.canvas.get_layers() {
        for layer_element in layer.get_layer_elements() {
            let (x_pos, y_pos) = layer_element.get_position();

            for (y, y_pixels) in layer_element.get_pixels().iter().enumerate() {
                for (x, x_pixel) in y_pixels.iter().enumerate() {
                    let (x_pos, y_pos) = (x_pos + x as u64, y_pos + y as u64);

                    if x_pos >= container.canvas.width() || y_pos >= container.canvas.height() {
                        continue;
                    }

                    draw_rectangle(
                        x_pos as f32,
                        y_pos as f32,
                        1.0, 1.0,
                        convert_color(if state.cooldown == 0.0 {
                            x_pixel.clone()
                        } else {
                            dim_color(x_pixel)
                        })
                    );
                }
            }
        }
    }
}

pub fn draw_image(state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    let pos = super::mouse_world_pos(state.camera_state.instance);
    if let Some(image) = &state.image {
        container
            .canvas
            .replace_part_with_image(pos.x as u64, pos.y as u64, image);
    }
}

pub fn dim_color(color: &Color) -> Color {
    let [r, g, b, _] = color.to_rgba_array();
    Color::new_rgb(r * 0.5, g * 0.5, b * 0.5)
}

pub fn convert_color(color: Color) -> macroquad::color::Color {
    macroquad::color::Color::from(color.to_rgba_array())
}
