use tokio::io::{self, AsyncBufReadExt, BufReader};
mod cli;
mod commands;

use cli::Cli;
use clap::Parser;

#[tokio::main]
async fn main() {

    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    let cmd_handle = tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            if line.trim().eq_ignore_ascii_case("exit") {
                break;
            }
            println!("msg received {}", line);
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
                        cli::Commands::HelloWorld => {
                            commands::hello_world().await;
                        }
                        cli::Commands::SendMsg { message } => {
                            commands::send_message(&message).await;
                        }
                    }
                },
                Err(e) => {
                    println!("Invalid command: {}", e);
                }
            }
        }
        println!("Exiting command input loop.");
    });

    if let Err(e) = cmd_handle.await {
        println!("Command input handler failed: {:?}", e);
    }
}
