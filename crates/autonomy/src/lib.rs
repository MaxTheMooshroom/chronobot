#![feature(associated_type_defaults)]
#![feature(never_type)]

mod bot;
mod cmdset;

pub mod runtime;
pub mod util;

pub use cmdset::{CommandContext, CommandSet};

// #[cfg(feature = "async")]
// pub use cmdset::CommandDelegateAsync;

