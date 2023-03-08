pub mod cooldown;
pub mod pixels;
pub mod color;

pub mod prelude {
    pub use crate::{
        color::{
            Color,
            ColorMode
        },
        pixels::{
            Pixels,
            PixelsIterator
        },
        cooldown::Cooldown,
    };
}



#[macro_export]
macro_rules! from {
    ($from:ty, $to:ident, $body:expr $(,)?) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                $body(value)
            }
        }
    };
}

#[macro_export]
macro_rules! into {
    ($from:ident, $to:ty, $body:expr $(,)?) => {
        impl Into<$to> for $from {
            fn into(self) -> $to {
                $body(self)
            }
        }
    };
}
