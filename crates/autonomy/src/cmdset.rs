#![allow(unused)]

use anyhow::Result;
use clap::{Parser, Subcommand};
// use enum_trait::{Enum, NotEnum};

use std::borrow::Borrow;
use std::ffi::OsString;
use std::future::Future;

// pub use h_autonomy_derive::{CommandContext, CommandSet};

pub trait CommandDelegate {
    type ExtraContext = ();
    type ParameterTypes = ();
    type ReturnType = ();

    fn call(&self, ec: Self::ExtraContext, args: Self::ParameterTypes) -> Result<Self::ReturnType>;
}

// #[cfg(feature = "async")]
// #[async_trait::async_trait]
// pub trait CommandDelegateAsync {
//     type ExtraContext = ();
//     type ParameterTypes = ();
//     type ReturnType = ();
//
//     async fn execute(&self) -> Self::ReturnType;
// }

/// The various subcommands of a [`CommandContext`].
///
/// This trait is only applicable to enum types that derive the
/// [`Enum`] trait (eg. `#[derive(enum_trait::Enum, CommandSet)]`),
/// and that implement the [`Subcommand`] trait, whether by deriving it
/// or by manual implementation.
///
/// Each subcommand must have a delegate that this trait can dispatch to
/// for executing it. This is done with the `#[dispatch()]` attribute, where
/// the argument is the name or path to a function that will receive the
/// current [`CommandContext`].
///
/// If no `delegate` attribute is used for an arm, it is assumed that the first
/// unnamed argument is a type that implements
/// [`CommandDelegate`] (or [`CommandDelegateAsync`]) with respective
/// associated type bounds.
///
/// For example:
/// ```
/// # use clap::{Parser, Subcommand};
/// # // use enum_trait::{Enum, NotEnum};
/// # use h_autonomy::cmdset::{CommandContext, CommandDelegate, CommandSet};
/// #[derive(Subcommand, CommandSet)]
/// #[cmdset(returns = )]
/// enum ExampleEnum {
///     #[delegate(func_a)]
///     A,
///     B(),
/// }
/// ```
// pub trait CommandSet: Subcommand + Enum {
pub trait CommandSet: Subcommand {
    type ExtraContext = ();
    type ReturnType = ();

    // match self {
    //     A(a, b) => delegate_a(ec, args),
    //     ...
    // }
    fn dispatch(self, ec: Self::ExtraContext) -> Result<Self::ReturnType>;

    // #[cfg(feature = "async")]
    // fn dispatch_async(&self) -> Self::ReturnType;
}

// pub trait CommandContext: Parser + NotEnum {
pub trait CommandContext: Parser {
    /// The prefix to use for detecting commands. Defaults to `/`.
    ///
    /// Eg. <br />
    /// `/echo "The quick brown fox jumps over the lazy dog"`
    const PREFIX: &'static str = "/";

    type Commands: CommandSet;

    fn commands(self) -> Self::Commands;

    fn execute_from_raw_unchecked(
        raw_input: impl Borrow<str>,
        ctx: <Self::Commands as CommandSet>::ExtraContext,
    ) -> Result<<Self::Commands as CommandSet>::ReturnType> {
        let raw_input = raw_input.borrow();
        let input = raw_input.strip_prefix(Self::PREFIX).unwrap_or(raw_input);

        Self::parse_from(input.split_whitespace()).commands().dispatch(ctx)
    }

    fn try_execute_from_raw_unchecked(
        raw_input: impl Borrow<str>,
        ctx: <Self::Commands as CommandSet>::ExtraContext,
    ) -> Result<<Self::Commands as CommandSet>::ReturnType> {
        let raw_input = raw_input.borrow();
        let input = raw_input.strip_prefix(Self::PREFIX).unwrap_or(raw_input);

        Self::try_parse_from(input.split_whitespace())?.commands().dispatch(ctx)
    }

    fn execute_from_raw(
        raw_input: impl Borrow<str>,
        ctx: <Self::Commands as CommandSet>::ExtraContext,
    ) -> Result<<Self::Commands as CommandSet>::ReturnType> {
        Self::parse_from(raw_input.borrow().split_whitespace()).commands().dispatch(ctx)
    }

    fn try_execute_from_raw(
        raw_input: impl Borrow<str>,
        ctx: <Self::Commands as CommandSet>::ExtraContext,
    ) -> Result<<Self::Commands as CommandSet>::ReturnType> {
        Self::try_parse_from(raw_input.borrow().split_whitespace())?.commands().dispatch(ctx)
    }

    fn execute_from<I, T>(
        args: I,
        ctx: <Self::Commands as CommandSet>::ExtraContext,
    ) -> Result<<Self::Commands as CommandSet>::ReturnType>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::parse_from(args).commands().dispatch(ctx)
    }

    fn try_execute_from<I, T>(
        args: I,
        ctx: <Self::Commands as CommandSet>::ExtraContext,
    ) -> Result<<Self::Commands as CommandSet>::ReturnType>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        Self::try_parse_from(args)?.commands().dispatch(ctx)
    }

    fn execute(ctx: <Self::Commands as CommandSet>::ExtraContext) -> Result<<Self::Commands as CommandSet>::ReturnType> {
        Self::parse().commands().dispatch(ctx)
    }

    fn try_execute(ctx: <Self::Commands as CommandSet>::ExtraContext) -> Result<<Self::Commands as CommandSet>::ReturnType> {
        Self::try_parse()?.commands().dispatch(ctx)
    }
}

// #[cfg(feature = "async")]
// #[async_trait::async_trait]
// pub trait CommandSetAsync<T>: Parser + CommandSet<ReturnType = Box<dyn Future<Output = T>>> {
//     /// The prefix to use for detecting commands. Defaults to `/`.
//     ///
//     /// Eg. <br />
//     /// `/echo "The quick brown fox jumps over the lazy dog"`
//     const PREFIX: &'static str = "/";
//
//     type Commands: CommandSet;
//
//     fn commands(&self) -> Self::Commands;
//
//     async fn execute_from_raw_unchecked(raw_input: impl Borrow<str> + Send) -> <Self::Commands as CommandSet>::ReturnType {
//         let raw_input = raw_input.borrow();
//         let input = raw_input.strip_prefix(Self::PREFIX).unwrap_or(raw_input);
//
//         Self::parse_from(input.split_whitespace()).commands().dispatch(ctx)
//     }
//
//     async fn try_execute_from_raw_unchecked(raw_input: impl Borrow<str> + Send) -> Result<<Self::Commands as CommandSet>::ReturnType> {
//         let raw_input = raw_input.borrow();
//         let input = raw_input.strip_prefix(Self::PREFIX).unwrap_or(raw_input);
//
//         Ok(Self::try_parse_from(input.split_whitespace())?.commands().dispatch(ctx))
//     }
//
//     async fn execute_from_raw(raw_input: impl Borrow<str> + Send) -> <Self::Commands as CommandSet>::ReturnType {
//         Self::parse_from(raw_input.borrow().split_whitespace()).dispatch(ctx).await
//     }
//
//     async fn try_execute_from_raw(raw_input: impl Borrow<str> + Send) -> Result<<Self::Commands as CommandSet>::ReturnType> {
//         Ok(Self::try_parse_from(raw_input.borrow().split_whitespace())?.dispatch(ctx).await)
//     }
//
//     async fn execute_from<I, T>(args: I) -> <Self::Commands as CommandSet>::ReturnType
//     where
//         I: IntoIterator<Item = T> + Send,
//         // T doesn't need `Send` because clap consumes all of the arguments
//         // before returning the `Self` value, with no held references to the
//         // args. As such, there are no values held across thread boundaries,
//         // and no need for `Send`.
//         T: Into<OsString> + Clone,
//     {
//         Self::parse_from(args).dispatch(ctx).await
//     }
//
//     async fn try_execute_from<I, T>(args: I) -> Result<<Self::Commands as CommandSet>::ReturnType>
//     where
//         I: IntoIterator<Item = T> + Send,
//         // T doesn't need `Send` because clap consumes all of the arguments
//         // before returning the `Self` value, with no held references to the
//         // args. As such, there are no values held across thread boundaries,
//         // and no need for `Send`.
//         T: Into<OsString> + Clone,
//     {
//         Ok(Self::try_parse_from(args)?.dispatch(ctx).await)
//     }
//
//     async fn dispatch() -> <Self::Commands as CommandSet>::ReturnType {
//         Self::parse().dispatch(ctx).await
//     }
//
//     async fn try_dispatch() -> Result<<Self::Commands as CommandSet>::ReturnType> {
//         Ok(Self::try_parse()?.dispatch(ctx).await)
//     }
// }

