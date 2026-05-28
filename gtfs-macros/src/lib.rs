// example of struct input, showcasing intended implementation for date/time/datetime alternate
// parsing

use std::{error::Error, mem};

use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{
    LitInt, Token, Type, TypePath, bracketed,
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

struct CustomInput {
    base_converter: Option<syn::Ident>,
    // _cma: Option<syn::Ident>,
    feature_clause: Option<CustomInputFeature>,
}

impl Parse for CustomInput {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            base_converter: input.parse()?,
            feature_clause: CustomInputFeature::parse(input).ok(),
        })
    }
}

struct CustomInputFeature {
    _brk: syn::token::Bracket,
    feature: syn::LitStr,
    _cma1: syn::Ident,
    alt_type: syn::TypePath,
    _cma2: syn::Ident,
    custom_convert: syn::Ident,
}

impl Parse for CustomInputFeature {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _brk: bracketed!(content in input),
            feature: content.parse()?,
            _cma1: content.parse()?,
            alt_type: content.parse()?,
            _cma2: content.parse()?,
            custom_convert: content.parse()?,
        })
    }
}

struct FieldMeta {
    field_name: syn::Ident,
    base_parser: Option<syn::Ident>,
    feature: Option<syn::LitStr>,
    feature_parser: Option<syn::Ident>,
}

// impl ToTokens for FieldMeta {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let FieldMeta {
//             field_name,
//             custom_parser,
//             is_required,
//             is_vec,
//         } = self;
//
//         let a = bracketed!(tokens);
//
//         let field_str = format!("{}", field_name);
//
//         let new_content = match (is_required, is_vec, custom_parser) {
//             (true, false, Some(parser)) => {
//                 quote! { #field_name: #parser(#field_name.ok_or(format!("Failed to process required field {}", #field_str))?) }
//             }
//             (true, true, Some(parser)) => {
//                 quote! { #field_name: #field_name.into_iter().map(#parser).collect() }
//             }
//             (false, false, Some(parser)) => {
//                 quote! { #field_name: #field_name.map(|c| #parser(c))}
//             }
//             (true, false, None) => {
//                 quote! { #field_name: #field_name.ok_or(format!("Failed to process required field {}", #field_str))?.try_into().map_err(|_| format!("Unable to convert {} into target type", #field_str))? }
//             }
//             (true, true, None) => {
//                 quote! { #field_name: #field_name.into_iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()? }
//             }
//             (false, false, None) => {
//                 quote! { #field_name: #field_name.map(|c| c.try_into()).collect::<Result<_, _>>()? }
//             }
//             (false, true, _) => unreachable!(),
//         };
//
//         tokens.append_all(new_content);
//     }
// }

// impl ToTokens for FieldMeta {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let FieldMeta {
//             field_name,
//             custom_parser,
//             is_required,
//             is_vec,
//         } = self;
//
//         let a = bracketed!(tokens);
//
//         let field_str = format!("{}", field_name);
//
//         let new_content = match (is_required, is_vec, custom_parser) {
//             (true, false, Some(parser)) => {
//                 quote! { #field_name: #parser(#field_name.ok_or(format!("Failed to process required field {}", #field_str))?) }
//             }
//             (true, true, Some(parser)) => {
//                 quote! { #field_name: #field_name.into_iter().map(#parser).collect() }
//             }
//             (false, false, Some(parser)) => {
//                 quote! { #field_name: #field_name.map(|c| #parser(c))}
//             }
//             (true, false, None) => {
//                 quote! { #field_name: #field_name.ok_or(format!("Failed to process required field {}", #field_str))?.try_into().map_err(|_| format!("Unable to convert {} into target type", #field_str))? }
//             }
//             (true, true, None) => {
//                 quote! { #field_name: #field_name.into_iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()? }
//             }
//             (false, false, None) => {
//                 quote! { #field_name: #field_name.map(|c| c.try_into()).collect::<Result<_, _>>()? }
//             }
//             (false, true, _) => unreachable!(),
//         };
//
//         tokens.append_all(new_content);
//     }
// }

impl ToTokens for FieldMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldMeta {
            field_name,
            base_parser,
            feature,
            feature_parser,
        } = self;

        let new_tokens = match (base_parser, feature, feature_parser) {
            (Some(base_parser), Some(feature), Some(feature_parser)) => {
                quote! {
                    #[cfg(feature = #feature)]
                    #field_name: #base_parser(#feature_parser(#field_name)?)?,
                    #[cfg(not(feature = #feature))]
                    #field_name: #base_parser(#field_name)?
                }
            }
            (Some(base_parser), None, None) => {
                quote! {
                    #field_name: #base_parser(#field_name)?
                }
            }
            (None, Some(feature), Some(feature_parser)) => {
                quote! {
                    #[cfg(feature = #feature)]
                    #field_name: #feature_parser(#field_name)?,
                    #[cfg(not(feature = #feature))]
                    #field_name: #field_name
                }
            }
            (None, None, None) => {
                quote! {
                    #field_name: match #field_name {
                        None => None,
                        Some(field) => Some(field.try_into().map_err(|_| String::from("Unable to parse field"))?)
                    }
                }
            }
            _ => panic!("Unexpected FieldMeta value"),
        };

        tokens.append_all(new_tokens.into_iter());
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
        let mut custom_input: Option<CustomInput> = None;

        for attr in mem::take(&mut ptr.attrs) {
            if attr.path().get_ident().map(|i| i.to_string()).as_deref() == Some("gtfs_custom") {
                custom_input = Some(
                    attr.parse_args()
                        .expect("Unable to parse chrono macro input"),
                );
            } else {
                attrs.push(attr);
            }
        }

        let _ = mem::replace(&mut ptr.attrs, attrs);

        let mut custom_convert: Option<syn::Ident> = None;

        match custom_input {
            Some(input) => {
                let CustomInput {
                    base_converter,
                    feature_clause,
                } = input;
                if let Some(CustomInputFeature {
                    feature,
                    alt_type,
                    custom_convert,
                    ..
                }) = feature_clause
                {
                    fields.push(quote!(
                        #[cfg(not(feature = #feature))]
                        #ptr
                    ));

                    ptr.ty = Type::Path(alt_type);

                    fields.push(quote!(
                        #[cfg(not(feature = #feature))]
                        #ptr
                    ));

                    fields_meta.push(FieldMeta {
                        field_name: ptr.ident.clone().expect("Fields should be named"),
                        base_parser: base_converter,
                        feature: Some(feature),
                        feature_parser: Some(custom_convert),
                    });
                } else {
                    // No feature clause
                    fields.push(quote! {
                        #ptr
                    });

                    fields_meta.push(FieldMeta {
                        field_name: ptr.ident.clone().expect("Fields should be named"),
                        base_parser: base_converter,
                        feature: None,
                        feature_parser: None,
                    });
                }
            }
            None => {
                fields.push(quote! {
                    #ptr
                });
                fields_meta.push(FieldMeta {
                    field_name: ptr.ident.clone().expect("Fields should be named"),
                    base_parser: None,
                    feature: None,
                    feature_parser: None,
                });
            }
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

    let struct_ident = item_struct.ident.clone();
    let proto_ident: syn::TypePath = syn::parse(args).expect("Unable to parse path to proto type");

    let field_names = fields_meta.iter().map(|f| &f.field_name);

    let fields_meta_alt: Vec<&FieldMeta> = fields_meta.iter().collect();

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
                        #fields_meta_alt
                    ),*
                })
            }
        }
    };

    // panic!("{}", struct_convert);

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
        #[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
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
// pub fn parse_required<T>(input: Option<T>) -> Result<T, gtfs-rust::error::Error> {
//     input.ok_or(gtfs-rust::error::Error::CustomConvertError(String::from("ABCD")))
// }
//
// pub fn parse_date(input: Option<String>) -> Result<Option<NaiveDate>, gtfs-rust::error::Error {
//     Ok(input.map(|d| {
//         // parse the date from the string
//
//         Ok(date)
//     })?)
// }
//
// #[gtfs_realtime_model(protos::TestStruct2)]
// pub struct TestStruct2 {
//     field1: Option<String>,
//     #[gtfs_custom(parse_required)]
//     field2: String,
//     field3: Vec<String>,
//     #[gtfs_custom(parse_required, [chrono, NaiveDate, parse_date])]
//     field4: String,
// }
//
// generates
// pub struct TestStruct2 {
//     field1: Option<String>,
//     field2: String,
//     field3: Vec<String>,
//     #[cfg(feature = "chrono")]
//     field4: NaiveDate,
//     #[cfg(not(feature = "chrono"))]
//     field4: String
// }
//
// impl TryFrom<protos::TestStruct2> for TestStruct2 {
//    type Error = gtfs-rust::error::Error;
//
//    fn try_from(value: protos::TestStruct2) -> Result<Self, Self::Error> {
//        let protos::TestStruct2 {
//            field1,
//            field2,
//            field3,
//            field4,
//            ..
//        } = value;
//
//        Self {
//            field1,
//            field2: parse_required(field2)?,
//            field3,
//            #[cfg(feature = "chrono")]
//            field4: parse_required(parse_date(field4)?)?,
//            #[cfg(not(feature = "chrono"))]
//            field4: parse_required(field4)?
//        }
//    }
// }
//
//
