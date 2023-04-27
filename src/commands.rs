use teloxide::{prelude::*, types::ParseMode, utils::command::BotCommands};

use crate::run;

/// Each variant of this struct represents a single command that the bot supports.
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

    #[command(description = "Run code that is sent to the bot in backticks ``")]
    Run { language: String },
}

/// Handles incoming messages and matches on the Command variant to perform the
/// appropriate action.
pub(crate) async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome! I am a bot that takes code and executes it. To get started, please view the /help").await?;
        }
        Command::Run { language } => {
            let input = msg.text();
            if input.is_none() {
                bot.send_message(msg.chat.id, "Your message contains no code to run!")
                    .await?;
                return Ok(());
            }

            let user_from = msg.from().unwrap();
            let user_mention = mention(user_from.id.0, user_from.full_name());

            match run::run(&msg, language).await {
                Ok(output) => {
                    bot.send_message(
                        msg.chat.id,
                        format!(
                            "Here's your output {}!\n<code>{}</code>",
                            user_mention, output
                        ),
                    )
                    .parse_mode(ParseMode::Html)
                    .await?;
                }
                Err(err) => {
                    bot.send_message(msg.chat.id, format!("Failed to execute code: {}", err))
                        .await?;
                }
            }
        }
    };

    Ok(())
}

fn mention(id: u64, name: String) -> String {
    format!(r#"<a href="tg://user?id={id}">{name}</a>"#)
}
