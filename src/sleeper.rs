/// A sleeper struct for pausing the execution of a thread.
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
            short: Duration::from_millis(300),
            medium: Duration::from_millis(1000),
            long: Duration::from_secs(2),
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
