use proc_macro::TokenStream;
use syn::DeriveInput;

#[inline(always)]
pub(crate) fn derive_macro(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();

    macro_impl(ast)
}

#[inline(always)]
pub(crate) fn async_derive_macro(tokens: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(tokens).unwrap();

    async_macro_impl(ast)
}

fn macro_impl(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;

    quote::quote! {
        impl harmony_autonomy::Execute for #ident {
            fn execute(&self) -> harmony_autonomy::execute::ExecuteReturn {
                todo!()
            }
        }
    }
    .into()
}

fn async_macro_impl(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;

    quote::quote! {
        impl harmony_autonomy::ExecuteAsync for #ident {
            fn execute(&self) -> harmony_autonomy::execute::ExecuteReturn {
                todo!()
            }
        }
    }
    .into()
}
