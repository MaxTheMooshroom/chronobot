use proc_macro2::TokenStream;
use syn::DeriveInput;

type Result<T = TokenStream> = deluxe::Result<T>;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(cmd_prefix))]
pub(crate) struct CommandSetPrefix(String);
// pub(crate) struct CommandSetAttributes {
//     // TODO
//     // switch to a wrapper type around `&'static str`
//     // to reduce heap allocation during builds?
//     prefix: String,
// }

#[inline(always)]
pub(crate) fn derive_macro(tokens: TokenStream) -> Result {
    let mut ast: DeriveInput = syn::parse2(tokens)?;
    let CommandSetPrefix(prefix) = deluxe::extract_attributes(&mut ast)?;

    Ok(macro_impl(ast, prefix))
}

fn macro_impl(ast: DeriveInput, prefix: String) -> TokenStream {
    let ident = ast.ident;
    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();

    quote::quote! {
        impl #impl_generics CommandSet for #ident #type_generics
        #where_clause
        {
            const PREFIX: &'static str = #prefix;
        }
    }
    .into()
}
