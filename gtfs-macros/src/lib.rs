// example of struct input, showcasing intended implementation for date/time/datetime alternate
// parsing

use std::{error::Error, mem};

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    Token, Type,
    parse::{self, Parse},
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

struct FieldMeta {
    field_name: syn::Ident,
    custom_parser: Option<syn::Ident>,
    is_required: bool,
}

impl ToTokens for FieldMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldMeta {
            field_name,
            custom_parser,
            is_required,
        } = self;

        let field_str = format!("{}", field_name);

        let new_content = match (is_required, custom_parser) {
            (true, Some(parser)) => {
                quote! { #field_name: #parser(#field_name.ok_or(format!("Failed to process required field {}", #field_str))?) }
            }
            (true, None) => {
                quote! { #field_name: #field_name.ok_or(format!("Failed to process required field {}", #field_str))?.try_into().map_err(|_| format!("Unable to convert {} into target type", #field_str))? }
            }
            (false, Some(parser)) => {
                quote! { #field_name: #field_name.map(#parser) }
            }
            (false, None) => {
                quote! { #field_name: #field_name.try_into().map_err(|_| format!("Unable to convert {} into target type", #field_str))? }
            }
        };

        tokens.append_all(new_content);
    }
}

#[proc_macro_attribute]
pub fn gtfs_realtime_model(
    args: proc_macro::TokenStream,
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
    let mut fields_meta = Vec::new();

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

        let mut custom_convert: Option<syn::Ident> = None;

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
                    custom_convert = Some(deser.clone());
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

        let mut is_required = true;

        let ty = &ptr.ty;
        if let Type::Path(type_path) = ty {
            let path = &type_path.path;

            for segment in path.segments.iter() {
                if &segment.ident.to_string() == "Option" {
                    is_required = false;
                }
            }
        }

        fields_meta.push(FieldMeta {
            field_name: ptr.ident.clone().expect("Fields should be named"),
            custom_parser: custom_convert,
            is_required,
        });
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

    let struct_ident = item_struct.ident.clone();
    let proto_ident: syn::TypePath = syn::parse(args).expect("Unable to parse path to proto type");

    let field_names = fields_meta.iter().map(|f| &f.field_name);

    let struct_convert = quote! {
        impl TryFrom<#proto_ident> for #struct_ident {
            type Error = crate::error::Error;

            fn try_from(value: #proto_ident) -> Result<Self, Self::Error> {
                let #proto_ident {
                    #(#field_names),*,
                    ..
                } = value;

                Ok(Self {
                    #(
                        #fields_meta
                    ),*
                })
            }
        }
    };

    quote! {
        #[cfg(feature = "schedule_parse")]
        #parse_derive_line
        #item_struct

        #[cfg(not(feature = "schedule_parse"))]
        #base_derive_line
        #item_struct

        #[cfg(feature = "schedule_parse")]
        #struct_convert
    }
    .into()
}

// ```
//
// #[gtfs_schedule_model]
// pub struct TestStruct {
//     #[gtfs_chrono(<chrono_type>, <custom_deser>?)]
//     field: <raw type>
// }
//
// generates
// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct TestStruct {
//    #[cfg(not(feature = "chrono"))]
//    field: <raw_type>,
//    #[cfg(all(feature = "chrono", feature = "schedule_parse"))]
//    #[serde(deserialize_with = "<custom_deser>")]?
//    #[serde(default)]?
//    field: <chrono_type>,
//    #[cfg(all(feature = "chrono", not(feature = "schedule_parse")))]
//    field: <chrono_type>,
// }
//
//
// #[gtfs_realtime_model]
// pub struct TestStruct2 {
//     field1: Option<String>,
//     field2: String,
//     field3: Vec<String>,
// }
//
// generates
// pub struct TestStruct2 {
//     field1: Option<String>,
//     field2: String,
//     field3: Vec<String>,
// }
//
// impl From<protos::TestStruct2> for TestStruct2 {
//    fn from(value: protos::TestStruct2) -> Self {
//        let protos::TestStruct2 {
//            field1,
//            field2,
//            field3,
//        } = value;
//
//        Self {
//            field1,
//            field2: field2.expect("field2 is required"),
//            field3,
//        }
//    }
// }
//
// ```
