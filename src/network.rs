use tokio::net::{UdpSocket};
use tokio::time::{Duration, timeout};
use std::net::SocketAddr;
use std::io::{Result, Error, ErrorKind};

pub async fn send_message(socket: &UdpSocket, message: &str, addr: &str) -> Result<()> {
    socket.send_to(message.as_bytes(), addr).await?;
    Ok(())
}

pub async fn receive_message(socket: &UdpSocket, timeout_duration: Duration) -> Result<(String, SocketAddr)> {
    let mut buf = [0; 1024];
    let (received, sender_address) = match timeout(timeout_duration, socket.recv_from(&mut buf)).await {
        Ok(Ok(result)) => result, // Directly using result which is a tuple (usize, SocketAddr)
        Ok(Err(e)) => return Err(e),
        Err(_) => return Err(Error::new(ErrorKind::TimedOut, "Receive timed out")),
    };

    let received_msg = String::from_utf8_lossy(&buf[..received]).to_string();
    Ok((received_msg, sender_address))
}