// node_manager.rs
use std::net::Ipv4Addr;
use tokio::time::Instant;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::udp_connection;
use tokio::time::{self, Duration};
use tokio::net::UdpSocket;
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub name: String, // UUID
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub last_active: Instant,
    pub alias: Option<String>, // Alias is optional and not set by default
}

impl NodeInfo {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        NodeInfo {
            ip,
            port,
            last_active: Instant::now(),
            alias: None, // Default to None
        }
    }
}

// Utility functions to manage nodes in a thread-safe manner
pub struct NodeManager {
    pub nodes: Arc<Mutex<HashMap<String, NodeInfo>>>,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub uuid: String,
}

impl NodeManager {
    pub fn new(ip: Ipv4Addr, port: u16, uuid: String) -> Self {
        NodeManager {
            nodes: Arc::new(Mutex::new(HashMap::new())),
            ip,
            port,
            uuid,
        }
    }

    pub async fn process_message(&self, message_data: Vec<u8>) {
        match serde_json::from_slice::<Message>(&message_data) {
            Ok(message) => {
                println!(
                    "Received Message: IP = {}, Port = {}, UUID = {}, Content = {}",
                    message.ip, message.port, message.name, message.content
                );

                self.add_or_update_node(message.name.clone(), message.ip, message.port).await;

            },
            Err(e) => {
                println!("Failed to deserialize message: {}", e);
            }
        }
    }

    pub async fn list_users(&self) -> Vec<String> {
        let nodes = self.nodes.lock().await;
        nodes.iter()
            .map(|(uuid, node)| format!("UUID: {}, IP: {}, Port: {}, Alias: {:?}", uuid, node.ip, node.port, node.alias))
            .collect()
    }

    pub async fn send_message(&self, uuid: &str, content: &str) -> Result<(), String> {
        let nodes = self.nodes.lock().await;
        if let Some(node_info) = nodes.get(uuid) {
            // 构建 Message 结构体
            let message = Message {
                ip: self.ip,
                port: self.port,
                name: self.uuid.clone(),
                content: content.to_string(),
            };

            // 将 Message 序列化为 JSON
            let serialized_message = serde_json::to_string(&message)
                .map_err(|e| format!("Failed to serialize message: {}", e))?;

            // 调用发送 UDP 消息的函数
            match udp_connection::send_message(node_info.ip, node_info.port, &serialized_message).await {
                Ok(_) => {
                    println!("Message '{}' sent to UUID: {} at {}:{}", content, uuid, node_info.ip, node_info.port);
                    Ok(())
                },
                Err(e) => Err(format!("Failed to send message: {}", e)),
            }
        } else {
            Err(format!("UUID {} not found", uuid))
        }
    }
    

    // Asynchronously add a node
    pub async fn add_or_update_node(&self, uuid: String, ip: Ipv4Addr, port: u16) -> Result<bool, String> {
        let mut nodes = self.nodes.lock().await;
        let node = NodeInfo::new(ip, port);
        match nodes.entry(uuid.clone()) {
            std::collections::hash_map::Entry::Vacant(e) => {
                // 如果 UUID 不存在，则插入新节点
                e.insert(node);
                Ok(true) // 返回 true 表示这是一个新节点
            },
            std::collections::hash_map::Entry::Occupied(mut e) => {
                // 如果 UUID 已存在，更新 last_active 时间并保留其他信息
                e.get_mut().last_active = Instant::now(); // 假设 NodeInfo 中有 last_active 字段
                Ok(false) // 返回 false 表示这是一个更新的老节点
            }
        }
    }
    


    // Asynchronously update a node's alias
    pub async fn update_node_alias(&self, uuid: &str, alias: String) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        // Check if the new alias is already in use by another node
        if nodes.values().any(|node| node.alias.as_ref() == Some(&alias)) {
            return Err("Alias already exists".to_string());
        }
    
        if let Some(node) = nodes.get_mut(uuid) {
            node.alias = Some(alias);  // Update the alias
            return Ok(());
        }
    
        Err("UUID not found".to_string())
    }

    // Asynchronously get node information
    pub async fn get_node_info(&self, uuid: &str) -> Option<NodeInfo> {
        let nodes = self.nodes.lock().await;
        nodes.values().find(|node| node.alias.as_ref().map(String::as_str) == Some(uuid))
            .or_else(|| nodes.get(uuid))
            .cloned() // Clone the data to release the lock
    }
    
    // Asynchronously remove a node
    pub async fn remove_node(&self, uuid: String) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        if !nodes.contains_key(&uuid) {
            return Err("UUID does not exist".to_string());
        }
        nodes.remove(&uuid);
        Ok(())
    }

     // Asynchronously check and notify offline nodes
     pub async fn check_and_notify_offline_nodes(&self, notify_tx: &mpsc::Sender<String>) -> Result<(), String> {
        let now = Instant::now();
        let mut offline_nodes = Vec::new();

        {
            let mut nodes_locked = self.nodes.lock().await;

            // Check each node's last active time and collect names of offline nodes
            nodes_locked.retain(|name, node_info| {
                if now.duration_since(node_info.last_active) > Duration::from_secs(20) {
                    offline_nodes.push(name.clone());
                    false // Remove the node from the map
                } else {
                    true // Keep the node in the map
                }
            });
        }

        // Send offline notifications for each offline node
        for name in offline_nodes {
            notify_tx.send(format!("Node {} went offline!", name)).await.map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
