use tokio::net::UdpSocket;
use std::net::SocketAddr;
use crate::time_management::TimeKeeper;
use serde_json::{Value, json};
use std::error::Error;

pub async fn handle_message(msg: String, src_addr: &SocketAddr, socket: &UdpSocket, time_keeper: &TimeKeeper) -> Result<(), Box<dyn Error>> {
    let parsed_msg: Value = serde_json::from_str(&msg)?;
    match parsed_msg["type"].as_str() {
        Some("adjust_time") => {
            if let Some(adjustment) = parsed_msg["adjustment"].as_i64() {
                time_keeper.adjust_time(adjustment);
            }
        }
        _ => println!("Unknown message: {}", msg),
    }

    // Periodically send the current time to the master
    let report = json!({
        "type": "time_report",
        "time": time_keeper.current_time(),
    }).to_string();
    socket.send_to(report.as_bytes(), src_addr).await?;
    Ok(())
}
