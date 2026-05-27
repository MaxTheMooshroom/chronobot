#![allow(unused)]
//! TODO:

pub(crate) mod bot;
pub(crate) mod chrono;
pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod consts;
pub(crate) mod env;
pub(crate) mod log;
pub(crate) mod tables;
pub(crate) mod util;

use serenity::all::{Context, Message};

use std::sync::Arc;

use cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::init()?;

    commands::Commands::parse().execute().await;

    Ok(())
}
