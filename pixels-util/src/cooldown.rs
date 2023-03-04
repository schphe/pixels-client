use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Copy, Clone)]
#[derive(Default)]
pub struct Cooldown{
    time: Option<DateTime<Utc>>
}

impl Cooldown{
    pub fn set(&mut self, seconds: f32){
        self.time = Some(Utc::now() + Duration::milliseconds((seconds * 1000.0) as i64));
    }

    pub fn is_ended(&self) -> bool {
        self.time.map(|time| Utc::now() >= time).unwrap_or(true)
    }

    pub fn remaining(&self) -> f32{
        self.time.map(|time| ((time - Utc::now()).num_milliseconds() as f32 / 1000.0).max(0.0)).unwrap_or(0.0)
    }
}
