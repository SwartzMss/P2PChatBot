use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::commands;

pub async fn run_terminal() -> io::Result<()> {
    let stdin = io::stdin();  // 获取异步的标准输入流
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if line.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        process_command(&line).await;
    }

    Ok(())
}

async fn process_command(input: &str) {
    let args: Vec<&str> = input.trim().split_whitespace().collect();
    match args.first() {
        Some(&"HelloWorld") => commands::hello_world(),
        Some(&"SendMsg") if args.len() > 1 => commands::send_msg(&args[1..].join(" ")),
        Some(&"CheckMsgSend") if args.len() > 2 => commands::check_msg_send(args[1], args[2]),
        _ => println!("Invalid command or insufficient arguments."),
    }
}
