use egui_macroquad::egui::Pos2;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;
use pixels_canvas::prelude::*;

use super::{
    CanvasContainer,
    ToolType,
    State,
};
use pixels_util::color::Color;

pub fn register_systems(update_schedule: &mut Schedule) {
    update_schedule.add_systems((
        update_zoom.run_if(not(is_panel_focused)),
        update_mouse.run_if(not(is_panel_focused)),
        update_tool_move.run_if(not(is_panel_focused)),
        update_tool_draw.run_if(not(is_panel_focused)),
        update_tool_pick.run_if(not(is_panel_focused)),
        update_tool_place.run_if(not(is_panel_focused)),
    ));
}

fn is_panel_focused(state: Res<State>) -> bool {
    let pos = Pos2::new(mouse_position().0, mouse_position().1);
    state.menu_state.area.contains(pos) || state.focus
}

pub fn update_zoom(mut state: ResMut<State>) {
    state.camera_state.zoom = (state.camera_state.zoom + mouse_wheel().1 / 120.0).clamp(1.0, 10.0);
}

pub fn update_mouse(mut state: ResMut<State>) {
    let pos = super::mouse_world_pos(state.camera_state.instance);

    if is_mouse_button_pressed(MouseButton::Left) {
        state.camera_state.move_origin = pos;
    } else if is_mouse_button_down(MouseButton::Left) && state.selected_tool == ToolType::Mover {
        let origin = state.camera_state.move_origin;
        state.camera_state.position += origin - pos;
    }
}

pub fn update_tool_move(mut state: ResMut<State>) {
    if is_key_down(KeyCode::M) {
        state.selected_tool = ToolType::Mover;
    }
}

pub fn update_tool_draw(mut state: ResMut<State>, mut container: ResMut<CanvasContainer>) {
    if is_key_down(KeyCode::B) {
        state.selected_tool = ToolType::Brush;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    if let ToolType::Brush = state.selected_tool {
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
        state.selected_tool = ToolType::Picker;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    if let ToolType::Picker = state.selected_tool {
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
    if is_key_down(KeyCode::P) {
        state.selected_tool = ToolType::Placer;
    }

    if !is_mouse_button_pressed(MouseButton::Left) {
        return;
    }

    if let ToolType::Placer = state.selected_tool {
        // todo: place logic
    }
}
