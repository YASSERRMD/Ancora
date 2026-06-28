use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooldown {
    pub scale_up_secs: i64,
    pub scale_down_secs: i64,
    last_scale_up: Option<DateTime<Utc>>,
    last_scale_down: Option<DateTime<Utc>>,
}

impl Cooldown {
    pub fn new(scale_up_secs: i64, scale_down_secs: i64) -> Self {
        Cooldown {
            scale_up_secs,
            scale_down_secs,
            last_scale_up: None,
            last_scale_down: None,
        }
    }

    pub fn can_scale_up(&self) -> bool {
        match self.last_scale_up {
            None => true,
            Some(t) => Utc::now() >= t + Duration::seconds(self.scale_up_secs),
        }
    }

    pub fn can_scale_down(&self) -> bool {
        match self.last_scale_down {
            None => true,
            Some(t) => Utc::now() >= t + Duration::seconds(self.scale_down_secs),
        }
    }

    pub fn record_scale_up(&mut self) {
        self.last_scale_up = Some(Utc::now());
    }

    pub fn record_scale_down(&mut self) {
        self.last_scale_down = Some(Utc::now());
    }
}
