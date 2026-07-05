use proc_macro::TokenStream;

macro_rules! parse_error {
    ($span:path, $msg:literal) => { syn::Error::new_spanned($span, $msg).to_compile_error() };
}

#[proc_macro_derive(Enum)]
pub fn enum_derive_macro(tokens: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let ident = &ast.ident;

    if !matches!(ast.data, syn::Data::Enum(_)) {
        return parse_error!(ident, "`#[derive(enum_trait::Enum)]` only supports enums").into();
    }

    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();

    quote::quote! {
        impl #impl_generics enum_trait::base::EnumBase for #ident #type_generics
        #where_clause
        {
            type AreYouSure = enum_trait::base::Yes;
        }

        impl #impl_generics enum_trait::Enum for #ident #type_generics #where_clause {}
    }
    .into()
}
