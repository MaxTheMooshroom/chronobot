use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ArgMatches};
use tokio::runtime::Runtime;

use crate::{bot, chrono, env};

// (variable name, requirement reason)
const REQUIRED_ENV_VARS: [(&str, &str); 1] = [
    ("DISCORD_AUTH_TOKEN", "Connecting to discord"),
];

async fn init_bot(mut bot: bot::BotState) -> bot::BotState {
    let cmdset = bot::CommandSet::new("/")
        .add_command("roll", chrono::roll as bot::CommandAsync);

    // bot.add_command_set(make_command_set_from("/", ))
    bot.add_command_set(cmdset).await;

    bot
}

async fn discord_bot_main_async(auth: impl AsRef<str>) -> Result<!> {
    let bot = init_bot(bot::BotState::new().await).await;
    bot.run(auth).await;

    futures::future::pending::<!>().await;
}

pub fn discord_bot_main() -> Result<!> {
    if env::load().is_err() {
        return Err(anyhow!(concat!(
            "Failed to read `.env` file.\n",
            "The `.env` file is used to get the auth token for the Discord API.\n",
            "\n",
            "Aborting...",
        )));
    }

    let auth: String = {
        let read_guard = env::read()?;

        for (var, reason) in REQUIRED_ENV_VARS {
            if !read_guard.contains_key(var) {
                return Err(anyhow!("Missing environment variable '{}'; Reason needed: {}", var, reason));
            }
        }

        read_guard.get("DISCORD_AUTH_TOKEN").unwrap().clone()
    };

    Runtime::new()?.block_on(discord_bot_main_async(auth));

    unreachable!()
}

