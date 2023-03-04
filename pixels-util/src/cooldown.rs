use std::fmt::Display;

use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Copy, Clone)]
pub struct Cooldown{
    time: Option<DateTime<Utc>>
}

impl Cooldown{
    pub fn set_cooldown(&mut self, seconds: f32){
        self.time = Some(Utc::now() + Duration::milliseconds((seconds * 1000.0) as i64));
    }

    pub fn is_cooldown_ended(&self) -> bool {
        self.time.and_then(|time| Some(Utc::now() >= time)).unwrap_or(true)
    }

    fn get_cooldown_secs(&self) -> f32{
        self.time.and_then(|time| Some(((time - Utc::now()).num_milliseconds() as f32 / 1000.0).max(0.0))).unwrap_or(0.0)
    }
}

impl Default for Cooldown {
    fn default() -> Self {
        Self { time: None }
    }
}

impl From<Cooldown> for f32{
    fn from(value: Cooldown) -> Self {
        value.get_cooldown_secs()
    }
}

impl Display for Cooldown{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_cooldown_secs().round())
    }
}

