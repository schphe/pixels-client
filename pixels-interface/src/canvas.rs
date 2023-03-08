use std::time::Duration;

use bevy_time::{Time, Timer, TimerMode};

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use pixels_util::color::Color;
use crate::state::ToolType;

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
    canvas: Canvas,
    world: &mut World,
    update_schedule: &mut Schedule,
    draw_schedule: &mut Schedule,
) {
    draw_schedule.add_systems((
        draw,
        draw_image
    ));

    update_schedule.add_systems((
        update_cooldown,
        update.run_if(not(is_cooldown)),
    ));

    world.insert_resource(CanvasContainer::new(canvas));
    world.insert_resource(CanvasTimer::new(Timer::new(
        Duration::from_secs(5),
        TimerMode::Repeating,
    )));
}

fn is_cooldown(state: Res<State>) -> bool {
    state.cooldown != 0.0
}

pub fn update_cooldown(mut state: ResMut<State>, container: ResMut<CanvasContainer>) {
    state.cooldown = container.canvas.get_cooldown();
}

pub fn update(
    time: Res<Time>,
    mut timer: ResMut<CanvasTimer>,
    mut container: ResMut<CanvasContainer>,
) {
    if timer.instance.tick(time.delta()).finished() {
        container
            .canvas
            .update_main_layer()
            .expect("couldn't update canvas pixels");
    }
}

pub fn draw(state: Res<State>, container: Res<CanvasContainer>) {
    for ((x, y), color) in container.canvas.get_layers_merged().iter() {
        draw_rectangle(
            x as f32, y as f32, 1.0, 1.0,
            convert_color(if state.cooldown == 0.0 {
                color
            } else {
                dim_color(color)
            })
        );
    }
}

pub fn draw_image(mut state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    let pos = super::mouse_world_pos(state.camera_state.instance);

    container.canvas.get_image_layer_mut().clean();

    if state.selected_tool != ToolType::Placer {
        return;
    }

    if state.image.is_some() {
        state.image.as_mut().unwrap().set_position(pos.x as u32, pos.y as u32);
        container.canvas.get_image_layer_mut().draw(state.image.clone().unwrap());
    }
}

pub fn dim_color(color: Color) -> Color {
    Color::new(color.r * 0.5, color.g * 0.5, color.b * 0.5, color.a)
}

pub fn convert_color(color: Color) -> macroquad::color::Color {
    let array: [f32; 4] = color.into();
    macroquad::color::Color::from(array)
}
