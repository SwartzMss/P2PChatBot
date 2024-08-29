use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::net::UdpSocket;
use shell_words;
use log::{info, error};
use std::env;
use std::net::SocketAddr;
mod terminal;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;
mod node_manager;
mod commands;
mod multicast_discovery;

use node_manager::NodeManager;
use commands::CommandHandler;


#[tokio::main]
async fn main()  -> tokio::io::Result<()> {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    env::set_current_dir(&exe_dir).expect("Failed to set current directory");

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Application is starting up...");

    let node_name = Uuid::new_v4().to_string();
    let multicast_addr = "239.255.255.250:3000";
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let local_addr = socket.local_addr()?;
    let communication_ip = local_addr.ip().to_string();
    let communication_port:u16 =  local_addr.port();
    println!("node_name = {}, communication_ip= {}, communication_port = {}", node_name, communication_ip, communication_port);
    let nodes = Arc::new(Mutex::new(HashMap::new()));
    let (notify_tx, mut notify_rx) = mpsc::channel(100);

    let monitor_handle = tokio::spawn(multicast_discovery::network_monitor(notify_tx, nodes.clone(), node_name.clone()));
    let sender_handle = tokio::spawn(multicast_discovery::multicast_sender(multicast_addr, communication_ip, communication_port, node_name.clone()));

    println!("Ready to accept commands. Type 'exit' to quit.");

    let node_manager = Arc::new(NodeManager::new());
    let command_handler = Arc::new(CommandHandler::new(node_manager.clone()));

    terminal::run_terminal(command_handler).await;
    tokio::spawn(async move {
        while let Some(notification) = notify_rx.recv().await {
            println!("Notification: {}", notification);
        }
    });

    if let Err(e) = monitor_handle.await {
        error!("Network monitor failed: {:?}", e);
    }
    if let Err(e) = sender_handle.await {
        error!("Multicast sender failed: {:?}", e);
    }

    Ok(())
}
