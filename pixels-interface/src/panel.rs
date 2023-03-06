use egui_macroquad::egui::{
    self,
    FontId,
    RichText
};

use bevy_ecs::prelude::*;
use pixels_canvas::prelude::*;

use super::{
    ToolState,
    State,
};

use crate::add_tool_button;

pub fn register_systems(_world: &mut World, _update_schedule: &mut Schedule, draw_schedule: &mut Schedule) {
    draw_schedule.add_stage(
        "draw_settings",
        SystemStage::single_threaded()
            .with_system(draw.after("canvas_draw"))
    );
}

pub fn draw(mut state: ResMut<State>) {
    egui_macroquad::ui(|ctx| {
        let panel = egui::SidePanel::left("settings").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label("");
                ui.label(RichText::new("Pixels Client Settings").font(FontId::proportional(16.0)));
            });
            ui.label("");
            ui.horizontal(|ui| {
                ui.label("Color:");
                ui.color_edit_button_rgb(&mut state.color);
            });
            ui.label("");
            ui.horizontal(|ui| {
                ui.label("Zoom:");
                ui.add(egui::Slider::new(&mut state.camera_state.zoom, 1.0..=10.0));
            });
            ui.label("");
            ui.label(format!("Selected Tool: {}", state.selected_tool));
            ui.horizontal(|ui| {
                add_tool_button!(ctx, ui, state, state.menu_state.move_icon, ToolState::Move, {
                    state.selected_tool = ToolState::Move;
                });
                add_tool_button!(ctx, ui, state, state.menu_state.brush_icon, ToolState::Draw, {
                    state.selected_tool = ToolState::Draw;
                });
                add_tool_button!(ctx, ui, state, state.menu_state.picker_icon, ToolState::Pick, {
                    state.selected_tool = ToolState::Pick;
                });
                add_tool_button!(ctx, ui, state, state.menu_state.image_icon, ToolState::Place, {
                    state.selected_tool = ToolState::Place;
                    state.image = state
                        .image
                        .is_none()
                        .then(|| Image::new("./assets/happy-ferris.png"));
                });
            });
            ui.add_space(ui.available_height() - 20.0);
            ui.horizontal(|ui| {
                ui.label("Cooldown: ");
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
