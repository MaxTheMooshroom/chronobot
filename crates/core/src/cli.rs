
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
/// A program for running games of chronomutants in various settings.
/// The default setting is Discord, as a bot.
///
/// Commissioned by Gary.
pub struct Cli {}

impl Cli {
    pub fn parse() -> Self { Parser::parse() }
}

