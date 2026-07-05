use deluxe::{ExtractAttributes, extract_attributes};
use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Index, Member, Type};

type Result<T = TokenStream> = deluxe::Result<T>;

#[derive(ExtractAttributes)]
#[deluxe(attributes(set_prefix))]
struct CommandContextPrefix(String);

#[derive(ExtractAttributes)]
#[deluxe(attributes(command))]
struct CommandContextSubcommands {
    subcommand: bool,
}

#[inline(always)]
pub(crate) fn derive_macro(tokens: TokenStream) -> Result {
    let mut ast: DeriveInput = syn::parse2(tokens)?;
    let CommandContextPrefix(prefix) = extract_attributes(&mut ast)?;

    let ident = ast.ident.clone();

    let subcommand_types = find_subcommands(&mut ast);
    let size = subcommand_types.len();

    if size == 0 {
        return Err(Error::new_spanned(
            ident,
            "could not find field annotated with `#[command(subcommand)]`"
        ));
    }

    if size > 1 {
        return Err(Error::new_spanned(
            ident,
            "It is illegal to annotate more than one field with `#[command(subcommand)]`"
        ));
    }

    let (member, field_type) = &subcommand_types[0];

    let (impl_generics, type_generics, where_clause) =
        ast.generics.split_for_impl();

    Ok(
        quote::quote! {
            impl #impl_generics CommandContext for #ident #type_generics
            #where_clause
            {
                const PREFIX: &'static str = #prefix;
                type Commands = #field_type;

                fn commands(self) -> Self::Commands {
                    self.#member
                }
            }
        }
        .into()
    )
}

fn find_subcommands(ast: &mut DeriveInput) -> Vec<(Member, Type)> {
    if let syn::Data::Struct(s) = &mut ast.data {
        s.fields.iter_mut()
            .enumerate()
            .map(|(i, field)| {
                let is_match = extract_attributes::<_, CommandContextSubcommands>(field)
                    .map(|ctx| ctx.subcommand)
                    .unwrap_or(false);

                (i, is_match, field)
            })
            .filter(|field| field.1)
            .map(|field| {
                let ident = field.2.ident.clone()
                    .map(|id| Member::Named(id))
                    .unwrap_or_else(
                        || Member::Unnamed(Index::from(field.0))
                    );

                (ident, field.2.ty.clone())
            })
            .fold(
                Vec::with_capacity(1),
                |mut vec, item| {
                    vec.push(item);
                    vec
                }
            )
    } else {
        Vec::new()
    }
}

