mod commands;
mod terminal;

#[tokio::main]
async fn main() {
    if let Err(e) = terminal::run_terminal().await {
        eprintln!("Error occurred: {}", e);
    }
}
