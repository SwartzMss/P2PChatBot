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
use std::net::Ipv4Addr;
mod udp_connection;
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

    let socket = Arc::new(Mutex::new(socket));

    // 监听任务
    let listen_socket = Arc::clone(&socket);
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        udp_connection::start_listening(listen_socket,tx).await.expect("Failed to listen");
    });

    println!("node_name = {}, communication_ip= {}, communication_port = {}", node_name, communication_ip, communication_port);
    let (notify_tx, mut notify_rx) = mpsc::channel(100);
    
    let communication_ip_addr: Ipv4Addr = communication_ip.parse().map_err(|_e| {
        tokio::io::Error::new(tokio::io::ErrorKind::InvalidInput, "Invalid IP address")
    })?;


    let node_manager = Arc::new(Mutex::new(NodeManager::new(communication_ip_addr, communication_port,node_name.clone())));
    let command_handler = Arc::new(CommandHandler::new(node_manager.clone()));
    let node_manager_bak = Arc::clone(&node_manager);
    tokio::spawn(async move {
        while let Some(message_data) = rx.recv().await {
            let node_manager = node_manager_bak.clone();
            // 处理每条消息时锁定 node_manager
            node_manager.lock().await.process_message(message_data).await;
        }
    });
    let monitor_handle = tokio::spawn(multicast_discovery::network_monitor(notify_tx, node_manager.clone(), node_name.clone()));
    let sender_handle = tokio::spawn(multicast_discovery::multicast_sender(multicast_addr, communication_ip, communication_port, node_name.clone()));

    println!("Ready to accept commands. Type 'exit' to quit.");



    tokio::spawn(async move {
        while let Some(notification) = notify_rx.recv().await {
            println!("Notification: {}", notification);
        }
    });

    let terminal_handle = tokio::spawn(async move {
        if let Err(e) = terminal::run_terminal(command_handler).await {
            error!("Terminal Error: {:?}", e);
        }
    });

    if let Err(e) = terminal_handle.await {
        error!("terminal handle failed: {:?}", e);
    }
    if let Err(e) = monitor_handle.await {
        error!("Network monitor failed: {:?}", e);
    }
    if let Err(e) = sender_handle.await {
        error!("Multicast sender failed: {:?}", e);
    }

    Ok(())
}
