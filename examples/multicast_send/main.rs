use tokio::net::UdpSocket;

#[tokio::main] 
async fn main() -> std::io::Result<()> {
    // 绑定到任意地址和端口
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    
    // 设置组播地址和端口
    let multicast_addr = "239.255.255.250:3000";

    // 将消息发送到组播地址
    socket.send_to("Hello multicast!".as_bytes(), multicast_addr).await?;
    println!("Message has been sent out");

    Ok(())
}
