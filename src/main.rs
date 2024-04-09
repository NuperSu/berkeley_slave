use tokio::net::UdpSocket;
use std::env;
use std::sync::Arc;
use std::error::Error;

mod network;
mod time_adjust;

use time_adjust::SlaveTimeAdjust;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <Slave Bind Address> <Master Node Address>", args[0]);
        return Err("Insufficient arguments provided".into());
    }

    let slave_bind_address = &args[1];
    let master_address = &args[2];

    let socket = Arc::new(UdpSocket::bind(slave_bind_address).await?);
    println!("Slave node bound to {}", socket.local_addr()?);

    let slave_time_adjust = SlaveTimeAdjust::new(Arc::clone(&socket), master_address.clone());

    // Instead of spawning a separate task, invoke listen_for_adjustments directly in the main task.
    // This keeps the main task alive and continuously listening for messages.
    if let Err(e) = slave_time_adjust.listen_for_adjustments().await {
        eprintln!("Error while listening for adjustments: {}", e);
    }

    Ok(())
}
