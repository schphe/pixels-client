use pixels_util::cooldown::Cooldown;

pub type CanvasResult = Result<(), CanvasError>;

#[derive(Debug)]
pub enum CanvasError {
    ClientError,
    Cooldown(Cooldown)
}

impl From<ureq::Error> for CanvasError {
    fn from(_value: ureq::Error) -> Self {
        return CanvasError::ClientError
    }
}
