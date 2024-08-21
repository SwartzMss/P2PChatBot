use tokio::io::{self, AsyncBufReadExt};
use clap::Parser;
use shell_words;
use log::{info, error};
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

mod cli;
mod commands;
mod multicast_discovery;
mod node_manager;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    env::set_current_dir(&exe_dir).expect("Failed to set current directory");

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Application is starting up...");

    let multicast_addr = "239.255.255.250:1900";
    let nodes = Arc::new(Mutex::new(HashMap::new()));
    let (notify_tx, mut notify_rx) = mpsc::channel(100);

    let monitor_handle = tokio::spawn(multicast_discovery::network_monitor(multicast_addr, notify_tx, nodes.clone()));
    let message_to_send = String::from("Hello, multicast network!");
    let sender_handle = tokio::spawn(multicast_discovery::multicast_send(multicast_addr, message_to_send));

    // 启动命令行输入处理循环
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();

    println!("Ready to accept commands. Type 'exit' to quit.");

    let cmd_handle = tokio::spawn(async move {
        while let Some(Ok(line)) = reader.next_line().await {
            if line.trim().eq_ignore_ascii_case("exit") {
                break;
            }

            let args = match shell_words::split(&line) {
                Ok(args) => args,
                Err(e) => {
                    println!("Error parsing input: {}", e);
                    continue;
                }
            };

            let cli = Cli::try_parse_from(args);
            match cli {
                Ok(cli) => {
                    match cli.command {
                        Commands::List => {
                            commands::list_users().await;
                        },
                        Commands::Send { identifier, message } => {
                            commands::send_message(&identifier, &message).await;
                        },
                        Commands::Update { uuid, alias } => {
                            commands::update_alias(&uuid, &alias).await;
                        },
                    }
                },
                Err(e) => {
                    println!("Invalid command: {}", e);
                }
            }
        }
        println!("Exiting command input loop.");
    });

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
    if let Err(e) = cmd_handle.await {
        error!("Command input handler failed: {:?}", e);
    }
}
