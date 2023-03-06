use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum Color {
    RGBA([f32; 4]),
    RGB([f32; 3]),
}

impl Default for Color {
    fn default() -> Self {
        Color::RGB([1.0; 3])
    }
}

impl From<[f32; 3]> for Color {
    fn from(value: [f32; 3]) -> Self {
        Color::RGB(value)
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Color::RGBA(value)
    }
}

impl TryFrom<&[u8]> for Color {
    type Error = ();
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if let [r, g, b, a] = value {
            Ok(Color::new_rgba(
                *r as f32 / 255.0,
                *g as f32 / 255.0,
                *b as f32 / 255.0,
                *a as f32 / 255.0,
            ))
        } else if let [r, g, b] = value {
            Ok(Color::new_rgb(
                *r as f32 / 255.0,
                *g as f32 / 255.0,
                *b as f32 / 255.0,
            ))
        } else {
            Err(())
        }
    }
}

impl TryFrom<Color> for [f32; 3] {
    type Error = ();

    fn try_from(value: Color) -> Result<Self, Self::Error> {
        match value {
            Color::RGBA(_) => Err(()),
            Color::RGB(array) => Ok(array),
        }
    }
}

impl Color {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Color::RGB([r, g, b])
    }

    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color::RGBA([r, g, b, a])
    }

    pub fn to_rgba_array(&self) -> [f32; 4] {
        match self {
            Color::RGBA(array) => *array,
            Color::RGB([r, g, b]) => [*r, *g, *b, 255.0],
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::RGBA(value) => format!(
                    "{:02x}{:02x}{:02x}{:02x}",
                    (value[0] * 255.0) as u8,
                    (value[1] * 255.0) as u8,
                    (value[2] * 255.0) as u8,
                    (value[3] * 255.0) as u8
                ),
                Color::RGB(value) => format!(
                    "{:02x}{:02x}{:02x}",
                    (value[0] * 255.0) as u8,
                    (value[1] * 255.0) as u8,
                    (value[2] * 255.0) as u8
                ),
            }
        )
    }
}
