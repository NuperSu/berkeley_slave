use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct TimeKeeper {
    offset: Arc<Mutex<i64>>, // Shared state among asynchronous tasks
}

impl TimeKeeper {
    pub fn new() -> Self {
        TimeKeeper {
            offset: Arc::new(Mutex::new(0)),
        }
    }

    pub fn adjust_time(&self, adjustment: i64) {
        let mut offset = self.offset.lock().unwrap();
        *offset += adjustment;
        println!("Adjusted time by {}ms. New offset: {}ms", adjustment, *offset);
    }

    pub fn current_time(&self) -> i64 {
        let offset = self.offset.lock().unwrap();
        chrono::Utc::now().timestamp_millis() + *offset
    }
}
