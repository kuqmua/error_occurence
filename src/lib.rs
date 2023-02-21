// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]

use proc_macro_helpers::global_variables::hardcode::ERROR_ENUM_NAME;
use proc_macro_helpers::global_variables::hardcode::ORIGIN_NAME;
use proc_macro_helpers::global_variables::hardcode::WRAPPER_NAME;

#[proc_macro_derive(ImplErrorOccurenceFromTufaCommon)]
pub fn derive_impl_error_occurence_tufa_common(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::TufaCommon)
}

#[proc_macro_derive(ImplErrorOccurenceFromCrate)]
pub fn derive_impl_error_occurence_from_crate(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::Crate)
}

enum OriginOrWrapper {
    Origin,
    Wrapper,
}

enum ErrorFieldName {
    Error,
    InnerError,
    InnerErrors,
}

impl From<proc_macro2::Ident> for ErrorFieldName {
    fn from(item: proc_macro2::Ident) -> Self {
        if item == *"error" {
            ErrorFieldName::Error
        } else if item == *"inner_error" {
            ErrorFieldName::InnerError
        } else if item == *"inner_errors" {
            ErrorFieldName::InnerErrors
        } else {
            panic!("ImplErrorOccurence only works with enums where variants named first field name == error | inner_error | inner_errors");
        }
    }
}

impl std::fmt::Display for ErrorFieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorFieldName::Error => write!(f, "error"),
            ErrorFieldName::InnerError => write!(f, "inner_error"),
            ErrorFieldName::InnerErrors => write!(f, "inner_errors"),
        }
    }
}

enum SuportedEnumVariant {
    Named,
    Unnamed,
}
//todo check on full path generation to enums
fn generate(
    input: proc_macro::TokenStream,
    path: proc_macro_helpers::path::Path,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplErrorOccurence syn::parse(input) failed");
    let ident = &ast.ident;
    let ident_as_string = ident.to_string();
    let ident_with_deserialize_token_stream = format!("{ident}WithDeserialize")
        .parse::<proc_macro2::TokenStream>()
        .expect("identWithDeserialize parse failed");
    let path_token_stream = format!("{path}")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let origin_or_wrapper = if ident_as_string.contains(WRAPPER_NAME)
        && ident_as_string.contains(ORIGIN_NAME)
    {
        panic!(
            "ImplErrorOccurence - ident name {ident_as_string} contains {WRAPPER_NAME} and {ORIGIN_NAME}",
        );
    } else if ident_as_string.contains(WRAPPER_NAME) {
        OriginOrWrapper::Wrapper
    } else if ident_as_string.contains(ORIGIN_NAME) {
        OriginOrWrapper::Origin
    } else {
        panic!(
            "ImplErrorOccurence - ident name {ident_as_string} does not contain {WRAPPER_NAME} or {ORIGIN_NAME}",
        );
    };
    match ast.data {
        syn::Data::Enum(data_enum) => {
            // println!("{data_enum:#?}");
            let mut all_equal: Option<SuportedEnumVariant> = None;
            for variant in &data_enum.variants {
                match &variant.fields {
                    syn::Fields::Named(_) => {
                        match &all_equal {
                            Some(supported_variant) => {
                                match supported_variant {
                                    SuportedEnumVariant::Named => (),
                                    SuportedEnumVariant::Unnamed => panic!("ImplErrorOccurence only works with enums where all variants are named or all variants are unnamed"),
                                }
                            },
                            None => {
                                all_equal = Some(SuportedEnumVariant::Named);
                            },
                        }
                    },
                    syn::Fields::Unnamed(_) => {
                        match &all_equal {
                            Some(supported_variant) => {
                                match supported_variant {
                                    SuportedEnumVariant::Named => panic!("ImplErrorOccurence only works with enums where all variants are named or all variants are unnamed"),
                                    SuportedEnumVariant::Unnamed => (),
                                }
                            },
                            None => {
                                all_equal = Some(SuportedEnumVariant::Unnamed);
                            },
                        }
                    },
                    syn::Fields::Unit => panic!("ImplErrorOccurence only works with named fields"),
                }
            }
            let config_name_for_source_to_string_with_config = match origin_or_wrapper {
                OriginOrWrapper::Origin => String::from("_config")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path parse failed"),
                OriginOrWrapper::Wrapper => String::from("config")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path parse failed"),
            };
            match all_equal {
                Some(supported_enum_variant) => {
                    let generated = match supported_enum_variant {
                        SuportedEnumVariant::Named => {
                            let vec_needed_info = {
                                let mut vec_needed_info: Vec<(&proc_macro2::Ident, ErrorFieldName, &syn::Type, proc_macro2::Ident, &syn::Type)> = Vec::new();
                                data_enum.variants.iter().for_each(|variant| {
                                    let variant_ident = &variant.ident;
                                    let needed_info = match &variant.fields {
                                        syn::Fields::Named(fields_named) => {
                                            let named = &fields_named.named;
                                            match named.len() == 2 {
                                                true => {
                                                    let first_field = &named[0];
                                                    let first_field_ident =
                                                        first_field.ident.clone().expect("ImplErrorOccurence enum variant first field ident is None");
                                                    let error_field_name = ErrorFieldName::from(first_field_ident);
                                                    let second_field = &named[1];
                                                    let second_field_ident =
                                                        second_field.ident.clone().expect("enum variant second field ident is None");
                                                    if second_field_ident != *"code_occurence" {
                                                        panic!("ImplErrorOccurence only works with enums where variants named first field name == error | inner_error | inner_errors");
                                                    }
                                                    (error_field_name, &first_field.ty, second_field_ident, &second_field.ty)
                                                },
                                                false => panic!("ImplErrorOccurence only works on named fields with length of 2"),
                                            }
                                        },
                                        syn::Fields::Unnamed(_) => panic!("ImplErrorOccurence unexpected named unnamed logic"),
                                        _ => panic!("ImplErrorOccurence only works with named fields"),
                                    };
                                    vec_needed_info.push((variant_ident, needed_info.0, needed_info.1, needed_info.2, needed_info.3));
                                });
                                vec_needed_info
                            };
                            match vec_needed_info.is_empty() {
                                true => panic!("ImplErrorOccurence enum variants are empty"),
                                false => (),
                            }
                            let logic_for_source_to_string_with_config = match &origin_or_wrapper {
                                OriginOrWrapper::Origin => quote::quote! {
                                    use #path_token_stream::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
                                    self.source_to_string_without_config()
                                },
                                OriginOrWrapper::Wrapper => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        let error_field_name_token_stream = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig;
                                                        #error_field_name_token_stream.to_string_with_config_for_source_to_string_with_config(config)
                                                    },
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
                                                        #error_field_name_token_stream.few_to_string_with_config(config)
                                                    },
                                                }
                                            },
                                        }
                                    });
                                    quote::quote! {
                                        match self {
                                            #(#generated_variants_logic),*
                                        }
                                    }
                                },
                            };
                            let logic_for_source_to_string_without_config = match &origin_or_wrapper {
                                OriginOrWrapper::Origin => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        let error_field_name_token_stream = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream,
                                                        #second_field_ident: _code_occurence,
                                                    } => format!("{}", error),
                                                }
                                            },
                                            ErrorFieldName::InnerError => panic!("ImplErrorOccurence error field name is inner_error, but struct/enum field is Origin"),
                                            ErrorFieldName::InnerErrors => panic!("ImplErrorOccurence error field name is inner_errors, but struct/enum field is Origin"),
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                                OriginOrWrapper::Wrapper => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        let error_field_name_token_stream = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                                                        #error_field_name_token_stream.to_string_without_config()
                                                    },
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
                                                        #error_field_name_token_stream.few_to_string_without_config()
                                                    },
                                                }
                                            },
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                            };
                            let logic_for_get_code_occurence = match &origin_or_wrapper {
                                OriginOrWrapper::Origin => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        match error_field_name {
                                            ErrorFieldName::Error => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                let error_field_name_underscore_token_stream = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                            ErrorFieldName::InnerError => panic!("ImplErrorOccurence error field name is inner_error, but struct/enum field is Origin"),
                                            ErrorFieldName::InnerErrors => panic!("ImplErrorOccurence error field name is inner_errors, but struct/enum field is Origin"),
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                                OriginOrWrapper::Wrapper => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                let error_field_name_underscore_token_stream = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                let error_field_name_underscore_token_stream = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                            };
                            let logic_for_enum_with_deserialize = match &origin_or_wrapper {
                                OriginOrWrapper::Origin => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        match error_field_name {
                                            ErrorFieldName::Error => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                // todo - maybe later add
                                                // let second_field_type_with_deserialize_token_stream = format!("{second_field_type}WithDeserialize")
                                                // .parse::<proc_macro2::TokenStream>()
                                                // .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #variant_ident {
                                                        #error_field_name_token_stream: #first_field_type,
                                                        #[serde(borrow)]
                                                        #second_field_ident: #path_token_stream::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
                                                    },
                                                }
                                            },
                                            ErrorFieldName::InnerError => panic!("ImplErrorOccurence error field name is inner_error, but struct/enum field is Origin"),
                                            ErrorFieldName::InnerErrors => panic!("ImplErrorOccurence error field name is inner_errors, but struct/enum field is Origin"),
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                                OriginOrWrapper::Wrapper => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                // todo - maybe later add
                                                // let second_field_type_with_deserialize_token_stream = format!("{second_field_type}WithDeserialize")
                                                // .parse::<proc_macro2::TokenStream>()
                                                // .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                //
                                                let first_field_type_stringified = match first_field_type {
                                                    syn::Type::Path(type_path_handle) => {
                                                        let mut segments_stringified = type_path_handle.path.segments.iter()
                                                        .fold(String::from(""), |mut acc, elem| {
                                                            acc.push_str(&format!("{}::", elem.ident));
                                                            acc
                                                        });
                                                        segments_stringified.pop();
                                                        segments_stringified.pop();
                                                        format!("{segments_stringified}WithDeserialize<'a>")
                                                    },
                                                    _ => panic!("ImplErrorOccurence works only with syn::Type::Path"),
                                                };
                                                let first_field_type_token_stream = first_field_type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #variant_ident {
                                                        #[serde(borrow)]
                                                        #error_field_name_token_stream: #first_field_type_token_stream,
                                                        #[serde(borrow)]
                                                        #second_field_ident: #path_token_stream::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
                                                    },
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                // todo - maybe later add
                                                // let second_field_type_with_deserialize_token_stream = format!("{second_field_type}WithDeserialize")
                                                // .parse::<proc_macro2::TokenStream>()
                                                // .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                let first_field_type_prep = match first_field_type {
                                                    syn::Type::Path(type_path) => {
                                                        // println!("####\n{:#?}\n####", type_path);
                                                        let variant_type = {
                                                            // let first_segment = &type_path.path.segments[0];
                                                            // let ident = &first_segment.ident;
                                                            let mut v = type_path.path.segments.iter()
                                                            .fold(String::from(""), |mut acc, elem| {
                                                                let elem_ident = &elem.ident;
                                                                //todo: check of HashMap true only once
                                                                //todo check on Vec true only once
                                                                //todo check on HashMap or Vec are last element
                                                                if *elem_ident == "Vec" {
                                                                    match &elem.arguments {
                                                                        syn::PathArguments::None => panic!("ImplErrorOccurence first_segment.arguments syn::PathArguments::None for Vec"),
                                                                        syn::PathArguments::AngleBracketed(angle_bracketed) => {
                                                                            match angle_bracketed.args.len() == 1 {
                                                                                true => {
                                                                                    match &angle_bracketed.args[0] {
                                                                                        syn::GenericArgument::Type(gt) => {
                                                                                            match gt {
                                                                                                syn::Type::Path(type_path_handle) => {
                                                                                                    let mut segments_stringified = type_path_handle.path.segments.iter()
                                                                                                    .fold(String::from(""), |mut acc, elem| {
                                                                                                        acc.push_str(&format!("{}::", elem.ident));
                                                                                                        acc
                                                                                                    });
                                                                                                    segments_stringified.pop();
                                                                                                    segments_stringified.pop();
                                                                                                    acc.push_str(&format!("Vec<{segments_stringified}WithDeserialize<'a>>::"))//todo remove ::
                                                                                                },
                                                                                                _ => panic!("ImplErrorOccurence works only with syn::Type::Path for Vec"),
                                                                                            }
                                                                                        },
                                                                                        _ => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for Vec"),
                                                                                    }
                                                                                },
                                                                                false => panic!("ImplErrorOccurence works only with angle_bracketed.args.len() == 1 for Vec"),
                                                                            }
                                                                        },
                                                                        syn::PathArguments::Parenthesized(_) => panic!("ImplErrorOccurence first_segment.arguments syn::PathArguments::Parenthesized for Vec"),
                                                                    }
                                                                }
                                                                else if *elem_ident == "HashMap" {
                                                                    match &elem.arguments {
                                                                        syn::PathArguments::None => panic!("ImplErrorOccurence first_segment.arguments syn::PathArguments::None for HashMap"),
                                                                        syn::PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => {
                                                                            match angle_bracketed_generic_arguments.args.len() == 2 {
                                                                                true => {
                                                                                    let hashmap_key = match &angle_bracketed_generic_arguments.args[0] {
                                                                                        syn::GenericArgument::Lifetime(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap key"),
                                                                                        syn::GenericArgument::Type(type_handle) => {
                                                                                            match type_handle {
                                                                                                syn::Type::Path(type_path_handle_two) => {
                                                                                                    let mut segments_stringified = type_path_handle_two.path.segments.iter()
                                                                                                    .fold(String::from(""), |mut acc, elem| {
                                                                                                        acc.push_str(&format!("{}::", elem.ident));
                                                                                                        acc
                                                                                                    });
                                                                                                    segments_stringified.pop();
                                                                                                    segments_stringified.pop();
                                                                                                    segments_stringified
                                                                                                },
                                                                                                _ => panic!("ImplErrorOccurence works only with syn::Type::Path for HashMap"),
                                                                                            }
                                                                                        },
                                                                                        syn::GenericArgument::Const(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap key"),
                                                                                        syn::GenericArgument::Binding(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap key"),
                                                                                        syn::GenericArgument::Constraint(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap key"),
                                                                                    };
                                                                                    let hashmap_value = match &angle_bracketed_generic_arguments.args[1] {
                                                                                        syn::GenericArgument::Lifetime(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap value"),
                                                                                        syn::GenericArgument::Type(type_handle) => {
                                                                                            match type_handle {
                                                                                                syn::Type::Path(type_path_handle_three) => {
                                                                                                    let mut segments_stringified = type_path_handle_three.path.segments.iter()
                                                                                                    .fold(String::from(""), |mut acc, elem| {
                                                                                                        acc.push_str(&format!("{}::", elem.ident));
                                                                                                        acc
                                                                                                    });
                                                                                                    segments_stringified.pop();
                                                                                                    segments_stringified.pop();
                                                                                                    format!("{segments_stringified}WithDeserialize<'a>")
                                                                                                },
                                                                                                _ => panic!("ImplErrorOccurence works only with syn::Type::Path for HashMap"),
                                                                                            }
                                                                                        },
                                                                                        syn::GenericArgument::Const(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap value"),
                                                                                        syn::GenericArgument::Binding(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap value"),
                                                                                        syn::GenericArgument::Constraint(_) => panic!("ImplErrorOccurence works only with syn::GenericArgument::Type for HashMap value"),
                                                                                    };
                                                                                    acc.push_str(&format!("{elem_ident}<{hashmap_key},{hashmap_value}>::"));//todo - maybe incorrect to add :: in the end
                                                                                },
                                                                                false => panic!("ImplErrorOccurence works only with angle_bracketed_generic_arguments.args.len() == 2 for HashMap"),
                                                                            }
                                                                        },
                                                                        syn::PathArguments::Parenthesized(_) => panic!("ImplErrorOccurence first_segment.arguments syn::PathArguments::Parenthesized for HashMap"),
                                                                    }
                                                                }
                                                                else {
                                                                    //todo - its maybe uncorrect
                                                                    acc.push_str(&format!("{elem_ident}::"));
                                                                }
                                                                acc
                                                            });
                                                            v.pop();
                                                            v.pop();
                                                            // println!("@##@@\n{}@##@\n", v);
                                                            v
                                                            // let variant_type_ident = type_path.path.segments
                                                        };
                                                        // println!("\n@@@{}\n@@@", variant_type);
                                                        variant_type
                                                    },
                                                    _ => panic!("ImplErrorOccurence first_field_type supports only syn::Type::Path"),
                                                };
                                                let first_field_type_with_deserialize_token_stream = first_field_type_prep
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #variant_ident {
                                                        #[serde(borrow)]
                                                        #error_field_name_token_stream: #first_field_type_with_deserialize_token_stream,
                                                        #[serde(borrow)]
                                                        #second_field_ident: #path_token_stream::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
                                                    },
                                                }
                                            },
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                            };
                            let logic_for_source_to_string_without_config_with_deserialize = match &origin_or_wrapper {
                                OriginOrWrapper::Origin => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        let error_field_name_token_stream = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                        let second_field_ident_underscore_token_stream = format!("_{second_field_ident}")
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => {
                                                quote::quote! {
                                                    #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_stream,
                                                        #second_field_ident: #second_field_ident_underscore_token_stream,
                                                    } => format!("{}", #error_field_name_token_stream),
                                                }
                                            },
                                            ErrorFieldName::InnerError => panic!("ImplErrorOccurence error field name is inner_error, but struct/enum field is Origin"),
                                            ErrorFieldName::InnerErrors => panic!("ImplErrorOccurence error field name is inner_errors, but struct/enum field is Origin"),
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                                OriginOrWrapper::Wrapper => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        let error_field_name_token_stream = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                        let second_field_ident_underscore_token_stream = format!("_{second_field_ident}")
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => quote::quote! {
                                                #ident_with_deserialize_token_stream::#variant_ident {
                                                    #error_field_name_token_stream,
                                                    #second_field_ident: #second_field_ident_underscore_token_stream,
                                                } => {
                                                    use #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
                                                    #error_field_name_token_stream.to_string_without_config_with_deserialize()
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => quote::quote! {
                                                #ident_with_deserialize_token_stream::#variant_ident {
                                                    #error_field_name_token_stream,
                                                    #second_field_ident: #second_field_ident_underscore_token_stream,
                                                } => {
                                                    use #path_token_stream::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfigWithDeserialize;
                                                    #error_field_name_token_stream.few_to_string_without_config_with_deserialize()
                                                }
                                            },
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                            };
                            let logic_for_get_code_occurence_with_deserialize = match &origin_or_wrapper {
                                OriginOrWrapper::Origin => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        match error_field_name {
                                            ErrorFieldName::Error => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                let error_field_name_underscore_token_stream = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                     #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                            ErrorFieldName::InnerError => panic!("ImplErrorOccurence error field name is inner_error, but struct/enum field is Origin"),
                                            ErrorFieldName::InnerErrors => panic!("ImplErrorOccurence error field name is inner_errors, but struct/enum field is Origin"),
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                                OriginOrWrapper::Wrapper => {
                                    let generated_variants_logic = vec_needed_info.iter().map(|(
                                        variant_ident, 
                                        error_field_name, 
                                        _first_field_type,
                                        second_field_ident, 
                                        _second_field_type
                                    )|{
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                let error_field_name_underscore_token_stream = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                let error_field_name_token_stream = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_stream parse failed");
                                                let error_field_name_underscore_token_stream = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_stream parse failed");
                                                quote::quote!{
                                                    #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                                },
                            };
                            quote::quote! {
                                //difference done?
                                impl<'a, ConfigGeneric>
                                    #path_token_stream::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
                                        'a,
                                        ConfigGeneric,
                                    > for #ident<'a>
                                    where ConfigGeneric: #path_token_stream::traits::fields::GetSourcePlaceType
                                        + #path_token_stream::traits::fields::GetTimezone
                                        + #path_token_stream::traits::get_server_address::GetServerAddress,
                                {
                                    fn source_to_string_with_config(
                                        &self,
                                        #config_name_for_source_to_string_with_config: &ConfigGeneric
                                    ) -> String {
                                        #logic_for_source_to_string_with_config
                                    }
                                }
                                //difference done?
                                impl<'a>
                                    #path_token_stream::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
                                        'a,
                                    > for #ident<'a>
                                {
                                    fn source_to_string_without_config(&self) -> String {
                                        match self {
                                            #logic_for_source_to_string_without_config
                                        }
                                    }
                                }
                                //difference(only in error naming)
                                impl<'a> #path_token_stream::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
                                    for #ident<'a>
                                {
                                    fn get_code_occurence(&self) -> &#path_token_stream::common::code_occurence::CodeOccurence<'a> {
                                        match self {
                                            #logic_for_get_code_occurence
                                        }
                                    }
                                }
                                //difference
                                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
                                pub enum #ident_with_deserialize_token_stream<'a> {
                                    #logic_for_enum_with_deserialize
                                }
                                //difference
                                impl<'a> #path_token_stream::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<'a,> for #ident_with_deserialize_token_stream<'a>
                                {
                                    fn source_to_string_without_config(&self) -> String {
                                        match self {
                                            #logic_for_source_to_string_without_config_with_deserialize
                                        }
                                    }
                                }
                                //difference(only in error naming)
                                impl<'a> #path_token_stream::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
                                    for #ident_with_deserialize_token_stream<'a>
                                {
                                    fn get_code_occurence_with_deserialize(
                                        &self,
                                    ) -> &#path_token_stream::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
                                        match self {
                                            #logic_for_get_code_occurence_with_deserialize
                                        }
                                    }
                                }
                            }
                        },
                        SuportedEnumVariant::Unnamed => {
                            let vec_needed_info = {
                                let mut vec_needed_info: Vec<(&proc_macro2::Ident, &syn::Type)> = Vec::new();
                                data_enum.variants.iter().for_each(|variant| {
                                    let needed_info = match &variant.fields {
                                        syn::Fields::Named(_) => panic!("ImplErrorOccurence unexpected named unnamed logic"),
                                        syn::Fields::Unnamed(fields_unnamed) => {
                                            let unnamed = &fields_unnamed.unnamed;
                                            let first_field = &unnamed[0];//todo - how to handle error in this case?
                                            &first_field.ty
                                        },
                                        _ => panic!("ImplErrorOccurence only works with named fields"),
                                    };
                                    vec_needed_info.push((&variant.ident, needed_info));
                                });
                                vec_needed_info
                            };
                            match vec_needed_info.is_empty() {
                                true => panic!("ImplErrorOccurence enum variants are empty"),
                                false => (),
                            }
                            let logic_for_to_string_with_config_for_source_to_string_with_config = {
                                let generated_variants_logic = vec_needed_info.iter().map(|(
                                    variant_ident, 
                                    first_field_type, 
                                )|{
                                    let gen = match first_field_type {
                                        syn::Type::Path(type_path) => {
                                            let last_segment_ident = type_path.path.segments.last().expect("ImplErrorOccurence no last segment in type_path.path.segments").ident.to_string();
                                            let origin_or_wrapper = match (last_segment_ident.contains(WRAPPER_NAME), last_segment_ident.contains(ORIGIN_NAME)) {
                                                (true, true) => panic!("ImplErrorOccurence last_segment_ident contains Wrapper and Origin"),
                                                (true, false) => OriginOrWrapper::Wrapper,
                                                (false, true) => OriginOrWrapper::Origin,
                                                (false, false) => panic!("ImplErrorOccurence last_segment_ident do not contain Wrapper or Origin"),
                                            };
                                            match origin_or_wrapper {
                                                OriginOrWrapper::Origin => quote::quote! {
                                                    use #path_token_stream::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
                                                    i.to_string_with_config_for_source_to_string_without_config(config)
                                                },
                                                OriginOrWrapper::Wrapper => quote::quote! {
                                                    i.to_string_with_config_for_source_to_string_with_config(config)
                                                },
                                            }
                                        },
                                        _ => panic!("ImplErrorOccurence first_field_type supports only syn::Type::Path"),
                                    };
                                    quote::quote!{
                                        #ident::#variant_ident(i) => {
                                            #gen
                                        }
                                    }
                                });
                                quote::quote! {
                                    #(#generated_variants_logic),*
                                }
                            };
                            // println!("_____________");
                            // println!("{}", logic_for_to_string_with_config_for_source_to_string_with_config);
                            let logic_for_to_string_without_config = {
                                let gen = vec_needed_info.iter().map(|(
                                    variant_ident, 
                                    _first_field_type, 
                                )|
                                    quote::quote!{
                                        #ident::#variant_ident(i) => i.to_string_without_config()
                                    }
                                );
                                quote::quote! {
                                    #(#gen),*
                                }
                            };
                            let logic_for_enum_with_deserialize = {
                                let generated_variants_logic = vec_needed_info.iter().map(|(
                                    variant_ident, 
                                    first_field_type
                                    )|{
                                        let variant_type_with_deserialize_token_stream = match first_field_type {
                                            syn::Type::Path(type_path) => {
                                                let variant_type = {
                                                    let mut segments_stringified = type_path.path.segments.iter()
                                                    .fold(String::from(""), |mut acc, elem| {
                                                        acc.push_str(&format!("{}::", elem.ident));
                                                        acc
                                                    });
                                                    segments_stringified.pop();
                                                    segments_stringified.pop();
                                                    segments_stringified
                                                };
                                                format!("{variant_type}WithDeserialize")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence variant_type_with_deserialize_token_stream parse failed")                                                
                                            },
                                            _ => panic!("ImplErrorOccurence first_field_type supports only syn::Type::Path"),
                                        };
                                        
                                        quote::quote!{
                                            #[serde(borrow)]
                                            #variant_ident(#variant_type_with_deserialize_token_stream<'a>)
                                        }
                                    });
                                    quote::quote! {
                                        #(#generated_variants_logic),*
                                    }
                            };
                            // println!("_________________)))))))");
                            // println!("{}", logic_for_enum_with_deserialize);
                            // println!("_________________)))))))");
                            let logic_for_to_string_without_config_with_deserialize = {
                                let gen = vec_needed_info.iter().map(|(
                                    variant_ident, 
                                    _first_field_type, 
                                )|
                                    quote::quote!{
                                        #ident_with_deserialize_token_stream::#variant_ident(i) => i.to_string_without_config_with_deserialize()
                                    }
                                );
                                quote::quote! {
                                    #(#gen),*
                                }
                            };
                            quote::quote! {
                                impl<'a, ConfigGeneric>
                                    #path_token_stream::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
                                    'a,
                                    ConfigGeneric,
                                    > for #ident<'a>
                                where
                                    ConfigGeneric: #path_token_stream::traits::fields::GetSourcePlaceType
                                    + #path_token_stream::traits::fields::GetTimezone
                                    + #path_token_stream::traits::get_server_address::GetServerAddress,
                                {
                                    fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
                                        match self {
                                            #logic_for_to_string_with_config_for_source_to_string_with_config
                                        }
                                    }
                                }
                                impl<'a> #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
                                    for #ident<'a>
                                {
                                    fn to_string_without_config(&self) -> String {
                                        match self {
                                            #logic_for_to_string_without_config
                                        }
                                    }
                                }
                                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)] 
                                pub enum #ident_with_deserialize_token_stream<'a> {
                                    #logic_for_enum_with_deserialize
                                }
                                
                                impl<'a>
                                    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
                                        'a,
                                    > for #ident_with_deserialize_token_stream<'a>
                                {
                                    fn to_string_without_config_with_deserialize(&self) -> String {
                                        match self {
                                            #logic_for_to_string_without_config_with_deserialize
                                            // #ident_with_deserialize_token_stream::SevenWrapper(i) => {
                                            //     i.to_string_without_config_with_deserialize()
                                            // }
                                        }
                                    }
                                }
                            }
                        },
                    };
                    let uuu = quote::quote! {
                        impl<'a> std::fmt::Display for #ident<'a> {
                            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                                use #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                                write!(f, "{}", self.to_string_without_config())
                            }
                        }
                        #generated
                        impl<'a> std::fmt::Display for #ident_with_deserialize_token_stream<'a> {
                            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                                use #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
                                write!(f, "{}", self.to_string_without_config_with_deserialize())
                            }
                        }
                    };
                    // println!("{}", uuu);
                    uuu.into()
                },
                None => panic!("ImplErrorOccurence enums where variants named first field name == error | inner_error | inner_errors not found"),
            }
        }
        _ => panic!("ImplErrorOccurence only works on enums!"),
    }
}