#[derive(Debug)]
pub enum CanvasError {
    Client(ureq::Error),
    Cooldown(f32),
}

impl From<ureq::Error> for CanvasError {
    fn from(value: ureq::Error) -> Self {
        Self::Client(value)
    }
}
