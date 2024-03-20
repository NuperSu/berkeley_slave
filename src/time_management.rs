use std::sync::{Arc, Mutex};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct TimeKeeper {
    offset: Arc<Mutex<i64>>, // The offset to the real system time in milliseconds.
}

impl TimeKeeper {
    pub fn new() -> Self {
        TimeKeeper {
            offset: Arc::new(Mutex::new(0)),
        }
    }

    pub fn adjust_time(&self, new_time: i64) {
        let current_system_time = Utc::now().timestamp_millis();
        let mut offset = self.offset.lock().unwrap();
        *offset = new_time - current_system_time;

        // Directly use the new_time for logging to avoid confusion
        println!("Time adjusted. New simulated current time (Unix timestamp in ms): {}", new_time);
    }

    pub fn current_time(&self) -> i64 {
        let offset = self.offset.lock().unwrap();
        Utc::now().timestamp_millis() + *offset
    }
}
