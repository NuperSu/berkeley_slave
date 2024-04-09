use tokio::net::UdpSocket;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::network::{send_message, receive_message};
use std::sync::{Arc, Mutex};
use std::error::Error;
use tokio::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct TimeAdjustmentMessage {
    adjustment: i64, // Adjustment in milliseconds
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
            match serde_json::from_str::<TimeAdjustmentMessage>(&message) {
                Ok(adjustment_msg) => {
                    self.adjust_time(adjustment_msg.adjustment).await;
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

    // Optionally, implement a function to periodically report current time to the master
    pub async fn report_current_time(&self) -> Result<(), Box<dyn Error>> {
        let time_offset = self.time_offset.lock().unwrap();
        let current_time = Utc::now().timestamp_millis() + *time_offset;
        let message = serde_json::to_string(&TimeAdjustmentMessage {
            adjustment: current_time, // Here 'adjustment' field is reused to send current time
        })?;
        send_message(&self.socket, &message, &self.master_address).await?;
        Ok(())
    }
}
