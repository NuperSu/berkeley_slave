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

    // Clone the Arc for use in each async task
    let slave_time_adjust_for_adjustments = Arc::clone(&slave_time_adjust);
    let slave_time_adjust_for_reporting = Arc::clone(&slave_time_adjust);

    let time_adjust_handle = tokio::spawn(async move {
        if let Err(e) = slave_time_adjust_for_adjustments.listen_for_adjustments().await {
            eprintln!("Error while listening for adjustments: {}", e);
        }
    });

    let report_time_handle = tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await; // Adjust as needed
            if let Err(e) = slave_time_adjust_for_reporting.report_current_time().await {
                eprintln!("Error reporting current time: {}", e);
            }
        }
    });

    let _ = tokio::try_join!(time_adjust_handle, report_time_handle);

    Ok(())
}
