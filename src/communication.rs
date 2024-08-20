use tokio::net::UdpSocket;
use std::io::Result;

pub async fn start_communication(local_ip: &str, local_port: u16, remote_ip: &str, remote_port: u16) -> Result<()> {
    let local_addr = format!("{}:{}", local_ip, local_port);
    let remote_addr = format!("{}:{}", remote_ip, remote_port);

    let socket = UdpSocket::bind(&local_addr).await?;
    println!("Bound to local address: {}", &local_addr);

    let message = b"Hello, world!";
    loop {
        // 发送消息
        socket.send_to(message, &remote_addr).await?;

        // 接收消息
        let mut buf = [0u8; 1024];
        let (len, peer) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}: {:?}", len, peer, &buf[..len]);
    }
}
