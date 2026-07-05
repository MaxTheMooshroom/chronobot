use proc_macro2::TokenStream;

fn get_type_name(_t: &Type, _suffix: impl std::borrow::Borrow<str>) -> String {
    todo!()
}

pub fn match_delegates(delegates: core::iter::Iter<super::VariantData>) -> TokenStream {
    delegates.iter()
        .map(|variant| {
            quote::quote! { }
        })
        .collect()
}
