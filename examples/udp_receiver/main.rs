use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080")?; // 绑定到指定的端口
    println!("Listening on 0.0.0.0:8080");

    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf)?;

    println!(
        "Received {} bytes from {}: {}",
        amt,
        src,
        String::from_utf8_lossy(&buf[..amt])
    );

    Ok(())
}
