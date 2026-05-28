use clap::{Parser, Subcommand, ArgMatches};

use crate::{bot, chrono, env};

// (variable name, requirement reason)
const REQUIRED_ENV_VARS: [(&str, &str); 1] = [
    ("DISCORD_AUTH_TOKEN", "Connecting to discord"),
];

async fn init_bot(mut bot: bot::BotState) -> bot::BotState {
    let cmdset = bot::CommandSet::new("/")
        .add_command("roll", chrono::roll as bot::CommandAsync);

    // bot.add_command_set(make_command_set_from("/", ))
    // bot.add_command_set(cmdset).await;

    bot
}

async fn discord_bot_main() -> anyhow::Result<()> {
    if env::load().is_err() {
        println!("Failed to read `.env` file.");
        println!("This is required to connect to discord.");
        println!("\nAborting...");

        return Ok(());
    }

    let auth: String = {
        let read_guard = env::read()?;

        for (var, reason) in REQUIRED_ENV_VARS {
            if !read_guard.contains_key(var) {
                println!("Missing environment variable '{}'; Reason needed: {}", var, reason);
                return Ok(());
            }
        }

        read_guard.get("DISCORD_AUTH_TOKEN").unwrap().clone()
    };

    let mut bot = init_bot(bot::BotState::new(auth).await).await;
    bot.run().await;

    futures::future::pending::<()>().await;

    unreachable!()
}

