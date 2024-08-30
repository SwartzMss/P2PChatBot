use tokio::net::UdpSocket;
use tokio::time::{self, Duration, Instant};
use std::collections::HashMap;
use serde_json::{from_str, to_string};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::net::SocketAddr;
use std::net::Ipv4Addr;
use tokio::io::{self, ErrorKind};
use crate::node_manager::{Message, NodeInfo,NodeManager};

pub async fn network_monitor(
    notify_tx: mpsc::Sender<String>,
    node_manager: Arc<Mutex<NodeManager>>,
    name:String
) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    socket.join_multicast_v4("239.255.255.250".parse().unwrap(), "0.0.0.0".parse().unwrap())?;

    let mut buf = [0u8; 1024];
    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        tokio::select! {
            Ok((size, _)) = socket.recv_from(&mut buf) => {
                if let Ok(msg_str) = String::from_utf8(buf[..size].to_vec()) {
                    // println!("Multicast message receive: {:?}", msg_str);
                    if let Ok(message) = from_str::<Message>(&msg_str) {
                        let node_name = &message.name;
                        if name == *node_name{
                            continue;
                        }
                        let node_manager = node_manager.lock().await; // 先获取锁
                        if let Ok(true) = node_manager.add_or_update_node(message.name.clone(), message.ip, message.port).await {
                            notify_tx.send(format!("Node {} came online!", message.name)).await.unwrap();
                        }
                    }
                }
            },
            _ = interval.tick() => {
                let node_manager = node_manager.lock().await; // 先获取锁
                node_manager.check_and_notify_offline_nodes(&notify_tx).await;
            }
        }
    }
}


pub async fn multicast_sender(multicast_addr: &str, communication_ip:  String, communication_port: u16,node_name:String,) -> tokio::io::Result<()> {
    let multicast_socket = UdpSocket::bind("0.0.0.0:0").await?;
    // multicast_socket.set_multicast_loop_v4(false)?;

    let multicast_addr: SocketAddr = multicast_addr.parse().map_err(|e| {
        io::Error::new(ErrorKind::InvalidInput, e)
    })?;

    let mut interval = time::interval(Duration::from_secs(5));
    let communication_ip: Ipv4Addr = communication_ip.parse().map_err(|_e| {
        tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, "Invalid IP address")
    })?;
    
    loop {
        interval.tick().await;
        let message = Message {
            ip: communication_ip,
            port: communication_port,
            name: node_name.clone(),
            content: format!("Node is online at {}:{}", communication_ip, communication_port),
        };
        let message_json = to_string(&message)?;
        multicast_socket.send_to(&message_json.as_bytes(), &multicast_addr).await?;
        // println!("Multicast message sent: {:?}", message);
    }
}
