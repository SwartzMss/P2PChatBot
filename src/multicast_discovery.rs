use tokio::sync::mpsc;
use tokio::time::{self, Duration, Instant};
use tokio::net::UdpSocket;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    ip: Ipv4Addr,
    port: u16,
    name: String,
    content: String,
}


struct NodeInfo {
    last_active: Instant,
}

pub async fn multicast_listener(multicast_addr: &str, mut msg_tx: mpsc::Sender<Message>) -> tokio::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.join_multicast_v4("239.255.255.250".parse::<Ipv4Addr>().unwrap(), "0.0.0.0".parse::<Ipv4Addr>().unwrap())?;


    let mut buf = [0u8; 1024];
    loop {
        let (size, _sender) = socket.recv_from(&mut buf).await?;
        if let Ok(msg_str) = String::from_utf8(buf[..size].to_vec()) {
            if let Ok(message) = serde_json::from_str::<Message>(&msg_str) {
                msg_tx.send(message).await.unwrap();
            }
        }
    }
}

pub async fn node_monitor(mut msg_rx: mpsc::Receiver<Message>, mut notify_tx: mpsc::Sender<String>) {
    let mut nodes = HashMap::new();
    let mut interval = time::interval(Duration::from_secs(10));
    let start_time = Instant::now();  // 跟踪启动时间

    loop {
        tokio::select! {
            Some(message) = msg_rx.recv() => {
                let node_name = &message.name;
                // 判断是否超过初始化期5秒
                if start_time.elapsed() > Duration::from_secs(5) {
                    let is_new_node = nodes.insert(node_name.clone(), NodeInfo { last_active: Instant::now() }).is_none();
                    if is_new_node {
                        notify_tx.send(format!("Node {} came online!", node_name)).await.unwrap();
                    }
                } else {
                    // 在5秒内，仅更新活动时间
                    nodes.entry(node_name.clone()).or_insert_with(|| NodeInfo { last_active: Instant::now() });
                }
            },
            _ = interval.tick() => {
                let now = Instant::now();
                let mut offline_nodes = Vec::new();

                for (name, node_info) in &nodes {
                    if now.duration_since(node_info.last_active) > Duration::from_secs(20) {
                        offline_nodes.push(name.clone());
                    }
                }

                for name in offline_nodes {
                    nodes.remove(&name);
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


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use std::net::{IpAddr, Ipv4Addr};

    #[tokio::test]
    async fn test_node_monitor_initialization_period() {
        let (msg_tx, msg_rx) = mpsc::channel(32);
        let (notify_tx, mut notify_rx) = mpsc::channel(32);

        let test_ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let test_port = 8080;
        let test_message = Message {
            ip: test_ip,
            port: test_port,
            name: "TestNode".to_string(),
            content: "Hello, world!".to_string(),
        };

        tokio::spawn(async move {
            node_monitor(msg_rx, notify_tx).await;
        });

        // 发送测试消息
        msg_tx.send(test_message).await.unwrap();

        // 给足够时间处理消息
        tokio::time::sleep(Duration::from_secs(6)).await;

        // 检查通知
        match notify_rx.recv().await {
            Some(notification) => assert_eq!(notification, "Node TestNode came online!"),
            None => panic!("No notification received, but expected one for TestNode."),
        }
    }
}
