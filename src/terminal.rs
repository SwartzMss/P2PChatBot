use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::commands::CommandHandler;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub async fn run_terminal(command_handler: Arc<CommandHandler>) -> io::Result<()> {
    let stdin = io::stdin();  // 获取异步的标准输入流
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    println!("run_terminal started");
    while let Some(line) = lines.next_line().await? {
        if line.trim().eq_ignore_ascii_case("exit") {
            std::process::exit(0);
        }
        let command_future = process_command(&line,  command_handler.clone()).await;
        command_future.await;
    }

    Ok(())
}

async fn process_command(input: &str, command_handler: Arc<CommandHandler>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        // 如果输入为空或者只有空白字符，则不执行任何操作
        return Box::pin(async {});
    }

    let args: Vec<&str> = trimmed_input.split_whitespace().collect();
    match args.first() {
        Some(&"list_users") => {
            let handler = Arc::clone(&command_handler);
            Box::pin(async move {
                handler.list_users().await;
            })
        },
        Some(&"send_message") if args.len() > 2 => {
            let handler = Arc::clone(&command_handler);
            let identifier = args[1].to_string();
            let message = args[2].to_string();
            Box::pin(async move {
                handler.send_message(&identifier, &message).await;
            })
        },
        Some(&"update_alias") if args.len() > 2 => {
            let handler = Arc::clone(&command_handler);
            let uuid = args[1].to_string();
            let alias = args[2].to_string();
            Box::pin(async move {
                handler.update_alias(&uuid, &alias).await;
            })
        },
        _ => Box::pin(async {
            println!("Invalid command or insufficient arguments.");
        }),
    }
}


