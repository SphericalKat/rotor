use dotenv::dotenv;
use teloxide::prelude::*;


#[tokio::main]
async fn main() {
    run().await;
}
async fn run() {
    dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    }).await;
}
