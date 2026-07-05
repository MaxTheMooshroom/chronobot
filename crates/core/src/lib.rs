#![feature(never_type)]
#![allow(unused)]

pub mod bot;
pub mod chrono;
pub mod commands;
pub mod consts;
pub mod env;
pub mod log;
pub mod tables;
pub mod util;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use commands::roll::SubcommandRoll;

use h_autonomy::{CommandContext, CommandSet};

/// Performs rolls of predefined groups of dice.
///
/// Use `roll <group> --help` for more information.
#[derive(Parser, Debug)]
pub struct CommandRoll {
    #[command(subcommand)]
    pub roll: SubcommandRoll,
}

#[derive(Subcommand, Debug)]
#[non_exhaustive]
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

/// A program for running games of chronomutants in various settings.
/// The default setting is Discord, as a bot.
///
/// Commissioned by Gary.
#[derive(Parser, Debug)]
#[command(version, about)]
#[non_exhaustive]
pub struct ChronobotArgs {
    #[command(subcommand)]
    cmd: Chronobot,
}

// fn roll_main(roll: SubcommandRoll) -> Result<!> {
//     roll.do_rolls()?;
//
//     std::process::exit(0)
// }

impl CommandSet for Chronobot {
    type ExtraContext = ();
    type ReturnType = !;

    fn dispatch(self, ec: Self::ExtraContext) -> Result<Self::ReturnType> {
        match self {
            // Chronobot::Roll { inner } => roll_main(inner),
            Chronobot::Roll { inner } => {
                inner.do_rolls()?;

                Ok(std::process::exit(0))
            },
            Chronobot::DiscordBot => commands::bot::discord_bot_main(),
        }
    }
}

impl CommandContext for ChronobotArgs {
    type Commands = Chronobot;

    fn commands(self) -> Self::Commands {
        self.cmd
    }
}

impl ChronobotArgs {
    /// A convenience wrapper around [`ChronobotArgs`]' implementation of
    /// [`CommandContext::execute`].
    pub fn execute() -> Result<!> {
        <Self as CommandContext>::execute(())
    }
}

