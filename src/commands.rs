use teloxide::utils::command;

use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "The following commands are supported:"
)]
pub(crate) enum Command {
    #[command(description = "display this text.")]
    Help,

    #[command(description = "Start the bot.")]
    Start,
}

pub(crate) async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome! I am a bot that takes code and executes it. To get started, please view the /help").await?
        },
    };

    Ok(())
}
