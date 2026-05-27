pub(crate) mod bot_command;
pub(crate) mod roll;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub enum Commands {
    /// Roll one of the [`roll::CommandRoll`] types.
    Roll {
        #[command(subcommand)]
        inner: roll::CommandRoll,
    },

    DiscordBot,
}

impl Commands {
    pub fn parse() -> Self { Parser::parse() }

    pub async fn execute(&self) -> anyhow::Result<()> {
        use Commands::*;

        match self {
            Roll{ .. } => {},
            DiscordBot => {

            },
        }
        Ok(())
    }
}

