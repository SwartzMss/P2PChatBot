use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "Network Application", version = "1.0", author = "Your Name", about = "Manages network sessions and interactions")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Lists all users
    List,
    /// Sends a message to a user
    Send {
        /// UUID or alias of the user
        identifier: String,
        /// Message to send to the user
        message: String,
    },
    /// Updates a user's alias
    Update {
        /// UUID of the user
        uuid: String,
        /// New alias for the user
        alias: String,
    },
}
