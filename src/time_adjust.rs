use tokio::net::UdpSocket;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::network::{send_message, receive_message};
use std::sync::{Arc, Mutex};
use std::error::Error;
use tokio::time::Duration;
#[derive(Serialize, Deserialize, Debug)]
struct TimeMessage {
    #[serde(rename = "type")]
    msg_type: String,
    time: Option<i64>,
    adjustment: Option<i64>,
}

pub struct SlaveTimeAdjust {
    socket: Arc<UdpSocket>,
    time_offset: Arc<Mutex<i64>>, // Adjusted time offset in milliseconds
}

impl SlaveTimeAdjust {
    pub fn new(socket: Arc<UdpSocket>) -> Self {
        Self {
            socket,
            time_offset: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn listen_for_adjustments(&self) -> Result<(), Box<dyn Error>> {
        loop {
            let (message, sender_address) = receive_message(&self.socket, Duration::from_secs(1_000)).await?;
            match serde_json::from_str::<TimeMessage>(&message) {
                Ok(msg) => {
                    match msg.msg_type.as_str() {
                        "adjust_time" => {
                            if let Some(adjustment) = msg.adjustment {
                                self.adjust_time(adjustment).await;
                            } else {
                                eprintln!("Adjustment value missing in message");
                            }
                        },
                        "request_time" => {
                            self.report_current_time(&sender_address.to_string()).await?;
                        },
                        _ => eprintln!("Unknown message type received: {}", msg.msg_type),
                    }
                },
                Err(e) => eprintln!("Failed to parse received message: {}", e),
            }
        }
    }

    async fn adjust_time(&self, adjustment: i64) {
        let mut time_offset = self.time_offset.lock().unwrap();
        *time_offset -= adjustment;
        println!("Time adjusted by {}ms. New offset: {}ms", adjustment, *time_offset);
    }

    pub async fn report_current_time(&self, sender_address: &str) -> Result<(), Box<dyn Error>> {
        let current_time = {
            let time_offset = self.time_offset.lock().unwrap();
            Utc::now().timestamp_millis() + *time_offset
        };

        let message = serde_json::to_string(&TimeMessage {
            msg_type: "time_report".to_string(),
            time: Some(current_time),
            adjustment: None,
        })?;

        send_message(&self.socket, &message, sender_address).await?;
        Ok(())
    }
}
