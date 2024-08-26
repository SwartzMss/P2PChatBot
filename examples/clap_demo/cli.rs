use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Clap Demo", version = "1.0", author = "SwartzMss", about = "Demo Usage")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    HelloWorld,
    /// Sends a message to a user
    SendMsg {
        /// Message to send to the user
        message: String,
    },

}
