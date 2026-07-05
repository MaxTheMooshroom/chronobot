mod command_context;
mod command_set;
mod macros;

use proc_macro::TokenStream;

#[proc_macro_derive(CommandContext, attributes(set_prefix, command))]
pub fn command_context_derive_macro(tokens: TokenStream) -> TokenStream {
    command_context::derive_macro(tokens.into()).unwrap().into()
}

#[proc_macro_derive(CommandSet, attributes(cmdset, delegate))]
pub fn command_set_derive_macro(tokens: TokenStream) -> TokenStream {
    command_set::derive_macro(tokens.into()).unwrap().into()
}

