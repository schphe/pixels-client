#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Default for Color{
    fn default() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0 }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn from(colors: [f32; 3]) -> Self {
        Self::new(colors[0], colors[1], colors[2])
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        )
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }

    pub fn to_hex(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }

    pub fn as_array(&self) -> [f32; 3] {
        [
            self.r,
            self.g,
            self.b
        ]
    }
}
