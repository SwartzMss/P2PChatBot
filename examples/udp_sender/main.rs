use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?; // 绑定到任意可用的端口
    let remote_addr = "127.0.0.1:59010"; // 指定目标地址和端口
    let message = b"Hello, UDP!";

    socket.send_to(message, remote_addr)?;
    println!("Message sent to {}", remote_addr);

    Ok(())
}
