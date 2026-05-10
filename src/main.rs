#![allow(unused)]
//! TODO:

mod bot;
mod chrono;
mod cli;
mod env;
mod log;

use serenity::all::{Context, Message};

use std::sync::Arc;

use cli::Cli;

// (variable name, requirement reason)
const REQUIRED_ENV_VARS: [(&str, &str); 1] = [
    ("DISCORD_AUTH_TOKEN", "Connecting to discord"),
];

fn test(state: bot::BotState, ctx: Arc<bot::CommandContext>) -> bot::CommandFuture<()> {
    Box::pin(async move {
        state.info("test").await;
        ctx.msg.channel_id.say(&ctx.ctx, "test").await.unwrap();
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = Cli::parse();

    log::init()?;

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

    // println!("{:#?}", chrono::dice::roll_dice(&vec![("Future Tech".into(), 17)]));

    let cmdset = bot::CommandSet::new("/")
        .add_command("test", test)
        .add_command("roll", chrono::roll);

    let mut bot = bot::BotState::new(auth).await;
    bot.add_command_set(cmdset).await;
    bot.run().await;

    futures::future::pending::<()>().await;

    unreachable!()
}
