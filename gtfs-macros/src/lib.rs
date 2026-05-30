// example of struct input, showcasing intended implementation for date/time/datetime alternate
// parsing

use std::{error::Error, mem};

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    LitInt, Token, Type, TypePath, bracketed, parenthesized,
    parse::{self, Parse},
    parse_macro_input,
};

struct GtfsChronoInput {
    chrono_type: syn::TypePath,
    _cma: Option<Token![,]>,
    custom_deser: Option<syn::Ident>,
}

impl Parse for GtfsChronoInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            chrono_type: input.parse()?,
            _cma: input.parse()?,
            custom_deser: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn gtfs_schedule_model(
    _args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let base_derive_line = quote! {
        #[derive(Debug, Clone)]
    };
    let parse_derive_line = quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    };

    let mut item_struct: syn::ItemStruct = syn::parse(item).expect("Unable to parse input struct");

    let mut fields = Vec::new();

    for ptr in item_struct.fields.iter_mut() {
        let mut attrs = Vec::new();
        let mut chrono_input: Option<GtfsChronoInput> = None;

        for attr in mem::take(&mut ptr.attrs) {
            if attr.path().get_ident().map(|i| i.to_string()).as_deref() == Some("gtfs_chrono") {
                chrono_input = Some(
                    attr.parse_args()
                        .expect("Unable to parse chrono macro input"),
                );
            } else {
                attrs.push(attr);
            }
        }

        let _ = mem::replace(&mut ptr.attrs, attrs);

        match chrono_input {
            Some(input) => {
                let GtfsChronoInput {
                    chrono_type,
                    custom_deser,
                    ..
                } = input;
                fields.push(quote! {
                    #[cfg(not(feature = "chrono"))]
                    #ptr
                });

                ptr.ty = Type::Path(chrono_type);

                fields.push(quote! {
                    #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
                    #ptr
                });

                if let Some(deser) = custom_deser {
                    let deser_path = format!("{}", deser);
                    fields.push(quote! {
                        #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
                        #[serde(deserialize_with = #deser_path)]
                        #[serde(default)]
                        #ptr
                    });
                }
            }
            None => fields.push(quote! {
                #ptr
            }),
        }
    }

    item_struct.fields = syn::Fields::Named(
        syn::parse2(quote! {
            {
                #(
                    #fields
                ),*
            }
        })
        .expect("Unable to parse fields"),
    );

    quote! {
        #[cfg(feature = "schedule_parse")]
        #parse_derive_line
        #item_struct

        #[cfg(not(feature = "schedule_parse"))]
        #base_derive_line
        #item_struct
    }
    .into()
}

struct FeatureAlt {
    feature: syn::LitStr,
    alt_type: syn::TypePath,
    convert_fn: syn::Ident,
}

struct FeatureOpt {
    feature: syn::LitStr,
}

enum FeatureClause {
    Alt(FeatureAlt),
    Opt(FeatureOpt),
}

impl From<RtHelperFeatureClause> for FeatureClause {
    fn from(value: RtHelperFeatureClause) -> Self {
        let RtHelperFeatureClause {
            feature,
            alt_clause,
        } = value;

        match alt_clause {
            Some(alt_clause) => {
                let RtHelperFeatureAltClause {
                    alt_type,
                    convert_fn,
                    ..
                } = alt_clause;

                Self::Alt(FeatureAlt {
                    feature,
                    alt_type,
                    convert_fn,
                })
            }
            None => Self::Opt(FeatureOpt { feature }),
        }
    }
}

struct Variable {
    name: syn::Ident,
    ty: VariableType,
    default: Option<RtHelperTypeDefault>,
    feature_clause: Option<FeatureClause>,
}

impl Variable {
    fn new(name: syn::Ident, input: Option<RtHelperInput>) -> Self {
        match input {
            None => Self {
                name,
                ty: VariableType::Passthrough,
                default: None,
                feature_clause: None,
            },
            Some(RtHelperInput {
                type_clause: None,
                feature_clause,
            }) => Self {
                name,
                ty: VariableType::Passthrough,
                default: None,
                feature_clause: feature_clause.map(|fc| fc.into()),
            },
            Some(RtHelperInput {
                type_clause: Some(RtHelperTypeClause { ident, default, .. }),
                feature_clause,
            }) => Self {
                name,
                ty: ident.into(),
                default,
                feature_clause: feature_clause.map(|fc| fc.into()),
            },
        }
    }
}

impl ToTokens for Variable {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let name_str = format!("{}", name);

        let default = match &self.default {
            Some(default) => quote!(#default),
            None => quote!(Err(
                format!("Unable to access required variable {}", #name_str)
            )?),
        };

        let new_tokens = match &self.feature_clause {
            None => match self.ty {
                VariableType::Passthrough => quote! {
                    #name
                },
                VariableType::Required => quote! {
                    #name: match #name {
                        Some(mtch) => mtch,
                        None => #default,
                    }
                },
                VariableType::MessageField => quote! {
                    #name: match #name.take() {
                        Some(fld) => {
                            Some(fld.try_into()?)
                        },
                        None => None,
                    }
                },
                VariableType::MessageFieldReq => quote! {
                    #name: #name.take().ok_or(format!("Unable to parse required MessageField field: {}", #name_str))?.try_into()?
                },
                VariableType::Vec => quote! {
                    #name: #name.into_iter().map(|i| i.try_into()).collect::<Result<_, _>>()?
                },
                VariableType::Enum => quote! {
                    #name: match #name {
                        Some(mtch) => Some(mtch.try_into()?),
                        None => None,
                    }
                },
                VariableType::EnumReq => quote! {
                    #name: match #name {
                        Some(mtch) => mtch.try_into()?,
                        None => #default,
                    }
                },
            },
            Some(FeatureClause::Opt(FeatureOpt { feature })) => match self.ty {
                VariableType::Passthrough => quote! {
                    #[cfg(feature = #feature)]
                    #name
                },
                VariableType::Required => quote! {
                    #[cfg(feature = #feature)]
                    #name: match #name {
                        Some(mtch) => mtch,
                        None => #default,
                    }
                },
                VariableType::MessageField => quote! {
                    #[cfg(feature = #feature)]
                    #name: match #name.take() {
                        Some(fld) => {
                            Some(fld.try_into()?)
                        },
                        None => None,
                    }
                },
                VariableType::MessageFieldReq => quote! {
                    #[cfg(feature = #feature)]
                    #name: #name.take().ok_or(format!("Unable to parse required MessageField field: {}", #name_str))?.try_into()?
                },
                VariableType::Vec => quote! {
                    #[cfg(feature = #feature)]
                    #name: #name.into_iter().map(|i| i.try_into()).collect::<Result<_, _>>()?
                },
                VariableType::Enum => quote! {
                    #[cfg(feature = #feature)]
                    #name: match #name {
                        Some(mtch) => Some(mtch.try_into()?),
                        None => None
                    }
                },
                VariableType::EnumReq => quote! {
                    #[cfg(feature = #feature)]
                    #name: match #name {
                        Some(mtch) => mtch.try_into()?,
                        None => #default,
                    }
                },
            },
            Some(FeatureClause::Alt(FeatureAlt {
                feature,
                convert_fn,
                ..
            })) => match self.ty {
                VariableType::Passthrough => quote! {
                    #[cfg(not(feature = #feature))]
                    #name,
                    #[cfg(feature = #feature)]
                    #name: #convert_fn(#name)?
                },
                VariableType::Required => quote! {
                    #[cfg(not(feature = #feature))]
                    #name: #name.ok_or(format!("Required field not provided: {}", #name_str))?,
                    #[cfg(feature = #feature)]
                    #name: #convert_fn(#name)?.ok_or(format!("Required field not provided: {}", #name_str))?
                },
                _ => panic!("Unsupported type for alt feature clause"),
            },
        };

        tokens.append_all(new_tokens);
    }
}

enum VariableType {
    // This variable is a bare Option type in the protobuf representation, and an Option
    // type in the gtfs-rust field
    // DEFAULT
    Passthrough,
    // This field is a bare Option type in the protobuf representation, and a required type in the
    // gtfs-rust field
    Required,
    // This field is a MessageField type, and an Option type in the gtfs-rust field
    MessageField,
    // This field is a MessageField type, and a required type in the gtfs-rust field
    MessageFieldReq,
    // This field is a Vec of non-trivial types
    Vec,
    Enum,
    EnumReq,
}

impl From<syn::Ident> for VariableType {
    fn from(value: syn::Ident) -> Self {
        if value.to_string().to_uppercase() == "PASSTHROUGH" {
            Self::Passthrough
        } else if value.to_string().to_uppercase() == "REQUIRED" {
            Self::Required
        } else if value.to_string().to_uppercase() == "MF" {
            Self::MessageField
        } else if value.to_string().to_uppercase() == "MFREQ" {
            Self::MessageFieldReq
        } else if value.to_string().to_uppercase() == "VEC" {
            Self::Vec
        } else if value.to_string().to_uppercase() == "ENUM" {
            Self::Enum
        } else if value.to_string().to_uppercase() == "ENUMREQ" {
            Self::EnumReq
        } else {
            panic!("Unrecognized variable type")
        }
    }
}

struct RtHelperTypeClause {
    ident: syn::Ident,
    _prn: Option<syn::token::Paren>,
    default: Option<RtHelperTypeDefault>,
    _cma: Option<Token![,]>,
}

impl Parse for RtHelperTypeClause {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let content;

        let ident = input.parse()?;
        if input.peek(syn::token::Paren) {
            let _prn = parenthesized!(content in input);
            let default = if content.peek(syn::Lit) {
                RtHelperTypeDefault::Lit(content.parse()?)
            } else {
                RtHelperTypeDefault::Path(content.parse()?)
            };

            Ok(Self {
                ident,
                _prn: Some(_prn),
                default: Some(default),
                _cma: input.parse()?,
            })
        } else {
            Ok(Self {
                ident,
                _prn: None,
                default: None,
                _cma: input.parse()?,
            })
        }
    }
}

enum RtHelperTypeDefault {
    Path(syn::Path),
    Lit(syn::Lit),
}

impl ToTokens for RtHelperTypeDefault {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            RtHelperTypeDefault::Path(path) => quote! {#path},
            RtHelperTypeDefault::Lit(lit) => quote! {#lit},
        });
    }
}

struct RtHelperFeatureClause {
    feature: syn::LitStr,
    alt_clause: Option<RtHelperFeatureAltClause>,
}

impl Parse for RtHelperFeatureClause {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let feature = input.parse()?;
        let alt_clause = if input.peek(Token![,]) {
            Some(RtHelperFeatureAltClause::parse(input)?)
        } else {
            None
        };

        Ok(Self {
            feature,
            alt_clause,
        })
    }
}

struct RtHelperFeatureAltClause {
    _cma1: Token![,],
    alt_type: syn::TypePath,
    _cma2: Token![,],
    convert_fn: syn::Ident,
}

impl Parse for RtHelperFeatureAltClause {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _cma1: input.parse()?,
            alt_type: input.parse()?,
            _cma2: input.parse()?,
            convert_fn: input.parse()?,
        })
    }
}

struct RtHelperInput {
    type_clause: Option<RtHelperTypeClause>,
    feature_clause: Option<RtHelperFeatureClause>,
}

impl Parse for RtHelperInput {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let type_clause = if input.peek(syn::Ident) {
            Some(RtHelperTypeClause::parse(input)?)
        } else {
            None
        };

        let feature_clause = if input.peek(syn::LitStr) {
            Some(RtHelperFeatureClause::parse(input)?)
        } else {
            None
        };

        Ok(Self {
            type_clause,
            feature_clause,
        })
    }
}

#[proc_macro_attribute]
pub fn gtfs_realtime_model(
    arg: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let base_derive_line = quote! {
        #[derive(Debug, Clone, PartialEq)]
    };
    let parse_derive_line = quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
    };

    let mut item: syn::ItemStruct = parse_macro_input!(item);
    let arg: syn::TypePath = parse_macro_input!(arg);

    let mut variables = Vec::new();
    let mut fields = Vec::new();

    let name = &item.ident;

    for field in item.fields.iter_mut() {
        let name: syn::Ident = field.ident.clone().unwrap();
        let mut helper_input: Option<RtHelperInput> = None;

        let mut new_attrs = Vec::new();

        for attr in mem::take(&mut field.attrs).into_iter() {
            if attr.meta.path().get_ident().map(|i| i.to_string()) == Some(String::from("gtfs")) {
                helper_input = Some(attr.parse_args().unwrap());
            } else {
                new_attrs.push(attr);
            }
        }

        item.attrs = new_attrs;

        if let Some(RtHelperInput {
            feature_clause:
                Some(RtHelperFeatureClause {
                    feature,
                    alt_clause,
                    ..
                }),
            ..
        }) = &helper_input
        {
            match alt_clause {
                Some(RtHelperFeatureAltClause { alt_type, .. }) => {
                    fields.push(quote!(
                        #[cfg(not(feature = #feature))]
                        #field
                    ));

                    field.ty = syn::Type::Path(alt_type.clone());

                    fields.push(quote!(
                        #[cfg(feature = #feature)]
                        #field
                    ));
                }
                None => fields.push(quote!(
                    #[cfg(feature = #feature)]
                    #field
                )),
            }
        } else {
            fields.push(quote!(#field));
        }

        variables.push(Variable::new(name, helper_input));
    }

    item.fields = syn::Fields::Named(
        syn::parse2(quote! {{
            #(
                #fields
            ),*
        }})
        .unwrap(),
    );

    let field_names = variables.iter().map(|v| &v.name);

    let impls = quote! {
        impl TryFrom<#arg> for #name {
            type Error = crate::error::Error;

            fn try_from(value: #arg) -> Result<Self, Self::Error> {
                let #arg {
                    #(
                        mut #field_names
                    ),*
                    ,..
                } = value;

                Ok(Self {
                    #(
                        #variables
                    ),*
                })
            }
        }
    };

    let result = quote!(
        #[cfg(feature = "realtime_parse")]
        #parse_derive_line
        #item

        #[cfg(not(feature = "realtime_parse"))]
        #base_derive_line
        #item

        #impls
    );

    result.into()
}

#[proc_macro_attribute]
pub fn gtfs_realtime_enum(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item: syn::ItemEnum = parse_macro_input!(item);
    let mut variants = Vec::new();

    let proto_ident: TypePath = parse_macro_input!(args);
    let item_ident = &item.ident;

    for variant in item.variants.iter() {
        let ident = &variant.ident;

        variants.push(quote! {
            #proto_ident::#ident => Self::#ident
        });
    }

    let value = quote!(
        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
        #[repr(u8)]
        #item

        impl TryFrom<protobuf::EnumOrUnknown<#proto_ident>> for #item_ident {
            type Error = crate::error::Error;

            fn try_from(value: protobuf::EnumOrUnknown<#proto_ident>) -> Result<Self, Self::Error> {
                Ok(
                    match value.enum_value().unwrap_or_default() {
                        #(
                            #variants
                        ),*
                    }
                )
            }
        }
    )
    .into();

    value
}

