use macroquad::prelude::*;

pub fn calculate_zoom(factor: f32) -> Vec2 {
    vec2(
        1.0 / (screen_width() as f32) * 2.0 * factor,
        -1.0 / (screen_height() as f32) * 2.0 * factor,
    )
}

pub fn mouse_world_pos(camera: Camera2D) -> Vec2 {
    camera.screen_to_world(
        vec2(mouse_position().0, mouse_position().1)
    )
}

pub fn color_to_hex(color: Color) -> String {
    return format!(
        "{:02x}{:02x}{:02x}",
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8
    )
}

pub fn rgb_f32_to_color(rgb: [f32; 3]) -> Color {
    Color::new(
        rgb[0], rgb[1], rgb[2], 1.0,
    )
}

pub fn color_to_rgb_u8(color: Color) -> [u8; 3] {
    [
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8
    ]
}
