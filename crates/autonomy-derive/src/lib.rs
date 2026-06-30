mod command_set;
mod execute;

use proc_macro::TokenStream;

#[proc_macro_derive(CommandSet, attributes(cmd_prefix))]
pub fn command_set_derive_macro(tokens: TokenStream) -> TokenStream {
    command_set::derive_macro(tokens.into()).unwrap().into()
}

#[proc_macro_derive(Execute)]
pub fn execute_derive_macro(tokens: TokenStream) -> TokenStream {
    execute::derive_macro(tokens)
}

#[proc_macro_derive(ExecuteAsync)]
pub fn execute_async_derive_macro(tokens: TokenStream) -> TokenStream {
    execute::async_derive_macro(tokens)
}

