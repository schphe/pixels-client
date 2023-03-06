use std::fmt::{
    Display,
    Formatter
};
use egui_macroquad::egui::Rect;

use macroquad::prelude::*;
use bevy_ecs::prelude::*;

use pixels_canvas::image::Image;

#[derive(Resource)]
pub struct State {
    pub image: Option<Image>,
    pub cooldown: f32,
    pub zoom: f32,
    pub focus: bool,
    pub color: [f32; 3],
    pub camera: Camera2D,
    pub position: Vec2,
    pub move_origin: Vec2,
    pub selected_tool: ToolState,
    pub menu_area: Rect,
}

impl Default for State {
    fn default() -> Self {
        State {
            image: None,
            cooldown: 0.0,
            zoom: 3.0,
            focus: false,
            color: [1.0; 3],
            camera: Camera2D::default(),
            position: vec2(0.0, 0.0),
            move_origin: vec2(0.0, 0.0),
            selected_tool: ToolState::Move,
            menu_area: Rect::NOTHING,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum ToolState {
    Draw,
    Move,
    ColorPick,
    PlaceImage,
}

impl Display for ToolState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draw => {
                write!(f, "Brush")
            }
            Self::Move => {
                write!(f, "Move Tool")
            }
            Self::ColorPick => {
                write!(f, "Color Picker")
            }
            Self::PlaceImage => {
                write!(f, "Image Placer")
            }
        }
    }
}
