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

    let slave_time_adjust = Arc::new(SlaveTimeAdjust::new(Arc::clone(&socket), master_address.clone()));

    // Listening for messages from the master, including time requests and adjustments.
    let _ = slave_time_adjust.listen_for_adjustments().await;

    Ok(())
}
