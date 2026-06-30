use anyhow::Result;
use clap::Parser;

use std::borrow::Borrow;
use std::ffi::OsString;

use crate::execute::{Execute, ExecuteReturn};

#[cfg(feature = "async")]
use crate::execute::ExecuteAsync;

pub use harmony_autonomy_derive::CommandSet;

pub trait CommandSet: Parser + Execute {
    /// The prefix to use for detecting commands. Defaults to `/`.
    ///
    /// Eg. <br />
    /// `/echo "The quick brown fox jumps over the lazy dog"`
    const PREFIX: &'static str = "/";

    fn execute_from_raw_unchecked(raw_input: impl Borrow<str>) -> ExecuteReturn {
        let mut command: &str = raw_input.borrow();

        if command.starts_with(Self::PREFIX) {
            command = &command[Self::PREFIX.len()..];
        }

        Self::parse_from(command.split_whitespace()).execute()
    }

    fn try_execute_from_raw_unchecked(command: impl Borrow<str>) -> Result<ExecuteReturn> {
        let mut command: &str = command.borrow();

        if command.starts_with(Self::PREFIX) {
            command = &command[Self::PREFIX.len()..];
        }

        let this: Self = Parser::try_parse_from(command.split_whitespace())?;

        Ok(this.execute())
    }

    fn execute_from_raw(raw_input: impl Borrow<str>) -> ExecuteReturn {
        Self::parse_from(raw_input.borrow().split_whitespace()).execute()
    }

    fn try_execute_from_raw(raw_input: impl Borrow<str>) -> Result<ExecuteReturn> {
        let result: Result<Self> =
            Self::try_parse_from(raw_input.borrow().split_whitespace())
            .map_err(Into::into);

        Ok(result?.execute())
    }

    fn execute_from<I, T>(args: I) -> ExecuteReturn
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args).execute()
    }

    fn try_execute_from<I, T>(args: I) -> Result<ExecuteReturn>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let result: Result<Self> =
            Self::try_parse_from(args).map_err(Into::into);

        Ok(result?.execute())
    }

    fn execute() -> ExecuteReturn {
        Self::parse().execute()
    }

    fn try_execute() -> Result<ExecuteReturn> {
        let result: Result<Self> =
            Self::try_parse().map_err(Into::into);

        Ok(result?.execute())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
pub trait CommandSetAsync: Parser + ExecuteAsync {
    /// The prefix to use for detecting commands. Defaults to `/`.
    ///
    /// Eg. <br />
    /// `/echo "The quick brown fox jumps over the lazy dog"`
    const PREFIX: &'static str = "/";

    async fn execute_from_raw_unchecked(raw_input: impl Borrow<str> + Send) -> ExecuteReturn {
        let mut command: &str = raw_input.borrow();

        if command.starts_with(Self::PREFIX) {
            command = &command[Self::PREFIX.len()..];
        }

        Self::parse_from(command.split_whitespace()).execute().await
    }

    async fn try_execute_from_raw_unchecked(command: impl Borrow<str> + Send) -> Result<ExecuteReturn> {
        let mut command: &str = command.borrow();

        if command.starts_with(Self::PREFIX) {
            command = &command[Self::PREFIX.len()..];
        }

        let this: Self = Parser::try_parse_from(command.split_whitespace())?;

        Ok(this.execute().await)
    }

    async fn execute_from_raw(raw_input: impl Borrow<str> + Send) -> ExecuteReturn {
        Self::parse_from(raw_input.borrow().split_whitespace()).execute().await
    }

    async fn try_execute_from_raw(raw_input: impl Borrow<str> + Send) -> Result<ExecuteReturn> {
        let result: Result<Self> =
            Self::try_parse_from(raw_input.borrow().split_whitespace())
            .map_err(Into::into);

        Ok(result?.execute().await)
    }

    async fn execute_from<I, T>(args: I) -> ExecuteReturn
    where
        I: IntoIterator<Item = T> + Send,
        // T doesn't need `Send` because clap consumes all of the arguments
        // before returning the `Self` value, with no held references to the
        // args. As such, there are no values held across thread boundaries,
        // and no need for `Send`.
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args).execute().await
    }

    async fn try_execute_from<I, T>(args: I) -> Result<ExecuteReturn>
    where
        I: IntoIterator<Item = T> + Send,
        // T doesn't need `Send` because clap consumes all of the arguments
        // before returning the `Self` value, with no held references to the
        // args. As such, there are no values held across thread boundaries,
        // and no need for `Send`.
        T: Into<OsString> + Clone,
    {
        let result: Result<Self> =
            Self::try_parse_from(args).map_err(Into::into);

        Ok(result?.execute().await)
    }

    async fn execute() -> ExecuteReturn {
        Self::parse().execute().await
    }

    async fn try_execute() -> Result<ExecuteReturn> {
        let result: Result<Self> = Self::try_parse().map_err(Into::into);

        Ok(result?.execute().await)
    }
}
