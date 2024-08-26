use tokio::net::UdpSocket;

#[tokio::main] // 使用 tokio 的异步主函数
async fn main() -> std::io::Result<()> {
    // 绑定到一个具体的地址和端口
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;

    // 加入组播组
    socket.join_multicast_v4("239.255.255.250".parse().unwrap(), "0.0.0.0".parse().unwrap())?;

    let mut buf = [0; 1024]; // 数据接收缓冲区

    loop {
        let (num_bytes, src) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", num_bytes, src);
        println!("Message: {}", String::from_utf8_lossy(&buf[..num_bytes]));
    }
}
