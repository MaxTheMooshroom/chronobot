use convert_case::{Case, Casing};
use deluxe::ExtractAttributes;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{DeriveInput, Field, Ident, Path, PathSegment, Token, Type, Variant, parse2};
use syn::punctuated::Punctuated;

use crate::macros::*;

type Result<T = TokenStream> = syn::Result<T>;

#[derive(ExtractAttributes)]
#[deluxe(attributes(cmdset))]
struct CommandSetAttributes {
    returns: Type,
    context: Type,
}

struct VariantData {
    ident: Ident,
    delegate: Path,
    field_names: Box<[Ident]>,
}

struct PathComponents(&'static [&'static str]);

impl std::cmp::PartialEq<Punctuated<PathSegment, Token![::]>> for PathComponents {
    fn eq(&self, other: &Punctuated<PathSegment, Token![::]>) -> bool {
        self.0.len() == other.len() && {
            let mut iter = other.iter();

            self.0.iter().fold(
                true,
                |b, segment| {
                    // this is always valid because we already did a check that
                    // they're the same length.
                    b && (iter.next().unwrap().ident == segment)
                }
            )
        }
    }
}

const VALID_ATTRIBUTE_PATHS_DELEGATE: &[PathComponents] = &[
    PathComponents(&[ "delegate" ]),
    PathComponents(&[ "harmony_autonomy", "delegate" ]),
];

fn sanitize_name(mut s: String) -> String {
    // SAFETY: replacing ASCII characters with other ASCII characters does not
    // change the length, so this is a safe operation.
    unsafe {
        for b in s.as_bytes_mut() {
            if !matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') {
                *b = b'_';
            }
        }
    }

    s
}

fn field_name((i, Field{ ident, ty, .. }): (usize, &Field)) -> Ident {
    let ty = ty.clone();

    ident.clone().unwrap_or_else(|| {
        let mut s = sanitize_name(ty.to_token_stream().to_string() + "_" + &i.to_string());
        s.retain(|c| !c.is_ascii_whitespace());
        Ident::new(&s.to_case(Case::Snake), ty.span())
    })
}

fn parse_fields<'a>(var: &'a Variant) -> Result<VariantData> {
    let Variant { attrs, ident, fields, .. } = var;

    let delegate_maybe: Option<Path> = attrs.iter()
        .map(|attr| &attr.meta)
        .filter_map(|meta| meta.require_list().ok())
        .filter(|list| VALID_ATTRIBUTE_PATHS_DELEGATE.iter().filter(|&s| *s == list.path.segments).next().is_some())
        .filter_map(|list| parse2::<Path>(list.tokens.clone()).ok())
        .next();

    let (delegate, field_names): (Path, Box<[Ident]>) = {
        match fields {
            syn::Fields::Unit => {
                delegate_maybe
                    .ok_or_else(|| parse_error!(
                        err,
                        ident,
                        "enums derived with `#[derive(CommandSet)]` must have their unit-variants annotated with `#[delegate(...)]`"
                    ))
                    .or_else(std::convert::identity)
                    .map(|delegate| (delegate, Box::default()))
            },
            syn::Fields::Unnamed(fields_unnamed) => {
                let mut fields_iter = fields_unnamed.unnamed.iter();

                let delegate: Path =
                    delegate_maybe.ok_or_else(|| {
                        let field_type: &Type = &fields_iter.next().unwrap().ty;
                        match field_type {
                            Type::Path(type_path) => Ok(type_path.path.clone()),
                            _ => parse_error!(err, ident, "Invalid `#[delegate(...)]` argument")
                        }
                    })
                    .or_else(std::convert::identity)?;

                let field_names = fields_iter
                    .enumerate()
                    .map(field_name)
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                Ok((delegate, field_names))
            },
            _ => parse_error!(
                err,
                ident,
                "if an enum is derived with `#[derive(CommandSet)]`, and a variant isn't annotated with `#[delegate(...)]`, it must be a tuple-variant (unnamed field) with at least one member"
            )
        }
    }?;

    Ok(VariantData { ident: ident.clone(), delegate, field_names, })
}

fn variant_match_arm(variants: &VariantData) -> TokenStream {
    let VariantData{ ident, delegate, field_names } = variants;

    quote!{
        #ident( #( #field_names ),* ) => #delegate(ec, ( #( #field_names ),* ))
    }
}

#[inline(always)]
pub(crate) fn derive_macro(tokens: TokenStream) -> Result {
    let mut ast: DeriveInput = parse2(tokens)?;

    let CommandSetAttributes{ returns, context, } = deluxe::extract_attributes(&mut ast)?;

    let ident = &ast.ident;
    let variants_raw: &Punctuated<Variant, Token![,]> = match &ast.data {
        syn::Data::Enum(data) => &data.variants,
        _ => return parse_error!(err, &ast.ident, "`#[derive(CommandSet)]` only supports enums")
    };

    let variants = variants_raw.iter()
        .map(parse_fields)
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(variant_match_arm)
        .collect::<Vec<_>>();

    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();

    Ok(
        quote! {
            impl #impl_generics CommandSet for #ident #type_generics
            #where_clause
            {
                type ExtraContext = #context;
                type ReturnType = #returns;

                fn dispatch(&self, ec: Self::ExtraContext) -> anyhow::Result<Self::ReturnType> {
                    match self {
                        #( #variants ),*
                    }
                }
            }
        }
        .into()
    )
}

