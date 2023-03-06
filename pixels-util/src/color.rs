use crate::{
    from, into,
    normalize_color,
    denormalize_color
};

#[derive(Copy, Clone)]
pub enum ColorMode {
    RGBA,
    RGB
}

impl ColorMode {
    pub fn size(&self) -> usize {
        match self {
            ColorMode::RGBA => 4,
            ColorMode::RGB => 3,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r,
            g,
            b,
            a,
        }
    }

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: normalize_color!(r),
            g: normalize_color!(g),
            b: normalize_color!(b),
            a: normalize_color!(a),
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Self::from_rgba(r, g, b, 255)
    }

    pub fn from_slice(buffer: &[u8], mode: ColorMode) -> Color {
        match mode {
            ColorMode::RGBA => Color::from_rgba(
                buffer[0],
                buffer[1],
                buffer[2],
                buffer[3],
            ),
            ColorMode::RGB => Color::from_rgb(
                buffer[0],
                buffer[1],
                buffer[2],
            ),
        }
    }

    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (
            denormalize_color!(self.r),
            denormalize_color!(self.g),
            denormalize_color!(self.b),
            denormalize_color!(self.a),
        )
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            denormalize_color!(self.r),
            denormalize_color!(self.g),
            denormalize_color!(self.b),
        )
    }

    pub fn to_hex(&self, mode: ColorMode) -> String {
        let color = self.to_rgba();
        match mode {
            ColorMode::RGBA => format!(
                "{:02x}{:02x}{:02x}{:02x}",
                color.0, color.1, color.2, color.3
            ),
            ColorMode::RGB => format!(
                "{:02x}{:02x}{:02x}",
                color.0, color.1, color.2
            ),
        }
    }

    pub fn merge_alpha(&self, other: Self, alpha: f32) -> Color {
        let inv_alpha = 1.0 - other.a * alpha;

        Self {
            r: self.r * inv_alpha + other.r * alpha,
            g: self.g * inv_alpha + other.g * alpha,
            b: self.b * inv_alpha + other.b * alpha,
            a: self.a * inv_alpha + other.a * alpha,
        }
    }
}

from!([u8; 3], Color, |value: [u8; 3]| {
    Color::from_rgb(value[0], value[1], value[2])
});

from!([u8; 4], Color, |value: [u8; 4]| {
    Color::from_rgba(value[0], value[1], value[2], value[3])
});

from!([f32; 3], Color, |value: [f32; 3]| {
    Color::new(value[0], value[1], value[2], 1.0)
});

from!([f32; 4], Color, |value: [f32; 4]| {
    Color::new(value[0], value[1], value[2], value[3])
});

into!(Color, [u8; 3], |value: Color| {
    let color = value.to_rgb();
    [color.0, color.1, color.2]
});

into!(Color, [u8; 4], |value: Color| {
    let color = value.to_rgba();
    [color.0, color.1, color.2, color.3]
});

into!(Color, [f32; 3], |value: Color| {
    [value.r, value.g, value.b]
});

into!(Color, [f32; 4], |value: Color| {
    [value.r, value.g, value.b, value.a]
});

#[macro_export]
macro_rules! normalize_color {
    ($color:expr) => {
        $color as f32 / 255.0
    };
}

#[macro_export]
macro_rules! denormalize_color {
    ($color:expr) => {
        ($color * 255.0) as u8
    };
}
