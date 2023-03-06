use egui_macroquad::egui::{self, FontId, RichText};

use bevy_ecs::prelude::*;
use pixels_canvas::prelude::*;

use super::{State, ToolState};

use crate::add_tool_button;

pub fn register_systems(
    _world: &mut World,
    _update_schedule: &mut Schedule,
    draw_schedule: &mut Schedule,
) {
    draw_schedule.add_stage(
        "draw_settings",
        SystemStage::single_threaded().with_system(draw.after("canvas_draw")),
    );
}

pub fn draw(mut state: ResMut<State>) {
    egui_macroquad::ui(|ctx| {
        let panel = egui::SidePanel::left("settings").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_width(0.0);
                ui.add_space(20.0);
                ui.color_edit_button_rgb(&mut state.color);
                ui.add_space(5.0);
                add_tool_button!(ctx, ui, state, state.menu_state.move_icon, ToolState::Move, {
                    state.selected_tool = ToolState::Move;
                });
                ui.add_space(5.0);
                add_tool_button!(ctx, ui, state, state.menu_state.brush_icon, ToolState::Draw, {
                    state.selected_tool = ToolState::Draw;
                });
                ui.add_space(5.0);
                add_tool_button!(ctx, ui, state, state.menu_state.picker_icon, ToolState::Pick, {
                    state.selected_tool = ToolState::Pick;
                });
                ui.add_space(5.0);
                add_tool_button!(ctx, ui, state, state.menu_state.image_icon, ToolState::Place, {
                    state.selected_tool = ToolState::Place;
                    state.image = state
                        .image
                        .is_none()
                        .then(|| Image::new("./assets/happy-ferris.png"));
                });
                ui.add_space(5.0);
                ui.label(RichText::new(state.cooldown.round().to_string()).strong());
            });
        });
        state.focus = ctx.is_pointer_over_area();
        state.menu_area = panel.response.rect;
    });

    egui_macroquad::draw();
}

#[macro_export]
macro_rules! add_tool_button {
    ($ctx:expr, $ui:expr, $state:expr, $icon:expr, $tool:expr, $body:block) => {{
        let button = $ui.add(egui::ImageButton::new(
            $icon.texture_id($ctx),
            $icon.size_vec2() / 5.0,
        ));

        if $state.selected_tool == $tool {
            button.clone().highlight();
        }

        if button.clicked() {
            $body
        }
    }};
}
