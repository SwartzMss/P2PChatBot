use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io;
use std::net::SocketAddr;

pub async fn start_listening(socket: Arc<Mutex<UdpSocket>>) -> io::Result<()> {
    let mut buf = vec![0; 1024];

    loop {
        let (len, addr) = {
            let mut socket = socket.lock().await;
            socket.recv_from(&mut buf).await?
        };
        println!(
            "Received {} bytes from {}: {}",
            len,
            addr,
            String::from_utf8_lossy(&buf[..len])
        );
    }
}

pub async fn send_message(socket: Arc<Mutex<UdpSocket>>, remote_addr: &str, message: &str) -> io::Result<()> {
    let remote: SocketAddr = remote_addr.parse().unwrap();  // 确保 remote_addr 是一个有效的 SocketAddr

    {
        let mut socket = socket.lock().await;
        socket.send_to(message.as_bytes(), &remote).await?;  // 明确指定 remote 是 SocketAddr 类型
    }

    println!("Message sent to {}", remote);

    Ok(())
}
