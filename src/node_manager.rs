use std::net::Ipv4Addr;
use std::time::Instant;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    nodes: Arc<Mutex<HashMap<String, NodeInfo>>>,
}

impl NodeManager {
    pub fn new() -> Self {
        NodeManager {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Asynchronously add a node
    pub async fn add_node(&self, ip: Ipv4Addr, port: u16, alias: Option<String>) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        if let Some(ref a) = alias {
            if nodes.values().any(|node| node.alias.as_ref() == Some(a)) {
                return Err("Alias already exists".to_string());
            }
        }
        let node = NodeInfo::new(ip, port);
        let uuid = format!("{}:{}", ip, port); // UUID based on IP and port
        nodes.insert(uuid, node);
        Ok(())
    }

    // Asynchronously update a node's alias
    pub async fn update_node_alias(&self, uuid: &str, alias: Option<String>) -> Result<(), String> {
        let mut nodes = self.nodes.lock().await;
        if let Some(ref a) = alias {
            if nodes.values().any(|node| node.alias.as_ref() == Some(a)) {
                return Err("Alias already exists".to_string());
            }
        }
        if let Some(node) = nodes.get_mut(uuid) {
            node.alias = alias;
            return Ok(());
        }
        Err("UUID not found".to_string())
    }

    // Asynchronously get node information
    pub async fn get_node_info(&self, identifier: &str) -> Option<NodeInfo> {
        let nodes = self.nodes.lock().await;
        nodes.values().find(|node| node.alias.as_ref() == Some(identifier))
            .or_else(|| nodes.get(identifier))
            .cloned() // Clone the data to release the lock
    }
}
