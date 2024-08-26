use tokio::io::{self, AsyncBufReadExt, BufReader};
use clap::Parser;
use shell_words;
use log::{info, error};
use std::env;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
mod node_manager;
mod cli;
mod commands;
mod multicast_discovery;

use node_manager::NodeManager;
use commands::CommandHandler;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    env::set_current_dir(&exe_dir).expect("Failed to set current directory");

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Application is starting up...");

    let multicast_addr = "239.255.255.250:1900";
    let communication_ip = "192.168.3.196";
    let communication_port:u16 = 3699;
    let nodes = Arc::new(Mutex::new(HashMap::new()));
    let (notify_tx, mut notify_rx) = mpsc::channel(100);

    let monitor_handle = tokio::spawn(multicast_discovery::network_monitor(multicast_addr, notify_tx, nodes.clone()));
    let sender_handle = tokio::spawn(multicast_discovery::multicast_sender(multicast_addr, communication_ip, communication_port));

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    println!("Ready to accept commands. Type 'exit' to quit.");

    let node_manager = Arc::new(NodeManager::new());


    let command_handler = CommandHandler::new(node_manager.clone());

    let cmd_handle = tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
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


            let cli = Cli::try_parse_from(&args);
            match cli {
                Ok(cli) => {
                    match cli.command {
                        Commands::List => {
                            command_handler.list_users().await;
                        },
                        Commands::Send { identifier, message } => {
                            // commands::send_message(&identifier, &message).await;
                        },
                        Commands::Update { uuid, alias } => {
                            // commands::update_alias(&uuid, &alias).await;
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
