use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input,
    punctuated::Punctuated,
    token::{Comma, Gt, Lt},
    AngleBracketedGenericArguments, Attribute, Data, DeriveInput, Field, Fields, GenericArgument,
    Ident, Meta, Path, PathArguments, PathSegment, Type, TypePath,
};

fn check_attr(attr: &Attribute, name: &str) -> bool {
    let Meta::Path(path) = &attr.meta else {
        return false;
    };
    let Some(s) = path.segments.first() else {
        return false;
    };
    s.ident == name
}
fn is_attr_internal(attr: &Attribute) -> bool {
    check_attr(attr, "internal")
}

#[proc_macro_derive(Optional, attributes(derives, internal))]
pub fn derive_optionnal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", input);

    let name = input.ident;
    let internal_name = Ident::new(format!("{}Internal", name).as_str(), Span::call_site());

    let derives = input
        .attrs
        .into_iter()
        .find(|attr| {
            attr.meta
                .path()
                .segments
                .first()
                .map_or(false, |s| s.ident.to_string() == "derives")
        })
        .map_or(TokenStream::new(), |attr| {
            let Meta::List(list) = attr.meta else {
                return TokenStream::new();
            };
            let items = list.tokens;
            quote!(#[derive(#items)])
        });

    let Data::Struct(structure) = input.data else {
        panic!("Expected a struct");
    };
    let Fields::Named(fields) = structure.fields else {
        panic!("Expected named fields");
    };
    let new_fields: Punctuated<Field, Comma> =
        Punctuated::from_iter(fields.named.into_iter().map(|field| {
            Field {
                ty: Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: Punctuated::from_iter([PathSegment {
                            ident: Ident::new("Option", Span::call_site()),
                            arguments: PathArguments::AngleBracketed(
                                AngleBracketedGenericArguments {
                                    colon2_token: None,
                                    lt_token: Lt {
                                        spans: [Span::call_site()],
                                    },
                                    args: Punctuated::from_iter([GenericArgument::Type(
                                        if field.attrs.iter().any(is_attr_internal) {
                                            Type::Path(TypePath {
                                                qself: None,
                                                path: Path {
                                                    leading_colon: None,
                                                    segments: Punctuated::from_iter([
                                                        PathSegment {
                                                            ident: Ident::new(
                                                                format!(
                                                                    "{}Internal",
                                                                    field.ty.to_token_stream()
                                                                )
                                                                .as_str(),
                                                                Span::call_site(),
                                                            ),
                                                            arguments: PathArguments::None,
                                                        },
                                                    ]),
                                                },
                                            })
                                        } else {
                                            field.ty
                                        },
                                    )]),
                                    gt_token: Gt {
                                        spans: [Span::call_site()],
                                    },
                                },
                            ),
                        }]),
                    },
                }),
                attrs: field
                    .attrs
                    .into_iter()
                    .filter(|attr| !is_attr_internal(attr))
                    .collect(),
                ..field
            }
        }));

    println!("{}", new_fields.to_token_stream());

    let output: TokenStream = quote! {
        impl #name {
            fn hello() {
                todo!()
            }
        }
        #derives
        struct #internal_name {
            #new_fields
        }
    };

    output.into()
}
