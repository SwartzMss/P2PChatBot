// commands.rs
use crate::node_manager::NodeManager;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CommandHandler {
    node_manager: Arc<NodeManager>,
}

impl CommandHandler {
    pub fn new(node_manager: Arc<NodeManager>) -> Self {
        CommandHandler { node_manager }
    }

    // List all users
    pub async fn list_users(&self) {
        let nodes = self.node_manager.nodes.lock().await;
        for (uuid, node) in nodes.iter() {
            println!("UUID: {}, IP: {}, Port: {}, Alias: {:?}", uuid, node.ip, node.port, node.alias);
        }
    }

    // Send a message to a user
    pub async fn send_message(&self, identifier: &str, message: &str) {
        // Here, we would typically look up the node and send a message, for now we just print
        println!("Message '{}' sent to {}", message, identifier);
        // 实际发送逻辑需要实现网络通讯部分
    }

    // Update a user's alias
    pub async fn update_alias(&self, uuid: &str, alias: &str) {
        match self.node_manager.update_node_alias(uuid, alias.to_string()).await {
            Ok(_) => println!("Alias updated for UUID {}: {}", uuid, alias),
            Err(e) => println!("Failed to update alias for UUID {}: {}", uuid, e),
        }
    }
}
