pub(crate) mod bot;
pub(crate) mod roll;

use anyhow::{anyhow, Result};
use clap::Parser;

pub trait CommandSet: Parser {
    fn try_execute<S: AsRef<str>>(&self, s: S) -> Result<()>;
}

