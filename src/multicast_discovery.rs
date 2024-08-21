use tokio::net::UdpSocket;
use tokio::time::{self, Duration, Instant};
use std::collections::HashMap;
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::node_manager::{Message, NodeInfo};

pub async fn network_monitor(
    multicast_addr: &str,
    notify_tx: mpsc::Sender<String>,
    nodes: Arc<Mutex<HashMap<String, NodeInfo>>>,
) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.join_multicast_v4("239.255.255.250".parse().unwrap(), "0.0.0.0".parse().unwrap())?;

    let mut buf = [0u8; 1024];
    let mut interval = time::interval(Duration::from_secs(10));
    let start_time = Instant::now();

    loop {
        tokio::select! {
            Ok((size, _)) = socket.recv_from(&mut buf) => {
                if let Ok(msg_str) = String::from_utf8(buf[..size].to_vec()) {
                    if let Ok(message) = from_str::<Message>(&msg_str) {
                        let node_name = &message.name;
                        let mut nodes_locked = nodes.lock().await;
                        if start_time.elapsed() > Duration::from_secs(5) {
                            let node_info = NodeInfo::new(message.ip, message.port);
                            let is_new_node = nodes_locked.insert(node_name.clone(), node_info).is_none();
                            if is_new_node {
                                notify_tx.send(format!("Node {} came online!", node_name)).await.unwrap();
                            }
                        } else {
                            nodes_locked.entry(node_name.clone()).or_insert_with(|| NodeInfo::new(message.ip, message.port));
                        }
                    }
                }
            },
            _ = interval.tick() => {
                let now = Instant::now();
                let mut offline_nodes = Vec::new();
                let mut nodes_locked = nodes.lock().await;

                for (name, node_info) in nodes_locked.iter() {
                    if now.duration_since(node_info.last_active) > Duration::from_secs(20) {
                        offline_nodes.push(name.clone());
                    }
                }

                for name in offline_nodes {
                    nodes_locked.remove(&name);
                    notify_tx.send(format!("Node {} went offline!", name)).await.unwrap();
                }
            }
        }
    }
}


pub async fn multicast_sender(multicast_addr: &str, communication_ip: IpAddr, communication_port: u16) -> tokio::io::Result<()> {
    let multicast_socket = UdpSocket::bind("0.0.0.0:0").await?;
    multicast_socket.set_multicast_loop_v4(false)?;
    let multicast_addr = multicast_addr.parse::<std::net::SocketAddr>()?;

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        let message = Message {
            ip: communication_ip,
            port: communication_port,
            name: Uuid::new_v4().to_string(),
            content: format!("Node is online at {}:{}", communication_ip, communication_port),
        };
        let message_json = to_string(&message)?;
        multicast_socket.send_to(&message_json.as_bytes(), &multicast_addr).await?;
        println!("Multicast message sent: {:?}", message);
    }
}
