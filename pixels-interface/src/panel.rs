use egui_macroquad::egui::{self, RichText};

use bevy_ecs::prelude::*;
use pixels_canvas::prelude::*;

use super::{State, ToolType};

use crate::add_tool_button;

pub fn draw(world: &mut World) {
    let mut state = world.get_resource_mut::<State>().unwrap();

    egui_macroquad::ui(|ctx| {
        let panel = egui::SidePanel::left("settings").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_width(0.0);
                ui.add_space(20.0);
                ui.color_edit_button_rgb(&mut state.color);
                ui.add_space(5.0);
                add_tool_button!(ctx, ui, state, state.menu_state.move_icon, ToolType::Mover, {
                    state.selected_tool = ToolType::Mover;
                });
                ui.add_space(5.0);
                add_tool_button!(ctx, ui, state, state.menu_state.brush_icon, ToolType::Brush, {
                    state.selected_tool = ToolType::Brush;
                });
                ui.add_space(5.0);
                add_tool_button!(ctx, ui, state, state.menu_state.picker_icon, ToolType::Picker, {
                    state.selected_tool = ToolType::Picker;
                });
                ui.add_space(5.0);
                if state.image.is_some() {
                    add_tool_button!(ctx, ui, state, state.menu_state.image_icon, ToolType::Placer, {
                        state.selected_tool = ToolType::Placer;
                    });
                    ui.add_space(5.0);
                }
                ui.label(RichText::new(state.cooldown.round().to_string()).strong());
            });
        });
        state.focus = ctx.is_pointer_over_area();
        state.menu_state.area = panel.response.rect;
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
