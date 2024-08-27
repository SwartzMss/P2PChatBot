mod cli;
mod commands;

use cli::Cli;
use clap::Parser;

fn main() {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Execute the command synchronously
    match cli.command {
        cli::Commands::HelloWorld => {
            commands::hello_world();
        }
        cli::Commands::SendMsg { message } => {
            commands::send_message(&message);
        }
    }
}