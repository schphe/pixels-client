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
                if ui.add(egui::Button::new("brush")).clicked() {
                    state.selected_tool = ToolState::Draw;
                }
                if ui.add(egui::Button::new("move tool")).clicked() {
                    state.selected_tool = ToolState::Move;
                }
                if ui.add(egui::Button::new("color picker")).clicked() {
                    state.selected_tool = ToolState::ColorPick;
                }
                if ui.add(egui::Button::new("add image")).clicked() {
                    state.selected_tool = ToolState::PlaceImage;
                    state.image = state
                        .image
                        .is_none()
                        .then(|| Image::new("./assets/happy-ferris.png"));
                }
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
