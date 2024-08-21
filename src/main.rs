use log::{info};
use std::env;
mod multicast_discovery;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
mod multicast_discovery;
mod node_manager;

fn main() {
    // 获取可执行文件的路径
    let exe_path = env::current_exe().expect("Failed to get current executable path");

    // 获取可执行文件的父目录
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    // 设置当前工作目录为可执行文件的父目录
    env::set_current_dir(&exe_dir).expect("Failed to set current directory");
    
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Application is starting up...");
    
    let multicast_addr = "239.255.255.250:1900";
    let nodes = Arc::new(Mutex::new(node_manager::HashMap::new()));
    let (notify_tx, mut notify_rx) = mpsc::channel(100);

    let monitor_handle = tokio::spawn(multicast_discovery::network_monitor(multicast_addr, notify_tx, nodes.clone()));
    
    let message_to_send = String::from("Hello, multicast network!");
    let sender_handle = tokio::spawn(multicast_discovery::multicast_send(multicast_addr, message_to_send));
    
    tokio::spawn(async move {
        while let Some(notification) = notify_rx.recv().await {
            println!("Notification: {}", notification);
        }
    });

    if let Err(e) = monitor_handle.await {
        eprintln!("Network monitor failed: {:?}", e);
    }
    if let Err(e) = sender_handle.await {
        eprintln!("Multicast sender failed: {:?}", e);
    }
}
