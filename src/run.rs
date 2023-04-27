use anyhow::{anyhow, bail};
use piston_rs::{Client, Executor};
use teloxide::{prelude::*, types::MessageEntityKind};
use tracing::info;

pub(crate) async fn run(msg: &Message, lang: String) -> anyhow::Result<String> {
    let entities = msg
        .parse_entities()
        .ok_or(anyhow!("Message contains no code!"))?;

    for entity in entities {
        match entity.kind() {
            MessageEntityKind::Code | MessageEntityKind::Pre { .. } => {
                // extract language user wants us to use
                let language = lang
                    .split_ascii_whitespace()
                    .nth(0)
                    .ok_or(anyhow!("Invalid language selected."))?;

                let client = Client::new();
                let executor = Executor::new()
                    .set_language(language)
                    .set_version("*");

                // execute code
                info!("Got code: {}", entity.text());
                return Ok("".to_string());
            }
            _ => continue,
        }
    }

    bail!("Message contains no code!");
}
