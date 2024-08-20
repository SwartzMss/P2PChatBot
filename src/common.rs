use std::net::IpAddr;
use std::time::Instant;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub ip: IpAddr,
    pub port: u16,
    pub name: String, // UUID
    pub content: String,
}

#[derive(Debug)]
pub struct NodeInfo {
    pub ip: IpAddr,
    pub port: u16,
    pub last_active: Instant,
    pub alias: Option<String>, // Alias is now optional
}

impl NodeInfo {
    pub fn new(ip: IpAddr, port: u16, alias: Option<String>) -> Self {
        NodeInfo {
            ip,
            port,
            last_active: Instant::now(),
            alias,
        }
    }

    // Check if alias is unique across all nodes
    pub fn is_alias_unique(nodes: &HashMap<String, NodeInfo>, alias: &str) -> bool {
        !nodes.values().any(|node| node.alias.as_ref() == Some(alias))
    }

    // Add a node, ensuring alias uniqueness if provided
    pub fn add_node(nodes: &mut HashMap<String, NodeInfo>, ip: IpAddr, port: u16, alias: Option<String>) -> Result<(), String> {
        if let Some(ref a) = alias {
            if !Self::is_alias_unique(nodes, a) {
                return Err("Alias already exists".to_string());
            }
        }
        let node = NodeInfo::new(ip, port, alias);
        let uuid = format!("{}:{}", ip, port); // UUID based on IP and port
        nodes.insert(uuid, node);
        Ok(())
    }

    // Update node alias, ensuring uniqueness
    pub fn update_node(nodes: &mut HashMap<String, NodeInfo>, uuid: &str, alias: Option<String>) -> Result<(), String> {
        if let Some(ref a) = alias {
            if !Self::is_alias_unique(nodes, a) {
                return Err("Alias already exists".to_string());
            }
        }
        if let Some(node) = nodes.get_mut(uuid) {
            node.alias = alias;
            return Ok(());
        }
        Err("UUID not found".to_string())
    }

    // Get node information by alias or UUID
    pub fn get_node_info(nodes: &HashMap<String, NodeInfo>, identifier: &str) -> Option<&NodeInfo> {
        nodes.values().find(|node| node.alias.as_ref() == Some(identifier))
            .or_else(|| nodes.get(identifier))
    }
}
