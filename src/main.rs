mod time_management;
mod network;

use tokio::{time::{sleep, Duration}, net::UdpSocket};
use std::{env, error::Error};
use time_management::TimeKeeper;
use network::handle_message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Usage: {} [Slave Node Address]", args.next().unwrap());
        return Ok(());
    }

    let slave_address = args.nth(1).unwrap();
    println!("Slave node starting. Binding to address: {}", slave_address);
    let socket = UdpSocket::bind(&slave_address).await?;
    println!("Slave node running on {}", slave_address);

    let time_keeper = TimeKeeper::new();

    loop {
        let mut buf = [0; 1024];
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).await?;
        let received_message = String::from_utf8_lossy(&buf[..number_of_bytes]).into_owned();

        println!("Received message from {}: {}", src_addr, received_message);
        handle_message(received_message, &src_addr, &socket, &time_keeper).await?;
        sleep(Duration::from_secs(5)).await; // Customize based on how frequently you want to update or respond
    }
}
