use pixels_util::cooldown::Cooldown;

#[derive(Debug)]
pub enum CanvasError {
    ClientError,
    Cooldown(Cooldown),
}
