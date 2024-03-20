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

        let current_simulated_time_before_adjustment = current_system_time + *offset;

        *offset = new_time - current_system_time;

        let adjustment = new_time - current_simulated_time_before_adjustment;

        println!("Time adjusted by {}ms. New offset: {}ms", adjustment, *offset);

        if adjustment > 0 {
            println!("Time was moved forward by {}ms.", adjustment);
        } else if adjustment < 0 {
            println!("Time was moved backward by {}ms.", -adjustment);
        } else {
            println!("Time remains unchanged.");
        }
    }

    pub fn current_time(&self) -> i64 {
        let offset = self.offset.lock().unwrap();
        Utc::now().timestamp_millis() + *offset
    }
}
