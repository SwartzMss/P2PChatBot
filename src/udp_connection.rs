use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::net::{SocketAddrV4, Ipv4Addr, SocketAddr};
use std::io;

pub async fn start_listening(socket: Arc<Mutex<UdpSocket>>, sender: mpsc::Sender<Vec<u8>>) -> io::Result<()> {
    let mut buf = vec![0; 1024];

    loop {
        let (len, _addr) = {
            let mut socket = socket.lock().await;
            socket.recv_from(&mut buf).await?
        };

        // 将接收到的数据发送到通道
        if sender.send(buf[..len].to_vec()).await.is_err() {
            println!("Failed to send message to NodeManager");
            break;
        }
    }

    Ok(())
}

pub async fn send_message(ip: Ipv4Addr, port: u16, message: &str) -> io::Result<()> {

    let ip = if ip == Ipv4Addr::new(0, 0, 0, 0) {
        Ipv4Addr::new(127, 0, 0, 1)
    } else {
        ip
    };
    // 使用传入的 IP 和端口构建 SocketAddr
    let remote_addr = SocketAddr::V4(SocketAddrV4::new(ip, port));

    // 创建一个新的 UdpSocket 并绑定到随机端口
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    // 发送消息到指定的远程地址
    socket.send_to(message.as_bytes(), &remote_addr).await?;

    println!("Message sent to {}", remote_addr);

    Ok(())
}