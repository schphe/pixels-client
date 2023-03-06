use egui_macroquad::egui::Rect;
use egui_extras::RetainedImage;

use bevy_ecs::prelude::*;
use macroquad::prelude::*;

use pixels_canvas::image::Image;

#[derive(Resource)]
pub struct State {
    pub focus: bool,
    pub color: [f32; 3],
    pub cooldown: f32,
    pub image: Option<Image>,
    pub selected_tool: ToolType,
    pub camera_state: CameraState,
    pub menu_state: MenuState,
}

pub struct CameraState {
    pub zoom: f32,
    pub instance: Camera2D,
    pub position: Vec2,
    pub move_origin: Vec2,
}

pub struct MenuState {
    pub area: Rect,
    pub move_icon: RetainedImage,
    pub brush_icon: RetainedImage,
    pub image_icon: RetainedImage,
    pub picker_icon: RetainedImage,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ToolType {
    Mover,
    Brush,
    Picker,
    Placer,
}

impl State {
    pub(crate) fn new(image: Option<Image>) -> Self {
        State {
            image,
            focus: false,
            color: [1.0; 3],
            cooldown: 0.0,
            selected_tool: ToolType::Mover,
            camera_state: CameraState::default(),
            menu_state: MenuState::default(),
        }
    }
}

impl Default for CameraState {
    fn default() -> Self {
        CameraState {
            zoom: 3.0,
            instance: Camera2D::default(),
            position: vec2(0.0, 0.0),
            move_origin: vec2(0.0, 0.0),
        }
    }
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState {
            area: Rect::NOTHING,
            move_icon: RetainedImage::from_image_bytes(
                "move_icon",
                include_bytes!("../../assets/tool-move.png"),
            ).unwrap(),
            brush_icon: RetainedImage::from_image_bytes(
                "brush_icon",
                include_bytes!("../../assets/tool-brush.png"),
            ).unwrap(),
            image_icon: RetainedImage::from_image_bytes(
                "image_icon",
                include_bytes!("../../assets/tool-image.png"),
            ).unwrap(),
            picker_icon: RetainedImage::from_image_bytes(
                "picker_icon",
                include_bytes!("../../assets/tool-picker.png"),
            ).unwrap(),
        }
    }
}
