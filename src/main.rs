mod commands;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // install global tracing collector, this is configured using the RUST_LOG env var
    tracing_subscriber::fmt::init();

    let bot = Bot::from_env();
}
