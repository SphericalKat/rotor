mod commands;

use anyhow::Ok;
use commands::{Command, answer};
use teloxide::prelude::*;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // install global tracing collector, this is configured using the RUST_LOG env var
    tracing_subscriber::fmt::init();
    info!("Initialized tracing");

    // load env variables from .env file, but ignore error if it doesn't exist
    dotenv::dotenv().ok();
    info!("Loaded env variables");

    let bot = Bot::from_env();
    info!("Bot is ready.");

    Command::repl(bot, answer).await;

    info!("Shutdown complete.");

    Ok(())
}
