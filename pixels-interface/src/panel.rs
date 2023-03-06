use egui_macroquad::egui::{self, Response, RichText, TextureId, Widget, Vec2, Ui, Context};

use bevy_ecs::prelude::*;

use super::{
    State,
    ToolType
};

use crate::{
    panel,
    tool_button,
    tool_button_if
};

struct ToolButton {
    selected: bool,
    tool: ToolType,
    icon: TextureId,
    size: Vec2,
}

pub fn draw(world: &mut World) {
    panel!(world, |ctx: &Context, ui: &mut Ui, state: &mut State| {
        ui.add_space(20.0);
        ui.color_edit_button_rgb(&mut state.color);

        tool_button!(ctx, ui, state, ToolType::Mover, state.menu_state.move_icon, {
            state.selected_tool = ToolType::Mover;
        });

        tool_button!(ctx, ui, state, ToolType::Brush, state.menu_state.brush_icon, {
            state.selected_tool = ToolType::Brush;
        });

        tool_button!(ctx, ui, state, ToolType::Picker, state.menu_state.picker_icon, {
            state.selected_tool = ToolType::Picker;
        });

        tool_button_if!(ctx, ui, state, ToolType::Placer, state.menu_state.image_icon, {
            state.selected_tool = ToolType::Placer;
        }, state.image.is_some());

        ui.add_space(5.0);
        ui.label(RichText::new(
            state.cooldown.round().to_string()
        ).strong());
    });
}

impl ToolButton {
    fn new(selected: bool, tool: ToolType, icon: TextureId, size: Vec2) -> Self {
        Self {
            selected,
            tool,
            icon,
            size,
        }
    }
}

impl Widget for ToolButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add_space(5.0);
        let button = ui.add(egui::ImageButton::new(
            self.icon,
            self.size / 5.0,
        ));

        if self.selected {
            button.clone().highlight();
        }

        button
    }
}

#[macro_export]
macro_rules! panel {
    ($world:expr, $body:expr $(,)?) => {
        let mut res = $world.get_resource_mut::<State>().unwrap();
        let mut state = res.as_mut();

        egui_macroquad::ui(|ctx| {
            let panel = egui::SidePanel::left("settings").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.set_width(0.0);
                    $body(ctx, ui, state)
                });
            });

            state.focus = ctx.is_pointer_over_area();
            state.menu_state.area = panel.response.rect;
        });

        egui_macroquad::draw();
    };
}

#[macro_export]
macro_rules! tool_button {
    ($ctx:expr, $ui:expr, $state:expr, $tool:expr, $icon:expr, $body:block) => {{
        let button = $ui.add(ToolButton::new(
            $state.selected_tool == $tool, $tool, $icon.texture_id($ctx), $icon.size_vec2()
        ));

        if button.clicked() {
            $body
        }
    }};
}

#[macro_export]
macro_rules! tool_button_if {
    ($ctx:expr, $ui:expr, $state:expr, $tool:expr, $icon:expr, $body:block, $condition:expr) => {{
        if $condition {
            tool_button!($ctx, $ui, $state, $tool, $icon, $body);
        }
    }};
}
