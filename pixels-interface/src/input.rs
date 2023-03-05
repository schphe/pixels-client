use bevy_ecs::schedule::ShouldRun;
use egui_macroquad::egui::Pos2;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use super::{CanvasContainer, State, ToolState};
use pixels_canvas::image::Image;
use pixels_util::color::Color;
use rfd::FileDialog;

pub fn register_systems(
    _world: &mut World,
    update_schedule: &mut Schedule,
    _draw_schedule: &mut Schedule,
) {
    update_schedule.add_stage(
        "update_input",
        SystemStage::single_threaded()
            .with_system(update_zoom)
            .with_run_criteria(run_if_not_focus)
            .with_system(update_mouse)
            .with_run_criteria(run_if_not_focus)
            .with_system(update_tool_move)
            .with_run_criteria(run_if_not_focus)
            .with_system(update_tool_draw)
            .with_run_criteria(run_if_not_focus)
            .with_system(update_tool_pick)
            .with_run_criteria(run_if_not_focus)
            .with_system(update_tool_place)
            .with_run_criteria(run_if_not_focus),
    );
}

fn run_if_not_focus(state: Res<State>) -> ShouldRun {
    let pos = Pos2::new(mouse_position().0, mouse_position().1);

    if state.menu_area.contains(pos) || state.focus {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

pub fn update_zoom(mut state: ResMut<State>) {
    state.camera_state.zoom = (state.camera_state.zoom + mouse_wheel().1 / 120.0).clamp(1.0, 10.0);
}

pub fn update_mouse(mut state: ResMut<State>) {
    let pos = super::mouse_world_pos(state.camera_state.instance);

    if is_mouse_button_pressed(MouseButton::Left) {
        state.camera_state.move_origin = pos;
    } else if is_mouse_button_down(MouseButton::Left) && state.selected_tool == ToolState::Move {
        let origin = state.camera_state.move_origin;
        state.camera_state.position += origin - pos;
    }
}

pub fn update_tool_move(mut state: ResMut<State>) {
    if is_key_down(KeyCode::M) {
        state.selected_tool = ToolState::Move;
    }
}

pub fn update_tool_draw(mut state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    if is_key_down(KeyCode::B) {
        state.selected_tool = ToolState::Draw;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    if let ToolState::Draw = state.selected_tool {
        let pos = super::mouse_world_pos(state.camera_state.instance);

        if let Err(e) = container.canvas.set_main_pixel(
            pos.x as usize,
            pos.y as usize,
            Color::from(state.color),
        ) {
            match e {
                CanvasError::ClientError => {
                    panic!("couldn't set pixel");
                }
                CanvasError::Cooldown(cooldown) => {
                    println!("please wait cooldown to end: {}", cooldown.remaining());
                }
            }
        }
    }
}

pub fn update_tool_pick(mut state: ResMut<State>, container: ResMut<CanvasContainer>) {
    if is_key_down(KeyCode::I) {
        state.selected_tool = ToolState::Pick;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    if let ToolState::Pick = state.selected_tool {
        let pos = super::mouse_world_pos(state.camera_state.instance);

        state.color = (*container
            .canvas
            .get_main_pixel(pos.x as usize, pos.y as usize)
            .unwrap_or(&Color::default()))
        .try_into()
        .expect("Expected RGB found RGBA")
    }
}

pub fn update_tool_place(mut state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    if !is_key_down(KeyCode::P) {
        return;
    }

    let pos = super::mouse_world_pos(state.camera_state.instance);

    state.camera_state.move_origin = pos;
    let path = FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .set_directory("~")
        .pick_file();

    if let Some(path) = path {
        let image = Image::new(path);
        container.canvas.get_mut_layers()[1].add_layer_element((pos.x as u64, pos.y as u64), image);
    }
}
