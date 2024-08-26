use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    ExecutableCommand,
    terminal::{enable_raw_mode, disable_raw_mode},
};
use tokio::io::{self, AsyncWriteExt, Stdout};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::commands::{hello_world, send_msg, check_msg_send};

pub async fn run_terminal() -> crossterm::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.write_all(b"Please enter commands:\n").await?;
    stdout.flush().await?;

    let input_buffer = Arc::new(Mutex::new(String::new()));

    loop {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => {
                    println!("Exiting...");
                    break;
                }
                KeyCode::Char(c) => {
                    let mut input = input_buffer.lock().await;
                    input.push(c);
                    stdout.write_all(&[c as u8]).await?;
                }
                KeyCode::Enter => {
                    let input = input_buffer.lock().await.clone();
                    process_command(&input).await;
                    input_buffer.lock().await.clear();
                    stdout.write_all(b"\nPlease enter commands:\n").await?;
                }
                KeyCode::Backspace => {
                    let mut input = input_buffer.lock().await;
                    if !input.is_empty() {
                        input.pop();
                    }
                }
                _ => {}
            }
            stdout.flush().await?;
        }
    }

    disable_raw_mode()?;
    Ok(())
}

async fn process_command(input: &str) {
    let args: Vec<&str> = input.trim().split_whitespace().collect();
    match args.first() {
        Some(&"HelloWorld") => hello_world(),
        Some(&"SendMsg") if args.len() > 1 => send_msg(&args[1..].join(" ")),
        Some(&"CheckMsgSend") if args.len() > 2 => check_msg_send(args[1], args[2]),
        _ => println!("Invalid command or insufficient arguments."),
    }
}
