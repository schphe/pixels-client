use chrono::{
    DateTime,
    Duration,
    Utc,
};

pub struct Cooldown{
    time: DateTime<Utc>
}

impl Cooldown {
    pub fn new() -> Self {
        Self {
            time: DateTime::default()
        }
    }

    pub fn is_ended(&self) -> bool {
        Utc::now() >= self.time
    }

    pub fn set(&mut self, seconds: f32) {
        self.time = Utc::now() + Duration::milliseconds((seconds * 1000.0) as i64);
    }

    pub fn remaining(&self) -> f32 {
        ((self.time - Utc::now()).num_milliseconds() as f32 / 1000.0).max(0.0)
    }
}
