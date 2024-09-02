// commands.rs
use crate::node_manager::NodeManager;
use std::sync::Arc;
use tokio::sync::Mutex;
pub struct CommandHandler {
    node_manager: Arc<Mutex<NodeManager>>,
}

impl CommandHandler {
    pub fn new(node_manager:  Arc<Mutex<NodeManager>>) -> Self {
        CommandHandler { node_manager }
    }

    // List all users
    pub async fn list_users(&self) {
        println!("list_users function entered.");
        let manager = self.node_manager.lock().await; // 获取 Mutex 的锁
        let node_list = manager.list_users().await; // 调用 NodeManager 的方法
        for node in node_list {
            println!("{}", node);
        }
    }

    // Send a message to a user
    pub async fn send_message(&self, identifier: &str, message: &str) {
        println!("send_message function entered.");
        let manager = self.node_manager.lock().await; // 获取 Mutex 的锁
        if let Err(e) = manager.send_message(identifier, message).await {
            println!("Failed to send message: {}", e);
        }
    }

    // Update a user's alias
    pub async fn update_alias(&self, uuid: &str, alias: &str) {
        let manager = self.node_manager.lock().await; // 获取 Mutex 的锁
        match manager.update_node_alias(uuid, alias.to_string()).await {
            Ok(_) => println!("Alias updated for UUID {}: {}", uuid, alias),
            Err(e) => println!("Failed to update alias for UUID {}: {}", uuid, e),
        }
    }
}
