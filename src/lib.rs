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
    // let fields =
    match ast.data {
        syn::Data::Struct(_struct_item) => {
            quote::quote! {}.into()
        }
        syn::Data::Enum(data_enum) => {
            println!("{data_enum:#?}");
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
                                        let error_field_name_token_steam = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig;
                                                        #error_field_name_token_steam.to_string_with_config_for_source_to_string_with_config(config)
                                                    },
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
                                                        #error_field_name_token_steam.few_to_string_with_config(config)
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
                                        let error_field_name_token_steam = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam,
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
                                        let error_field_name_token_steam = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                                                        #error_field_name_token_steam.to_string_without_config()
                                                    },
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                quote::quote! {
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam,
                                                        #second_field_ident: _code_occurence,
                                                    } => {
                                                        use #path_token_stream::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
                                                        #error_field_name_token_steam.few_to_string_without_config()
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
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                let error_field_name_underscore_token_steam = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam: #error_field_name_underscore_token_steam,
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
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                let error_field_name_underscore_token_steam = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam: #error_field_name_underscore_token_steam,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                let error_field_name_underscore_token_steam = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #ident::#variant_ident {
                                                        #error_field_name_token_steam: #error_field_name_underscore_token_steam,
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
                            let logic_for_struct_or_enum_with_deserialize = match &origin_or_wrapper {
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
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                // todo - maybe later add
                                                // let second_field_type_with_deserialize_token_steam = format!("{second_field_type}WithDeserialize")
                                                // .parse::<proc_macro2::TokenStream>()
                                                // .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #variant_ident {
                                                        #error_field_name_token_steam: #first_field_type,
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
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                // todo - maybe later add
                                                // let second_field_type_with_deserialize_token_steam = format!("{second_field_type}WithDeserialize")
                                                // .parse::<proc_macro2::TokenStream>()
                                                // .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #variant_ident {
                                                        #error_field_name_token_steam: #first_field_type,
                                                        #[serde(borrow)]
                                                        #second_field_ident: #path_token_stream::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
                                                    },
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                // todo - maybe later add
                                                // let second_field_type_with_deserialize_token_steam = format!("{second_field_type}WithDeserialize")
                                                // .parse::<proc_macro2::TokenStream>()
                                                // .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #variant_ident {
                                                        #error_field_name_token_steam: #first_field_type,
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
                                        let error_field_name_token_steam = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                        let second_field_ident_underscore_token_steam = format!("_{second_field_ident}")
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => {
                                                quote::quote! {
                                                    #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_steam,
                                                        #second_field_ident: #second_field_ident_underscore_token_steam,
                                                    } => format!("{}", #error_field_name_token_steam),
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
                                        let error_field_name_token_steam = error_field_name.to_string()
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                        let second_field_ident_underscore_token_steam = format!("_{second_field_ident}")
                                        .parse::<proc_macro2::TokenStream>()
                                        .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                        match error_field_name {
                                            ErrorFieldName::Error => panic!("ImplErrorOccurence error field name is error, but struct/enum field is Wrapper"),
                                            ErrorFieldName::InnerError => quote::quote! {
                                                #ident_with_deserialize_token_stream::#variant_ident {
                                                    #error_field_name_token_steam,
                                                    #second_field_ident: #second_field_ident_underscore_token_steam,
                                                } => {
                                                    use #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                                                    #error_field_name_token_steam.to_string_without_config()
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => quote::quote! {
                                                #ident_with_deserialize_token_stream::#variant_ident {
                                                    #error_field_name_token_steam,
                                                    #second_field_ident: #second_field_ident_underscore_token_steam,
                                                } => {
                                                    use #path_token_stream::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig;
                                                    #error_field_name_token_steam.few_to_string_without_config()
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
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                let error_field_name_underscore_token_steam = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                     #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_steam: #error_field_name_underscore_token_steam,
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
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                let error_field_name_underscore_token_steam = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_steam: #error_field_name_underscore_token_steam,
                                                        #second_field_ident,
                                                    } => #second_field_ident,
                                                }
                                            },
                                            ErrorFieldName::InnerErrors => {
                                                let error_field_name_token_steam = error_field_name.to_string()
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_token_steam parse failed");
                                                let error_field_name_underscore_token_steam = format!("_{error_field_name}")
                                                .parse::<proc_macro2::TokenStream>()
                                                .expect("ImplErrorOccurence error_field_name_underscore_token_steam parse failed");
                                                quote::quote!{
                                                    #ident_with_deserialize_token_stream::#variant_ident {
                                                        #error_field_name_token_steam: #error_field_name_underscore_token_steam,
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
                                    #logic_for_struct_or_enum_with_deserialize
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
                        SuportedEnumVariant::Unnamed => todo!(),
                    };
                    quote::quote! {
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
                    }.into()
                },
                None => panic!("ImplErrorOccurence enums where variants named first field name == error | inner_error | inner_errors not found"),
            }
        }
        _ => panic!("ImplErrorOccurence only works on structs and enums!"),
    }

    // let error_and_where_was_init = if first_source_type_ident_as_string == *"Vec" {
    //     let ident_as_string = match source_type_ident.path.segments[0].arguments.clone() {
    //         syn::PathArguments::None => {
    //             panic!("ImplErrorOccurence does not work with syn::PathArguments::None")
    //         }
    //         syn::PathArguments::AngleBracketed(angle_bracketed) => {
    //             match angle_bracketed.args.len() {
    //                 1 => match angle_bracketed.args[0].clone() {
    //                     syn::GenericArgument::Type(type_handle) => match type_handle {
    //                         syn::Type::Path(type_path) => match type_path.path.segments.len() {
    //                             1 => type_path.path.segments[0].ident.to_string(),
    //                             _ => {
    //                                 panic!("ImplErrorOccurence type_path.path.segments.len() != 1")
    //                             }
    //                         },
    //                         _ => panic!("ImplErrorOccurence works only with syn::Type::Path"),
    //                     },
    //                     _ => {
    //                         panic!("ImplErrorOccurence works only with syn::GenericArgument::Type")
    //                     }
    //                 },
    //                 _ => panic!("ImplErrorOccurence 1 angle_bracketed.args.len() != 1"),
    //             }
    //         }
    //         syn::PathArguments::Parenthesized(_) => {
    //             panic!("ImplErrorOccurence does not work with syn::PathArguments::Parenthesized")
    //         }
    //     };
    //     if ident_as_string.contains(WRAPPER_NAME) && ident_as_string.contains(ORIGIN_NAME) {
    //         panic!(
    //             "ImplErrorOccurence - ident name {} contains {} and {}",
    //             ident_as_string, WRAPPER_NAME, ORIGIN_NAME
    //         );
    //     } else if ident_as_string.contains(WRAPPER_NAME) {
    //         quote::quote! {

    //             match source_place_type {
    //                 #source_place_type_source_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|e| e.get_log_where_was(source_place_type, CONFIG.log_type, e.get_source()))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_github_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|e| e.get_log_where_was(source_place_type, CONFIG.log_type, e.get_source()))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_none_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|e| format!("{}, ", e.get_source()))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //             };
    //         }
    //     } else if ident_as_string.contains(ORIGIN_NAME) {
    //         quote::quote! {

    //           match source_place_type {
    //               #source_place_type_source_token_stream => {
    //                   let mut error_handle = source
    //                   .iter()
    //                   .map(|e| format!("{}, ", e))
    //                   .fold(String::from(""), |mut acc, elem| {
    //                       acc.push_str(&elem);
    //                       acc
    //                   });
    //                   if !error_handle.is_empty() {
    //                       error_handle.pop();
    //                       error_handle.pop();
    //                   }
    //                   let where_was_handle = where_was.file_line_column();
    //                   match CONFIG.log_type {
    //                       #tracing_token_stream => {
    //                           tracing::error!(error = error_handle);
    //                       }
    //                       #stack_token_stream => {
    //                           println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                       }
    //                       #none_token_stream => (),
    //                   }
    //               }
    //               #source_place_type_github_token_stream => {
    //                   let mut error_handle = source
    //                   .iter()
    //                   .map(|e| format!("{}, ", e))
    //                   .fold(String::from(""), |mut acc, elem| {
    //                       acc.push_str(&elem);
    //                       acc
    //                   });
    //                   if !error_handle.is_empty() {
    //                       error_handle.pop();
    //                       error_handle.pop();
    //                   }
    //                   let where_was_handle = where_was.github_file_line_column(&where_was.git_info);
    //                   match CONFIG.log_type {
    //                       #tracing_token_stream => {
    //                           tracing::error!(error = error_handle);
    //                       }
    //                       #stack_token_stream => {
    //                           println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                       }
    //                       #none_token_stream => (),
    //                   }
    //               }
    //               #source_place_type_none_token_stream => {
    //                   let mut error_handle = source
    //                   .iter()
    //                   .map(|e| format!("{}, ", e))
    //                   .fold(String::from(""), |mut acc, elem| {
    //                       acc.push_str(&elem);
    //                       acc
    //                   });
    //                   if !error_handle.is_empty() {
    //                       error_handle.pop();
    //                       error_handle.pop();
    //                   }
    //                   match CONFIG.log_type {
    //                       #tracing_token_stream => {
    //                           tracing::error!(error = error_handle);
    //                       }
    //                       #stack_token_stream => {
    //                           println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                       }
    //                       #none_token_stream => (),
    //                   }
    //               }
    //           };
    //         }
    //     } else {
    //         panic!(
    //             "ImplErrorOccurence - ident name {} does not contain {} or {}",
    //             ident_as_string, WRAPPER_NAME, ORIGIN_NAME
    //         );
    //     }
    // } else if first_source_type_ident_as_string == *"HashMap" {
    //     let ident_as_string = match source_type_ident.path.segments[0].arguments.clone() {
    //         syn::PathArguments::None => {
    //             panic!("ImplErrorOccurence does not work with syn::PathArguments::None")
    //         }
    //         syn::PathArguments::AngleBracketed(angle_bracketed) => {
    //             match angle_bracketed.args.len() {
    //                 2 => match angle_bracketed.args[1].clone() {
    //                     syn::GenericArgument::Type(type_handle) => match type_handle {
    //                         syn::Type::Path(type_path) => match type_path.path.segments.len() {
    //                             1 => type_path.path.segments[0].ident.to_string(),
    //                             _ => {
    //                                 panic!("ImplErrorOccurence type_path.path.segments.len() != 1")
    //                             }
    //                         },
    //                         _ => panic!("ImplErrorOccurence works only with syn::Type::Path"),
    //                     },
    //                     _ => {
    //                         panic!("ImplErrorOccurence works only with syn::GenericArgument::Type")
    //                     }
    //                 },
    //                 _ => panic!("ImplErrorOccurence 2 angle_bracketed.args.len() != 1"),
    //             }
    //         }
    //         syn::PathArguments::Parenthesized(_) => {
    //             panic!("ImplErrorOccurence does not work with syn::PathArguments::Parenthesized")
    //         }
    //     };
    //     if ident_as_string.contains(WRAPPER_NAME) && ident_as_string.contains(ORIGIN_NAME) {
    //         panic!(
    //             "ImplErrorOccurence - ident name {} contains {} and {}",
    //             ident_as_string, WRAPPER_NAME, ORIGIN_NAME
    //         );
    //     } else if ident_as_string.contains(WRAPPER_NAME) || ident_as_string.contains(ORIGIN_NAME) {
    //         quote::quote! {

    //             match source_place_type {
    //                 #source_place_type_source_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|(key, e)| e.get_log_where_was(source_place_type, CONFIG.log_type, format!("{} {}", key, e.get_source())))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_github_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|(key, e)| e.get_log_where_was(source_place_type, CONFIG.log_type, format!("{} {}", key, e.get_source())))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_none_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|(key, e)| format!("{} {}, ", key, e.get_source()))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //             };
    //         }
    //     } else {
    //         quote::quote! {

    //             match source_place_type {
    //                 #source_place_type_source_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|(key, e)| format!("{} {}, ", key, e))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     let where_was_handle = where_was.file_line_column();
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_github_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|(key, e)| format!("{} {}, ", key, e))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     let where_was_handle = where_was.github_file_line_column(&where_was.git_info);
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_none_token_stream => {
    //                     let mut error_handle = source
    //                     .iter()
    //                     .map(|(key, e)| format!("{} {}, ", key, e))
    //                     .fold(String::from(""), |mut acc, elem| {
    //                         acc.push_str(&elem);
    //                         acc
    //                     });
    //                     if !error_handle.is_empty() {
    //                         error_handle.pop();
    //                         error_handle.pop();
    //                     }
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //             };
    //         }
    //     }
    // } else if first_source_type_ident_as_string.contains(ERROR_ENUM_NAME) {
    //     if first_source_type_ident_as_string.contains(WRAPPER_NAME)
    //         && first_source_type_ident_as_string.contains(ORIGIN_NAME)
    //     {
    //         panic!(
    //             "ImplErrorOccurence - ident name {} contains {} and {}",
    //             first_source_type_ident_as_string, WRAPPER_NAME, ORIGIN_NAME
    //         );
    //     } else if first_source_type_ident_as_string.contains(WRAPPER_NAME) {
    //         quote::quote! {

    //             match source_place_type {
    //                 #source_place_type_source_token_stream => {
    //                     let error_handle = source.get_log_with_additional_where_was(
    //                         &where_was,
    //                         source_place_type,
    //                         source.get_source(),
    //                         CONFIG.log_type
    //                     );
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_github_token_stream => {
    //                     let error_handle = source.get_log_with_additional_where_was(
    //                         &where_was,
    //                         source_place_type,
    //                         source.get_source(),
    //                         CONFIG.log_type
    //                     );
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_none_token_stream => {
    //                     let error_handle = source.get_source();
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //             };
    //         }
    //     } else if first_source_type_ident_as_string.contains(ORIGIN_NAME) {
    //         quote::quote! {
    //             match source_place_type {
    //                 #source_place_type_source_token_stream => {
    //                     let error_handle = source.get_source();
    //                     let where_was_handle = where_was.file_line_column();
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_github_token_stream => {
    //                     let error_handle = source.get_source();
    //                     let where_was_handle = where_was.github_file_line_column(&where_was.git_info);
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //                 #source_place_type_none_token_stream => {
    //                     let error_handle = source.get_source();
    //                     match CONFIG.log_type {
    //                         #tracing_token_stream => {
    //                             tracing::error!(error = error_handle);
    //                         }
    //                         #stack_token_stream => {
    //                             println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                         }
    //                         #none_token_stream => (),
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         panic!(
    //             "ImplErrorOccurence - ident name {} does not contain {} or {}",
    //             first_source_type_ident_as_string, WRAPPER_NAME, ORIGIN_NAME
    //         );
    //     }
    // } else {
    //     quote::quote! {
    //         match source_place_type {
    //             #source_place_type_source_token_stream => {
    //                 let error_handle = format!("{} {}", where_was.file_line_column(), source);
    //                 match CONFIG.log_type {
    //                     #tracing_token_stream => {
    //                         tracing::error!(error = error_handle);
    //                     }
    //                     #stack_token_stream => {
    //                         println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                     }
    //                     #none_token_stream => (),
    //                 }
    //             }
    //             #source_place_type_github_token_stream => {
    //                 let error_handle = format!("{} {}", where_was.github_file_line_column(&where_was.git_info), source);
    //                 match CONFIG.log_type {
    //                     #tracing_token_stream => {
    //                         tracing::error!(error = error_handle);
    //                     }
    //                     #stack_token_stream => {
    //                         println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                     }
    //                     #none_token_stream => (),
    //                 }
    //             }
    //             #source_place_type_none_token_stream => {
    //                 let error_handle = format!("{}", source);
    //                 match CONFIG.log_type {
    //                     #tracing_token_stream => {
    //                         tracing::error!(error = error_handle);
    //                     }
    //                     #stack_token_stream => {
    //                         println!("{}", CONFIG.get_error_color_bold().paint(error_handle));
    //                     }
    //                     #none_token_stream => (),
    //                 }
    //             }
    //         }
    //     }
    // };
}

//

//
