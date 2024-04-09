use tokio::net::UdpSocket;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::network::{send_message, receive_message};
use std::sync::{Arc, Mutex};
use std::error::Error;
use tokio::time::Duration;
#[derive(Serialize, Deserialize, Debug)]
struct TimeMessage {
    msg_type: String,
    time: Option<i64>,
    adjustment: Option<i64>,
}

pub struct SlaveTimeAdjust {
    socket: Arc<UdpSocket>,
    master_address: String,
    time_offset: Arc<Mutex<i64>>, // Adjusted time offset in milliseconds
}

impl SlaveTimeAdjust {
    pub fn new(socket: Arc<UdpSocket>, master_address: String) -> Self {
        Self {
            socket,
            master_address,
            time_offset: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn listen_for_adjustments(&self) -> Result<(), Box<dyn Error>> {
        loop {
            let message = receive_message(&self.socket, Duration::from_secs(5)).await?;
            match serde_json::from_str::<TimeMessage>(&message) {
                Ok(adjustment_msg) => {
                    // Check if `adjustment` is Some and then call `adjust_time` with the unwrapped value
                    if let Some(adjustment) = adjustment_msg.adjustment {
                        self.adjust_time(adjustment).await;
                    } else {
                        // Optionally, handle the case where `adjustment` is None
                        eprintln!("Adjustment value missing in message");
                    }
                },
                Err(e) => eprintln!("Failed to parse adjustment message: {:?}", e),
            }
        }
    }

    async fn adjust_time(&self, adjustment: i64) {
        let mut time_offset = self.time_offset.lock().unwrap();
        *time_offset += adjustment;
        println!("Time adjusted by {}ms. New offset: {}ms", adjustment, *time_offset);
    }

    pub async fn report_current_time(&self) -> Result<(), Box<dyn Error>> {
        // Clone or copy necessary data before the async block
        let socket_clone = self.socket.clone();
        let master_address = self.master_address.clone();

        // Scope to ensure MutexGuard is dropped before await
        let current_time = {
            let time_offset = self.time_offset.lock().unwrap(); // Lock is only held within this scope
            Utc::now().timestamp_millis() + *time_offset // Calculate current time
        }; // MutexGuard is dropped here

        let message = serde_json::to_string(&TimeMessage {
            msg_type: "time_report".to_string(),
            time: Some(current_time),
            adjustment: None, // Not used for time reports, can be None
        })?;

        send_message(&socket_clone, &message, &master_address).await?;
        Ok(())
    }
}
