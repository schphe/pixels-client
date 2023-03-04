pub type CanvasResult = Result<(), CanvasError>;

#[derive(Debug)]
pub enum CanvasError {
    ClientError,
    Cooldown
}

impl From<ureq::Error> for CanvasError {
    fn from(_value: ureq::Error) -> Self {
        return CanvasError::ClientError
    }
}
