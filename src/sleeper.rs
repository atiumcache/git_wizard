// src/sleeper.rs

use std::thread::sleep;
use std::time::Duration;

pub struct Sleeper {
    pub short: Duration,
    pub medium: Duration,
    pub long: Duration,
}

impl Sleeper {
    pub fn new() -> Self {
        Sleeper {
            short: Duration::from_millis(100),
            medium: Duration::from_millis(500),
            long: Duration::from_secs(1),
        }
    }

    pub fn sleep_short(&self) {
        sleep(self.short);
    }

    pub fn sleep_medium(&self) {
        sleep(self.medium);
    }

    pub fn sleep_long(&self) {
        sleep(self.long);
    }
}