// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]
use proc_macro_helpers::global_variables::hardcode::ORIGIN_NAME;
use proc_macro_helpers::global_variables::hardcode::WRAPPER_NAME;

enum OriginOrWrapper {
    Origin,
    Wrapper,
}

#[derive(
    Debug,
    strum_macros::EnumIter,
    strum_macros::Display,
    enum_extension::EnumExtension
)]
enum ErrorFieldName {
    Error,
    InnerError,
    InnerErrors,
}

impl ErrorFieldName {
    fn to_all_variants_lower_case_string_vec() -> Vec<String> {
        Self::into_array().into_iter().map(|e|e.to_lower_snake_case()).collect::<Vec<String>>()
    }
}

enum SuportedEnumVariant {
    Named,
    Unnamed,
}

enum SupportedInnerErrorsContainers {
    Vec,
    HashMap,
    Other
}

//todo check on full path generation to enums
#[proc_macro_derive(ImplErrorOccurence)]
pub fn derive_impl_error_occurence(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let proc_macro_name = "ImplErrorOccurence";
    let ast: syn::DeriveInput =
        syn::parse(input).unwrap_or_else(|_| panic!("{proc_macro_name} syn::parse(input) failed"));
    let ident = &ast.ident;
    let ident_stringified = ident.to_string();
    let ident_with_deserialize_stringified = format!("{ident}WithDeserialize");
    let ident_with_deserialize_token_stream = ident_with_deserialize_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {ident_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified = "crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig";
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified}WithDeserialize");
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    //
    let use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified = "use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;";
    let use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream = use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified = "use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;";
    let use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream = 
    use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    //
    let crate_traits_fields_get_source_place_type_stringified = "crate::traits::fields::GetSourcePlaceType";
    let crate_traits_fields_get_source_place_type_token_stream = 
    crate_traits_fields_get_source_place_type_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_fields_get_source_place_type_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_fields_get_timezone_stringified = "crate::traits::fields::GetTimezone";
    let crate_traits_fields_get_timezone_token_stream = 
    crate_traits_fields_get_timezone_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_fields_get_timezone_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_get_server_address_get_server_address_stringified = "crate::traits::get_server_address::GetServerAddress";
    let crate_traits_get_server_address_get_server_address_token_stream = 
    crate_traits_get_server_address_get_server_address_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_get_server_address_get_server_address_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified = "crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig";
    let crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream = 
    crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified = "crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig";
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified = "crate::traits::error_logs_logic::few_to_string_without_config::FewToStringWithoutConfig";
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream = 
    crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified}WithDeserialize");
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream = 
    crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let few_to_string_without_config_stringified = "few_to_string_without_config";
    let few_to_string_without_config_token_stream = 
    few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {few_to_string_without_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let few_to_string_without_config_with_deserialize_stringified = format!("{few_to_string_without_config_stringified}_with_deserialize");
    let few_to_string_without_config_with_deserialize_token_stream = 
    few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {few_to_string_without_config_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let to_string_with_config_for_source_to_string_with_config_stringified = "to_string_with_config_for_source_to_string_with_config";
    let to_string_with_config_for_source_to_string_with_config_token_stream = 
    to_string_with_config_for_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {to_string_with_config_for_source_to_string_with_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let source_to_string_without_config_stringified = "source_to_string_without_config";
    let source_to_string_without_config_token_stream = 
    source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {source_to_string_without_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let to_string_without_config_stringified = "to_string_without_config";
    let to_string_without_config_token_stream = 
    to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {to_string_without_config_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let to_string_without_config_with_deserialize_stringified = "to_string_without_config_with_deserialize";
    let to_string_without_config_with_deserialize_token_stream = 
    to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {to_string_without_config_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified = "crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence";
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified}WithDeserialize");
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_common_code_occurence_code_occurence_stringified = "crate::common::code_occurence::CodeOccurence";
    let crate_common_code_occurence_code_occurence_token_stream = 
    crate_common_code_occurence_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_common_code_occurence_code_occurence_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let crate_common_code_occurence_code_occurence_with_deserialize_stringified = format!("{crate_common_code_occurence_code_occurence_stringified}WithDeserialize");
    let crate_common_code_occurence_code_occurence_with_deserialize_token_stream = 
    crate_common_code_occurence_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_common_code_occurence_code_occurence_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let get_code_occurence_stringified = "get_code_occurence";
    let get_code_occurence_token_stream = 
    get_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {get_code_occurence_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let get_code_occurence_with_deserialize_stringified = format!("{get_code_occurence_stringified}_with_deserialize");
    let get_code_occurence_with_deserialize_token_stream = 
    get_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {get_code_occurence_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"));
    let origin_or_wrapper = if ident_stringified.contains(WRAPPER_NAME)
        && ident_stringified.contains(ORIGIN_NAME)
    {
        panic!(
            "{proc_macro_name} {ident_stringified} contains {WRAPPER_NAME} and {ORIGIN_NAME}",
        );
    } else if ident_stringified.contains(WRAPPER_NAME) {
        OriginOrWrapper::Wrapper
    } else if ident_stringified.contains(ORIGIN_NAME) {
        OriginOrWrapper::Origin
    } else {
        panic!(
            "{proc_macro_name} {ident_stringified} does not contain {WRAPPER_NAME} or {ORIGIN_NAME}",
        );
    };
    let data_enum = match ast.data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => panic!("{proc_macro_name} {ident_stringified} only works with syn::Data::Enum"),
    };
    let mut all_equal: Option<SuportedEnumVariant> = None;
    for variant in &data_enum.variants {
        match &variant.fields {
            syn::Fields::Named(_) => {
                match &all_equal {
                    Some(supported_variant) => {
                        match supported_variant {
                            SuportedEnumVariant::Named => (),
                            SuportedEnumVariant::Unnamed => panic!("{proc_macro_name} {ident_stringified} only works with enums where all variants are syn::Fields::Named or all variants are syn::Fields::Unnamed"),
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
                            SuportedEnumVariant::Named => panic!("{proc_macro_name} {ident_stringified} only works with enums where all variants are syn::Fields::Named or all variants are syn::Fields::Unnamed"),
                            SuportedEnumVariant::Unnamed => (),
                        }
                    },
                    None => {
                        all_equal = Some(SuportedEnumVariant::Unnamed);
                    },
                }
            },
            syn::Fields::Unit => panic!("{proc_macro_name} {ident_stringified} only works with enums where all variants are syn::Fields::Named or all variants are syn::Fields::Unnamed"),
        }
    }
    let config_name_for_source_to_string_with_config = match origin_or_wrapper {
        OriginOrWrapper::Origin => {
            let underscore_config_stringified = String::from("_config");
            underscore_config_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_config_stringified} .parse::<proc_macro2::TokenStream>() failed"))
        },
        OriginOrWrapper::Wrapper => {
            let config_stringified = String::from("config");
            config_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {config_stringified} .parse::<proc_macro2::TokenStream>() failed"))
        },
    };
    let supported_enum_variant = match all_equal {
        Some(supported_enum_variant) => supported_enum_variant,
        None => panic!("{proc_macro_name} {ident_stringified} only works with enums where variants named first field name is member of {:?}", ErrorFieldName::to_all_variants_lower_case_string_vec()),
    };
    let generated_impl_with_deserialize_alternatives = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            let vec_needed_info = {
                let mut vec_needed_info: Vec<(&proc_macro2::Ident, ErrorFieldName, &syn::Type, proc_macro2::Ident, &syn::Type)> = Vec::with_capacity(data_enum.variants.len());
                data_enum.variants.iter().for_each(|variant| {
                    let variant_ident = &variant.ident;
                    let needed_info = match &variant.fields {
                        syn::Fields::Named(fields_named) => {
                            let named = &fields_named.named;
                            match named.len() == 2 {
                                true => {
                                    let first_field = &named[0];
                                    let first_field_ident =
                                        first_field.ident.clone()
                                        .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} SuportedEnumVariant::Named syn::Fields::Named first_field_ident is None"));
                                    let error_field_name = if first_field_ident == *"error" {
                                        ErrorFieldName::Error
                                    } else if first_field_ident == *"inner_error" {
                                        ErrorFieldName::InnerError
                                    } else if first_field_ident == *"inner_errors" {
                                        ErrorFieldName::InnerErrors
                                    } else {
                                        panic!("{proc_macro_name} {ident_stringified} only works with enums where variants named first field name is member of {:?}", ErrorFieldName::to_all_variants_lower_case_string_vec());
                                    };
                                    let second_field = &named[1];
                                    let second_field_ident =
                                        second_field.ident.clone()
                                        .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} SuportedEnumVariant::Named syn::Fields::Named second_field_ident is None"));
                                    if second_field_ident != *"code_occurence" {
                                        panic!("{proc_macro_name} {ident_stringified} only works on enums where variants named second field name == code_occurence");
                                    }
                                    (error_field_name, &first_field.ty, second_field_ident, &second_field.ty)
                                },
                                false => panic!("{proc_macro_name} {ident_stringified} only works on named fields with length of 2"),
                            }
                        },
                        syn::Fields::Unnamed(_) => panic!("{proc_macro_name} {ident_stringified} expected field to be named"),
                        _ => panic!("{proc_macro_name} {ident_stringified} expected fields would be named"),
                    };
                    vec_needed_info.push((variant_ident, needed_info.0, needed_info.1, needed_info.2, needed_info.3));
                });
                vec_needed_info
            };
            match vec_needed_info.is_empty() {
                true => panic!("{proc_macro_name} {ident_stringified} vec_needed_info is empty"),
                false => (),
            }
            let logic_for_source_to_string_with_config = match &origin_or_wrapper {
                OriginOrWrapper::Origin => quote::quote! {
                    use #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream;
                    self.#source_to_string_without_config_token_stream()
                },
                OriginOrWrapper::Wrapper => {
                    let generated_variants_logic = vec_needed_info.iter().map(|(
                        variant_ident, 
                        error_field_name, 
                        _first_field_type,
                        second_field_ident, 
                        _second_field_type
                    )|{
                        let error_field_name_stringified = error_field_name.to_lower_snake_case();
                        let error_field_name_token_stream = error_field_name_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                        match error_field_name {
                            ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                            ErrorFieldName::InnerError => {
                                quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _code_occurence,
                                    } => {
                                        use #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream;
                                        #error_field_name_token_stream.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
                                    },
                                }
                            },
                            ErrorFieldName::InnerErrors => {
                                quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _code_occurence,
                                    } => {
                                        use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
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
                        let error_field_name_stringified = error_field_name.to_lower_snake_case();
                        let error_field_name_token_stream = error_field_name_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                        match error_field_name {
                            ErrorFieldName::Error => {
                                quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _code_occurence,
                                    } => #error_field_name_token_stream.to_string(),
                                }
                            },
                            ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
                            ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
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
                        let error_field_name_stringified = error_field_name.to_lower_snake_case();
                        let error_field_name_token_stream = error_field_name_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                        match error_field_name {
                            ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                            ErrorFieldName::InnerError => {
                                quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _code_occurence,
                                    } => {
                                        #use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream
                                        #error_field_name_token_stream.#to_string_without_config_token_stream()
                                    },
                                }
                            },
                            ErrorFieldName::InnerErrors => {
                                quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _code_occurence,
                                    } => {
                                        use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream;
                                        #error_field_name_token_stream.#few_to_string_without_config_token_stream()
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
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let underscore_error_field_name_stringified = format!("_{error_field_name}");
                                let error_field_name_underscore_token_stream = underscore_error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                quote::quote!{
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                }
                            },
                            ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
                            ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
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
                            ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                            ErrorFieldName::InnerError => {
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let underscore_error_field_name_stringified = format!("_{error_field_name}");
                                let error_field_name_underscore_token_stream = underscore_error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                quote::quote!{
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                }
                            },
                            ErrorFieldName::InnerErrors => {
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let underscore_error_field_name_stringified = format!("_{error_field_name}");
                                let error_field_name_underscore_token_stream = underscore_error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
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
                        second_field_type
                    )|{
                        match error_field_name {
                            ErrorFieldName::Error => {
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let second_field_ident_token_stream = form_code_occurence_deserialize(second_field_type, proc_macro_name, &ident_stringified);
                                quote::quote!{
                                    #variant_ident {
                                        #error_field_name_token_stream: #first_field_type,
                                        #[serde(borrow)]
                                        #second_field_ident: #second_field_ident_token_stream
                                    },
                                }
                            },
                            ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
                            ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
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
                        second_field_type
                    )|{
                        match error_field_name {
                            ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                            ErrorFieldName::InnerError => {
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
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
                                    _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path"),
                                };
                                let first_field_type_token_stream = first_field_type_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let second_field_ident_token_stream = form_code_occurence_deserialize(second_field_type, proc_macro_name, &ident_stringified);
                                quote::quote!{
                                    #variant_ident {
                                        #[serde(borrow)]
                                        #error_field_name_token_stream: #first_field_type_token_stream,
                                        #[serde(borrow)]
                                        #second_field_ident: #second_field_ident_token_stream
                                    },
                                }
                            },
                            ErrorFieldName::InnerErrors => {
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let first_field_type_stringified = match first_field_type {
                                    syn::Type::Path(type_path) => {
                                        let supported_inner_errors_container =  match type_path.path.segments.last() {
                                            Some(path_segment) => {
                                                if path_segment.ident == "Vec" {
                                                    SupportedInnerErrorsContainers::Vec
                                                }
                                                else if path_segment.ident == "HashMap" {
                                                    SupportedInnerErrorsContainers::HashMap
                                                }
                                                else {
                                                    SupportedInnerErrorsContainers::Other
                                                }
                                            },
                                            None => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() is None"),
                                        };
                                        let first_field_type_prep = match supported_inner_errors_container {
                                            SupportedInnerErrorsContainers::Vec => {
                                                let mut vec_checker: Option<()> = None;
                                                let type_path_path_segments_stringified = type_path.path.segments.iter()
                                                .fold(String::from(""), |mut acc, elem| {
                                                    let elem_ident = &elem.ident;
                                                    if *elem_ident == "Vec" {
                                                        if vec_checker.is_some() {
                                                            panic!("{proc_macro_name} {ident_stringified} first_field_type detected more than one Vec inside type path");
                                                        }
                                                        match &elem.arguments {
                                                            syn::PathArguments::None => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::None for Vec"),
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
                                                                                        acc.push_str(&format!("Vec<{segments_stringified}WithDeserialize<'a>>"))
                                                                                    },
                                                                                    _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for Vec"),
                                                                                }
                                                                            },
                                                                            _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for Vec"),
                                                                        }
                                                                    },
                                                                    false => panic!("{proc_macro_name} {ident_stringified} works only with angle_bracketed.args.len() == 1 for Vec"),
                                                                }
                                                            },
                                                            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::Parenthesized for Vec"),
                                                        }
                                                        vec_checker = Some(());
                                                    }
                                                    else {
                                                        acc.push_str(&format!("{elem_ident}::"));
                                                    }
                                                    acc
                                                });
                                                type_path_path_segments_stringified
                                            },
                                            SupportedInnerErrorsContainers::HashMap => {
                                                let mut hashmap_checker: Option<()> = None;
                                                let type_path_path_segments_stringified = type_path.path.segments.iter()
                                                .fold(String::from(""), |mut acc, elem| {
                                                    let elem_ident = &elem.ident;
                                                    if *elem_ident == "HashMap" {
                                                        if hashmap_checker.is_some() {
                                                            panic!("{proc_macro_name} {ident_stringified} first_field_type detected more than one HashMap inside type path");
                                                        }
                                                        match &elem.arguments {
                                                            syn::PathArguments::None => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::None for HashMap"),
                                                            syn::PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => {
                                                                match angle_bracketed_generic_arguments.args.len() == 2 {
                                                                    true => {
                                                                        let hashmap_key = match &angle_bracketed_generic_arguments.args[0] {
                                                                            syn::GenericArgument::Lifetime(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
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
                                                                                    _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for HashMap"),
                                                                                }
                                                                            },
                                                                            syn::GenericArgument::Const(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
                                                                            syn::GenericArgument::Binding(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
                                                                            syn::GenericArgument::Constraint(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
                                                                        };
                                                                        let hashmap_value = match &angle_bracketed_generic_arguments.args[1] {
                                                                            syn::GenericArgument::Lifetime(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
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
                                                                                    _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for HashMap"),
                                                                                }
                                                                            },
                                                                            syn::GenericArgument::Const(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
                                                                            syn::GenericArgument::Binding(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
                                                                            syn::GenericArgument::Constraint(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
                                                                        };
                                                                        acc.push_str(&format!("{elem_ident}<{hashmap_key},{hashmap_value}>"));
                                                                    },
                                                                    false => panic!("{proc_macro_name} {ident_stringified} works only with angle_bracketed_generic_arguments.args.len() == 2 for HashMap"),
                                                                }
                                                            },
                                                            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::Parenthesized for HashMap"),
                                                        }
                                                        hashmap_checker = Some(());
                                                    }
                                                    else {
                                                        acc.push_str(&format!("{elem_ident}::"));
                                                    }
                                                    acc
                                                });
                                                type_path_path_segments_stringified
                                            },
                                            SupportedInnerErrorsContainers::Other => {
                                                let mut type_path_path_segments_stringified = type_path.path.segments.iter()
                                                .fold(String::from(""), |mut acc, elem| {
                                                    let elem_ident = &elem.ident;
                                                    acc.push_str(&format!("{elem_ident}::"));
                                                    acc
                                                });
                                                type_path_path_segments_stringified.pop();
                                                type_path_path_segments_stringified.pop();
                                                type_path_path_segments_stringified
                                            },
                                        };
                                        first_field_type_prep
                                    },
                                    _ => panic!("{proc_macro_name} {ident_stringified} first_field_type supports only syn::Type::Path"),
                                };
                                let first_field_type_with_deserialize_token_stream = first_field_type_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let second_field_ident_token_stream = form_code_occurence_deserialize(second_field_type, proc_macro_name, &ident_stringified);
                                quote::quote!{
                                    #variant_ident {
                                        #[serde(borrow)]
                                        #error_field_name_token_stream: #first_field_type_with_deserialize_token_stream,
                                        #[serde(borrow)]
                                        #second_field_ident: #second_field_ident_token_stream
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
                        let error_field_name_stringified = error_field_name.to_lower_snake_case();
                        let error_field_name_token_stream = error_field_name_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                        let underscore_second_field_ident = format!("_{second_field_ident}");
                        let second_field_ident_underscore_token_stream = underscore_second_field_ident
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_second_field_ident} .parse::<proc_macro2::TokenStream>() failed"));
                        match error_field_name {
                            ErrorFieldName::Error => {
                                quote::quote! {
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: #second_field_ident_underscore_token_stream,
                                    } => format!("{}", #error_field_name_token_stream),
                                }
                            },
                            ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
                            ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
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
                        let error_field_name_stringified = error_field_name.to_lower_snake_case();
                        let error_field_name_token_stream = error_field_name_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                        let underscore_second_field_ident_stringified = format!("_{second_field_ident}");
                        let second_field_ident_underscore_token_stream = underscore_second_field_ident_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_second_field_ident_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                        match error_field_name {
                            ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                            ErrorFieldName::InnerError => quote::quote! {
                                #ident_with_deserialize_token_stream::#variant_ident {
                                    #error_field_name_token_stream,
                                    #second_field_ident: #second_field_ident_underscore_token_stream,
                                } => {
                                    #use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream
                                    #error_field_name_token_stream.#to_string_without_config_with_deserialize_token_stream()
                                }
                            },
                            ErrorFieldName::InnerErrors => quote::quote! {
                                #ident_with_deserialize_token_stream::#variant_ident {
                                    #error_field_name_token_stream,
                                    #second_field_ident: #second_field_ident_underscore_token_stream,
                                } => {
                                    use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream;
                                    #error_field_name_token_stream.#few_to_string_without_config_with_deserialize_token_stream()
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
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let underscore_error_field_name_stringified = format!("_{error_field_name}");
                                let error_field_name_underscore_token_stream = underscore_error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                quote::quote!{
                                     #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                }
                            },
                            ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
                            ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
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
                            ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                            ErrorFieldName::InnerError => {
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let underscore_error_field_name_stringified = format!("_{error_field_name}");
                                let error_field_name_underscore_token_stream = underscore_error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                quote::quote!{
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream: #error_field_name_underscore_token_stream,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                }
                            },
                            ErrorFieldName::InnerErrors => {
                                let error_field_name_stringified = error_field_name.to_lower_snake_case();
                                let error_field_name_token_stream = error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
                                let underscore_error_field_name_stringified = format!("_{error_field_name}");
                                let error_field_name_underscore_token_stream = underscore_error_field_name_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_error_field_name_stringified} .parse::<proc_macro2::TokenStream>() failed"));
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
                impl<'a, ConfigGeneric>
                    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
                        'a,
                        ConfigGeneric,
                    > for #ident<'a>
                    where ConfigGeneric: #crate_traits_fields_get_source_place_type_token_stream
                        + #crate_traits_fields_get_timezone_token_stream
                        + #crate_traits_get_server_address_get_server_address_token_stream,
                {
                    fn source_to_string_with_config(
                        &self,
                        #config_name_for_source_to_string_with_config: &ConfigGeneric
                    ) -> String {
                        #logic_for_source_to_string_with_config
                    }
                }
                impl<'a>
                    #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<
                        'a,
                    > for #ident<'a>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config
                        }
                    }
                }
                impl<'a> #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream<'a>
                    for #ident<'a>
                {
                    fn #get_code_occurence_token_stream(&self) -> &#crate_common_code_occurence_code_occurence_token_stream<'a> {
                        match self {
                            #logic_for_get_code_occurence
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
                pub enum #ident_with_deserialize_token_stream<'a> {
                    #logic_for_enum_with_deserialize
                }
                impl<'a> #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<'a,> for #ident_with_deserialize_token_stream<'a>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config_with_deserialize
                        }
                    }
                }
                impl<'a> #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_token_stream<'a>
                    for #ident_with_deserialize_token_stream<'a>
                {
                    fn #get_code_occurence_with_deserialize_token_stream(
                        &self,
                    ) -> &#crate_common_code_occurence_code_occurence_with_deserialize_token_stream<'a> {
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
                        syn::Fields::Named(_) => panic!("{proc_macro_name} {ident_stringified} unexpected named unnamed logic"),
                        syn::Fields::Unnamed(fields_unnamed) => {
                            let unnamed = &fields_unnamed.unnamed;
                            if let false = unnamed.len() == 1 {
                                panic!("{proc_macro_name} {ident_stringified} SuportedEnumVariant::Unnamed variant fields unnamed len != 1");
                            }
                            &unnamed[0].ty
                        },
                        _ => panic!("{proc_macro_name} {ident_stringified} only works with named fields"),
                    };
                    vec_needed_info.push((&variant.ident, needed_info));
                });
                vec_needed_info
            };
            if let true = vec_needed_info.is_empty() {
                panic!("{proc_macro_name} {ident_stringified} enum variants are empty");
            }
            let logic_for_to_string_with_config_for_source_to_string_with_config = {
                let generated_variants_logic = vec_needed_info.iter().map(|(
                    variant_ident, 
                    first_field_type, 
                )|{
                    let gen = match first_field_type {
                        syn::Type::Path(type_path) => {
                            let last_segment_ident = type_path.path.segments.last()
                            .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} no last segment in type_path.path.segments"))
                            .ident.to_string();
                            let origin_or_wrapper = match (last_segment_ident.contains(WRAPPER_NAME), last_segment_ident.contains(ORIGIN_NAME)) {
                                (true, true) => panic!("{proc_macro_name} {ident_stringified} last_segment_ident contains Wrapper and Origin"),
                                (true, false) => OriginOrWrapper::Wrapper,
                                (false, true) => OriginOrWrapper::Origin,
                                (false, false) => panic!("{proc_macro_name} {ident_stringified} last_segment_ident do not contain Wrapper or Origin"),
                            };
                            match origin_or_wrapper {
                                OriginOrWrapper::Origin => quote::quote! {
                                    use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithoutConfig;
                                    i.to_string_with_config_for_source_to_string_without_config(config)
                                },
                                OriginOrWrapper::Wrapper => quote::quote! {
                                    i.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
                                },
                            }
                        },
                        _ => panic!("{proc_macro_name} {ident_stringified} first_field_type supports only syn::Type::Path"),
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
            let logic_for_to_string_without_config = {
                let gen = vec_needed_info.iter().map(|(
                    variant_ident, 
                    _first_field_type, 
                )|
                    quote::quote!{
                        #ident::#variant_ident(i) => i.#to_string_without_config_token_stream()
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
                                let variant_type_with_deserialize_stringified = format!("{variant_type}WithDeserialize");
                                variant_type_with_deserialize_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {variant_type_with_deserialize_stringified} .parse::<proc_macro2::TokenStream>() failed"))                                             
                            },
                            _ => panic!("{proc_macro_name} {ident_stringified} first_field_type supports only syn::Type::Path"),
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
            let logic_for_to_string_without_config_with_deserialize = {
                let gen = vec_needed_info.iter().map(|(
                    variant_ident, 
                    _first_field_type, 
                )|
                    quote::quote!{
                        #ident_with_deserialize_token_stream::#variant_ident(i) => i.#to_string_without_config_with_deserialize_token_stream()
                    }
                );
                quote::quote! {
                    #(#gen),*
                }
            };
            quote::quote! {
                impl<'a, ConfigGeneric>
                    #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream<
                    'a,
                    ConfigGeneric,
                    > for #ident<'a>
                where
                    ConfigGeneric: #crate_traits_fields_get_source_place_type_token_stream
                    + #crate_traits_fields_get_timezone_token_stream
                    + #crate_traits_get_server_address_get_server_address_token_stream,
                {
                    fn #to_string_with_config_for_source_to_string_with_config_token_stream(&self, config: &ConfigGeneric) -> String {
                        match self {
                            #logic_for_to_string_with_config_for_source_to_string_with_config
                        }
                    }
                }
                impl<'a> #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream<'a>
                    for #ident<'a>
                {
                    fn #to_string_without_config_token_stream(&self) -> String {
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
                    #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream<
                        'a,
                    > for #ident_with_deserialize_token_stream<'a>
                {
                    fn #to_string_without_config_with_deserialize_token_stream(&self) -> String {
                        match self {
                            #logic_for_to_string_without_config_with_deserialize
                        }
                    }
                }
            }
        },
    };
    //todo - maybe add flag to implement display or not
    quote::quote! {
        impl<'a> std::fmt::Display for #ident<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                #use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream
                write!(f, "{}", self.#to_string_without_config_token_stream())
            }
        }
        #generated_impl_with_deserialize_alternatives
        impl<'a> std::fmt::Display for #ident_with_deserialize_token_stream<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                #use_crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream
                write!(f, "{}", self.#to_string_without_config_with_deserialize_token_stream())
            }
        }
    }.into()
}

fn form_code_occurence_deserialize(second_field_type: &syn::Type, proc_macro_name: &str, ident_stringified: &String) -> proc_macro2::TokenStream {
    let second_field_ident_prep = match second_field_type {
        syn::Type::Path(type_path) => {
            match type_path.path.segments.last() {
                Some(path_segment) => {
                    if let false = path_segment.ident == "CodeOccurence" {
                        panic!("{proc_macro_name} {ident_stringified} second_field_ident type_path.path.segments.last() != CodeOccurence");
                    }
                },
                None => panic!("{proc_macro_name} {ident_stringified} second_field_ident type_path.path.segments.last() is None"),
            }
            let mut code_occurence_checker: Option<()> = None;
            let second_field_ident_segments_stringified = type_path.path.segments.iter()
            .fold(String::from(""), |mut acc, path_segment| {
                let path_segment_ident = &path_segment.ident;
                if *path_segment_ident == "CodeOccurence" {
                    match code_occurence_checker {
                        Some(_) => panic!("{proc_macro_name} {ident_stringified} second_field_ident detected more than one CodeOccurence inside type path"),
                        None => {
                            acc.push_str(&format!("{path_segment_ident}WithDeserialize<'a>"));
                            code_occurence_checker = Some(());
                        },
                    }
                }
                else {
                    acc.push_str(&format!("{path_segment_ident}::"));
                }
                acc
            });
            if code_occurence_checker.is_none() {
                panic!("{proc_macro_name} {ident_stringified} no CodeOccurence detected inside second_field_ident type path");
            }
            second_field_ident_segments_stringified
        },
        _ => panic!("{proc_macro_name} {ident_stringified} second_field_type supports only syn::Type::Path"),
    };
    second_field_ident_prep
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {second_field_ident_prep} .parse::<proc_macro2::TokenStream>() failed"))
}