#![feature(never_type)]
#![allow(unused)]

pub mod bot;
pub mod chrono;
pub mod cli;
pub mod commands;
pub mod consts;
pub mod env;
pub mod log;
pub mod tables;
pub mod util;

use anyhow::{anyhow, Result};
use clap::Parser;

use commands::roll::SubcommandRoll;

use harmony_autonomy::CommandSet;

/// Performs rolls of predefined groups of dice.
///
/// Use `roll <group> --help` for more information.
#[derive(Parser, Debug)]
pub struct CommandRoll {
    #[command(subcommand)]
    pub roll: SubcommandRoll,
}

#[derive(Parser)]
#[command(version, about)]
pub enum Chronobot {
    /// A one-off command for Chronobot to roll a given dice-pool.
    Roll {
        #[command(subcommand)]
        inner: SubcommandRoll,
    },

    /// Run Chronobot as a discord bot. This requires a discord
    /// auth token `DISCORD_AUTH_TOKEN` in the .env file, wherever
    /// this executable is located.
    DiscordBot,
}

fn roll_main(roll: SubcommandRoll) -> Result<!> {
    roll.do_rolls()?;

    std::process::exit(0)
}

impl Chronobot {
    pub fn parse() -> Self { Parser::parse() }

    pub fn execute(self) -> Result<!> {
        use Chronobot::*;

        match self {
            Roll{ inner } => roll_main(inner),
            DiscordBot => commands::bot::discord_bot_main(),
        }
    }
}

