// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]
use convert_case::Casing;
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
//todo use macro #[origin] #[wrapper]
//tood or maybe use just first field error, inner_error, inner_errors and annotation for #[from_origin] for unnamed
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
    let parse_proc_macro2_token_stream_failed_message = ".parse::<proc_macro2::TokenStream>() failed";
    let lifetime_stringified = "'a";
    let lifetime_token_stream = lifetime_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {lifetime_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let with_deserialize_camel_case = "WithDeserialize";
    let with_deserialize_lower_case = with_deserialize_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let ident_with_deserialize_stringified = format!("{ident}{with_deserialize_camel_case}");
    let ident_with_deserialize_token_stream = ident_with_deserialize_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {ident_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let config_generic_token_stream = "ConfigGeneric"
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {ident_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_with_config_camel_case = "ToStringWithConfig";
    let to_string_with_config_lower_case = to_string_with_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_with_config_camel_case = format!("Source{to_string_with_config_camel_case}");
    let to_string_without_config_camel_case = "ToStringWithoutConfig";
    let to_string_without_config_lower_case = to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_without_config_camel_case = format!("Source{to_string_without_config_camel_case}");
    let source_to_string_without_config_lower_case = source_to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_without_config_token_stream = 
    source_to_string_without_config_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {source_to_string_without_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let code_occurence_camel_case = "CodeOccurence";
    let code_occurence_lower_case = code_occurence_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let get_code_occurence_lower_case = format!("get_{code_occurence_lower_case}");
    let crate_traits_fields_stringified = "crate::traits::fields::";
    let crate_traits_error_logs_logic_stringified = "crate::traits::error_logs_logic::";
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{to_string_without_config_lower_case}::{to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified}{with_deserialize_camel_case}");
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_fields_get_source_place_type_stringified = format!("{crate_traits_fields_stringified}GetSourcePlaceType");
    let crate_traits_fields_get_source_place_type_token_stream = 
    crate_traits_fields_get_source_place_type_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_fields_get_source_place_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_fields_get_timezone_stringified = format!("{crate_traits_fields_stringified}GetTimezone");
    let crate_traits_fields_get_timezone_token_stream = 
    crate_traits_fields_get_timezone_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_fields_get_timezone_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_get_server_address_get_server_address_stringified = "crate::traits::get_server_address::GetServerAddress";
    let crate_traits_get_server_address_get_server_address_token_stream = 
    crate_traits_get_server_address_get_server_address_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_get_server_address_get_server_address_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{source_to_string_without_config_lower_case}::{source_to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream = 
    crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{to_string_with_config_lower_case}::{to_string_with_config_camel_case}For{source_to_string_with_config_camel_case}");
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let few_to_string_without_config_stringified = format!("few_{to_string_without_config_lower_case}");
    let few_to_string_without_config_token_stream = 
    few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {few_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let few_to_string_without_config_with_deserialize_stringified = format!("{few_to_string_without_config_stringified}_{with_deserialize_lower_case}");
    let few_to_string_without_config_with_deserialize_token_stream = 
    few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {few_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{few_to_string_without_config_stringified}::Few{to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream = 
    crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified}{with_deserialize_camel_case}");
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream = 
    crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let source_to_string_with_config_stringified = format!("source_{to_string_with_config_lower_case}");
    let source_to_string_with_config_token_stream = 
    source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_with_config_for_source_to_string_with_config_stringified = format!("{to_string_with_config_lower_case}_for_{source_to_string_with_config_stringified}");
    let to_string_with_config_for_source_to_string_with_config_token_stream = 
    to_string_with_config_for_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {to_string_with_config_for_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_without_config_token_stream = 
    to_string_without_config_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {to_string_without_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_without_config_with_deserialize_stringified = format!("{to_string_without_config_lower_case}_{with_deserialize_lower_case}");
    let to_string_without_config_with_deserialize_token_stream = 
    to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified = format!("{crate_traits_error_logs_logic_stringified}{get_code_occurence_lower_case}::Get{code_occurence_camel_case}");
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified}{with_deserialize_camel_case}");
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_code_occurence_code_occurence_stringified = format!("crate::common::{code_occurence_lower_case}::{code_occurence_camel_case}");
    let crate_common_code_occurence_code_occurence_token_stream = 
    crate_common_code_occurence_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_common_code_occurence_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_code_occurence_code_occurence_with_deserialize_stringified = format!("{crate_common_code_occurence_code_occurence_stringified}{with_deserialize_camel_case}");
    let crate_common_code_occurence_code_occurence_with_deserialize_token_stream = 
    crate_common_code_occurence_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_common_code_occurence_code_occurence_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let get_code_occurence_token_stream = 
    get_code_occurence_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {get_code_occurence_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let get_code_occurence_with_deserialize_stringified = format!("{get_code_occurence_lower_case}_{with_deserialize_lower_case}");
    let get_code_occurence_with_deserialize_token_stream = 
    get_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {get_code_occurence_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{to_string_with_config_lower_case}::{to_string_with_config_camel_case}For{source_to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_without_config_token_stream = 
    crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_with_config_for_source_to_string_without_config_stringified = format!("{to_string_with_config_lower_case}_for_{source_to_string_without_config_lower_case}");
    let to_string_with_config_for_source_to_string_without_config_token_stream = 
    to_string_with_config_for_source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {to_string_with_config_for_source_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let few_to_string_with_config_stringified = format!("few_{to_string_with_config_lower_case}");
    let few_to_string_with_config_token_stream = 
    few_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {few_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{few_to_string_with_config_stringified}::Few{to_string_with_config_camel_case}");
    let crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{source_to_string_with_config_stringified}::{source_to_string_with_config_camel_case}");
    let crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_stringified} {crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
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
    let supported_enum_variant = match all_equal {
        Some(supported_enum_variant) => supported_enum_variant,
        None => panic!("{proc_macro_name} {ident_stringified} only works with enums where variants named first field name is member of {:?}", ErrorFieldName::to_all_variants_lower_case_string_vec()),
    };
    // let generated_impl_with_deserialize_alternatives_handle = match (supported_enum_variant, origin_or_wrapper) {
    //     (SuportedEnumVariant::Named, OriginOrWrapper::Origin) => {
    //         //duplicate
    //         let config_name_for_source_to_string_with_config = {
    //             let underscore_config_stringified = "_config";
    //             underscore_config_stringified.parse::<proc_macro2::TokenStream>()
    //             .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_config_stringified} {parse_proc_macro2_token_stream_failed_message}"))
    //         };
    //         //duplicate
    //         let vec_needed_info = {
    //             let mut vec_needed_info_prep: Vec<(&proc_macro2::Ident, ErrorFieldName, &syn::Type, proc_macro2::Ident, &syn::Type, proc_macro2::TokenStream)> = Vec::with_capacity(data_enum.variants.len());
    //             data_enum.variants.iter().for_each(|variant| {
    //                 let variant_ident = &variant.ident;
    //                 let needed_info = match &variant.fields {
    //                     syn::Fields::Named(fields_named) => {
    //                         let named = &fields_named.named;
    //                         if let false = named.len() == 2 {
    //                             panic!("{proc_macro_name} {ident_stringified} only works on named fields with length of 2");
    //                         }
    //                         let first_field = &named[0];
    //                         let first_field_ident =
    //                             first_field.ident.clone()
    //                             .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} SuportedEnumVariant::Named syn::Fields::Named first_field_ident is None"));
    //                         let error_field_name = if first_field_ident == *"error" {
    //                             ErrorFieldName::Error
    //                         } else if first_field_ident == *"inner_error" {
    //                             ErrorFieldName::InnerError
    //                         } else if first_field_ident == *"inner_errors" {
    //                             ErrorFieldName::InnerErrors
    //                         } else {
    //                             panic!("{proc_macro_name} {ident_stringified} only works with enums where variants named first field name is member of {:?}", ErrorFieldName::to_all_variants_lower_case_string_vec());
    //                         };
    //                         let second_field = &named[1];
    //                         let second_field_ident =
    //                             second_field.ident.clone()
    //                             .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} SuportedEnumVariant::Named syn::Fields::Named second_field_ident is None"));
    //                         if second_field_ident != *code_occurence_lower_case {
    //                             panic!("{proc_macro_name} {ident_stringified} only works on enums where variants named second field name == {code_occurence_lower_case}");
    //                         }
    //                         let error_field_name_stringified = error_field_name.to_lower_snake_case();
    //                         let error_field_name_token_stream = error_field_name_stringified
    //                         .parse::<proc_macro2::TokenStream>()
    //                         .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    //                         (error_field_name, &first_field.ty, second_field_ident, &second_field.ty, error_field_name_token_stream)
    //                     },
    //                     syn::Fields::Unnamed(_) => panic!("{proc_macro_name} {ident_stringified} expected field to be named"),
    //                     _ => panic!("{proc_macro_name} {ident_stringified} expected fields would be named"),
    //                 };
    //                 vec_needed_info_prep.push((variant_ident, needed_info.0, needed_info.1, needed_info.2, needed_info.3, needed_info.4));
    //             });
    //             match vec_needed_info_prep.is_empty() {
    //                 true => panic!("{proc_macro_name} {ident_stringified} vec_needed_info is empty"),
    //                 false => (),
    //             }
    //             vec_needed_info_prep
    //         };
    //         let logic_for_source_to_string_with_config = quote::quote! {
    //             use #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream;
    //             self.#source_to_string_without_config_token_stream()
    //         };
    //         let logic_for_source_to_string_without_config = {
    //             let generated_variants_logic = vec_needed_info.iter().map(|(
    //                 variant_ident, 
    //                 error_field_name, 
    //                 _first_field_type,
    //                 second_field_ident, 
    //                 _second_field_type,
    //                 error_field_name_token_stream
    //             )|{
    //                 match error_field_name {
    //                     ErrorFieldName::Error => {
    //                         quote::quote! {
    //                             #ident::#variant_ident {
    //                                 #error_field_name_token_stream,
    //                             #second_field_ident: _unused_second_argument,
    //                             } => #error_field_name_token_stream.to_string(),
    //                         }
    //                     },
    //                     ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
    //                     ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
    //                 }
    //             });
    //             quote::quote! {
    //                 #(#generated_variants_logic),*
    //             }
    //         };
    //         let logic_for_get_code_occurence = {
    //             let generated_variants_logic = vec_needed_info.iter().map(|(
    //                 variant_ident, 
    //                 error_field_name, 
    //                 _first_field_type,
    //                 second_field_ident, 
    //                 _second_field_type,
    //                 error_field_name_token_stream
    //             )|{
    //                 match error_field_name {
    //                     ErrorFieldName::Error => {
    //                         quote::quote!{
    //                             #ident::#variant_ident {
    //                                 #error_field_name_token_stream: _unused_first_argument,
    //                                 #second_field_ident,
    //                             } => #second_field_ident,
    //                         }
    //                     },
    //                     ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
    //                     ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
    //                 }
    //             });
    //             quote::quote! {
    //                 #(#generated_variants_logic),*
    //             }
    //         };
    //         let logic_for_enum_with_deserialize = {
    //             let generated_variants_logic = vec_needed_info.iter().map(|(
    //                 variant_ident, 
    //                 error_field_name, 
    //                 first_field_type,
    //                 second_field_ident, 
    //                 second_field_type,
    //                 error_field_name_token_stream
    //             )|{
    //                 match error_field_name {
    //                     ErrorFieldName::Error => {
    //                         let second_field_ident_token_stream = form_code_occurence_deserialize(
    //                             second_field_type, 
    //                             proc_macro_name, 
    //                             &ident_stringified, 
    //                             with_deserialize_camel_case, 
    //                             parse_proc_macro2_token_stream_failed_message,
    //                             code_occurence_camel_case,
    //                             lifetime_stringified
    //                         );
    //                         quote::quote!{
    //                             #variant_ident {
    //                                 #error_field_name_token_stream: #first_field_type,
    //                                 #[serde(borrow)]
    //                                 #second_field_ident: #second_field_ident_token_stream
    //                             },
    //                         }
    //                     },
    //                     ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
    //                     ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
    //                 }
    //             });
    //             quote::quote! {
    //                 #(#generated_variants_logic),*
    //             }
    //         };
    //         let logic_for_source_to_string_without_config_with_deserialize = {
    //             let generated_variants_logic = vec_needed_info.iter().map(|(
    //                 variant_ident, 
    //                 error_field_name, 
    //                 first_field_type,
    //                 second_field_ident, 
    //                 second_field_type,
    //                 error_field_name_token_stream
    //             )|{
    //                 match error_field_name {
    //                     ErrorFieldName::Error => {
    //                         let second_field_ident_token_stream = form_code_occurence_deserialize(
    //                             second_field_type, 
    //                             proc_macro_name, 
    //                             &ident_stringified, 
    //                             with_deserialize_camel_case, 
    //                             parse_proc_macro2_token_stream_failed_message,
    //                             code_occurence_camel_case,
    //                             lifetime_stringified
    //                         );
    //                         quote::quote!{
    //                             #variant_ident {
    //                                 #error_field_name_token_stream: #first_field_type,
    //                                 #[serde(borrow)]
    //                                 #second_field_ident: #second_field_ident_token_stream
    //                             },
    //                         }
    //                     },
    //                     ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
    //                     ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
    //                 }
    //             });
    //             quote::quote! {
    //                 #(#generated_variants_logic),*
    //             }
    //         };
    //         let logic_for_get_code_occurence_with_deserialize = {
    //             let generated_variants_logic = vec_needed_info.iter().map(|(
    //                 variant_ident, 
    //                 error_field_name, 
    //                 _first_field_type,
    //                 second_field_ident, 
    //                 _second_field_type,
    //                 error_field_name_token_stream
    //             )|{
    //                 match error_field_name {
    //                     ErrorFieldName::Error => {
    //                         quote::quote!{
    //                              #ident_with_deserialize_token_stream::#variant_ident {
    //                                 #error_field_name_token_stream: _unused_first_argument,
    //                                 #second_field_ident,
    //                             } => #second_field_ident,
    //                         }
    //                     },
    //                     ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
    //                     ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
    //                 }
    //             });
    //             quote::quote! {
    //                 #(#generated_variants_logic),*
    //             }
    //         };
    //     },
    //     (SuportedEnumVariant::Named, OriginOrWrapper::Wrapper) => {

    //     },
    //     (SuportedEnumVariant::Unnamed, OriginOrWrapper::Origin) => {

    //     },
    //     (SuportedEnumVariant::Unnamed, OriginOrWrapper::Wrapper) => {

    //     },
    // };
    let generated_impl_with_deserialize_alternatives = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            let vec_needed_info = {
                let mut vec_needed_info_prep: Vec<(&proc_macro2::Ident, ErrorFieldName, &syn::Type, proc_macro2::Ident, &syn::Type, proc_macro2::TokenStream)> = Vec::with_capacity(data_enum.variants.len());
                data_enum.variants.iter().for_each(|variant| {
                    let variant_ident = &variant.ident;
                    let needed_info = match &variant.fields {
                        syn::Fields::Named(fields_named) => {
                            let named = &fields_named.named;
                            if let false = named.len() == 2 {
                                panic!("{proc_macro_name} {ident_stringified} only works on named fields with length of 2");
                            }
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
                            if second_field_ident != *code_occurence_lower_case {
                                panic!("{proc_macro_name} {ident_stringified} only works on enums where variants named second field name == {code_occurence_lower_case}");
                            }
                            let error_field_name_stringified = error_field_name.to_lower_snake_case();
                            let error_field_name_token_stream = error_field_name_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            (error_field_name, &first_field.ty, second_field_ident, &second_field.ty, error_field_name_token_stream)
                        },
                        syn::Fields::Unnamed(_) => panic!("{proc_macro_name} {ident_stringified} expected field to be named"),
                        _ => panic!("{proc_macro_name} {ident_stringified} expected fields would be named"),
                    };
                    vec_needed_info_prep.push((variant_ident, needed_info.0, needed_info.1, needed_info.2, needed_info.3, needed_info.4));
                });
                match vec_needed_info_prep.is_empty() {
                    true => panic!("{proc_macro_name} {ident_stringified} vec_needed_info is empty"),
                    false => (),
                }
                vec_needed_info_prep
            };
            if let true = vec_needed_info.is_empty() {
                panic!("{proc_macro_name} {ident_stringified} enum variants are empty");
            }
            let (
                config_name_for_source_to_string_with_config,
                logic_for_source_to_string_with_config,
                logic_for_source_to_string_without_config,
                logic_for_get_code_occurence,
                logic_for_enum_with_deserialize,
                logic_for_source_to_string_without_config_with_deserialize,
                logic_for_get_code_occurence_with_deserialize
            ) =  match origin_or_wrapper {
                OriginOrWrapper::Origin => {
                    let config_name_for_source_to_string_with_config = {
                        let underscore_config_stringified = "_config";
                        underscore_config_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_config_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                    };
                    let logic_for_source_to_string_with_config = quote::quote! {
                        use #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream;
                        self.#source_to_string_without_config_token_stream()
                    };
                    let mut logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
                    let mut logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
                    let mut logic_for_enum_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
                    let mut logic_for_source_to_string_without_config_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
                    let mut logic_for_get_code_occurence_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
                    vec_needed_info.iter().for_each(|(
                        variant_ident, 
                        error_field_name, 
                        first_field_type,
                        second_field_ident, 
                        second_field_type,
                        error_field_name_token_stream
                    )|{
                        match error_field_name {
                            ErrorFieldName::Error => (),
                            ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
                            ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
                        }
                        logic_for_source_to_string_without_config.push(quote::quote! {
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => #error_field_name_token_stream.to_string(),
                        });
                        logic_for_get_code_occurence.push(quote::quote!{
                            #ident::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident,
                        });
                        logic_for_enum_with_deserialize.push({
                            let second_field_ident_token_stream = form_code_occurence_deserialize(
                                second_field_type, 
                                proc_macro_name, 
                                &ident_stringified, 
                                with_deserialize_camel_case, 
                                parse_proc_macro2_token_stream_failed_message,
                                code_occurence_camel_case,
                                lifetime_stringified
                            );
                            quote::quote!{
                                #variant_ident {
                                    #error_field_name_token_stream: #first_field_type,
                                    #[serde(borrow)]
                                    #second_field_ident: #second_field_ident_token_stream
                                },
                            }
                        });
                        logic_for_source_to_string_without_config_with_deserialize.push(quote::quote! {
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => #error_field_name_token_stream.to_string(),
                        });
                        logic_for_get_code_occurence_with_deserialize.push(quote::quote!{
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident,
                        });
                    });
                    let logic_for_source_to_string_without_config_generated = logic_for_source_to_string_without_config.iter();
                    let logic_for_get_code_occurence_generated = logic_for_get_code_occurence.iter();
                    let logic_for_enum_with_deserialize_generated = logic_for_enum_with_deserialize.iter();
                    let logic_for_source_to_string_without_config_with_deserialize_generated = logic_for_source_to_string_without_config_with_deserialize.iter();
                    let logic_for_get_code_occurence_with_deserialize_generated = logic_for_get_code_occurence_with_deserialize.iter();
                    let logic_for_source_to_string_without_config = quote::quote! {
                        #(#logic_for_source_to_string_without_config_generated),*
                    };
                    let logic_for_get_code_occurence = quote::quote! {
                        #(#logic_for_get_code_occurence_generated),*
                    };
                    let logic_for_enum_with_deserialize = quote::quote! {
                        #(#logic_for_enum_with_deserialize_generated),*
                    };
                    let logic_for_source_to_string_without_config_with_deserialize = quote::quote! {
                        #(#logic_for_source_to_string_without_config_with_deserialize_generated),*
                    };
                    let logic_for_get_code_occurence_with_deserialize = quote::quote! {
                        #(#logic_for_get_code_occurence_with_deserialize_generated),*
                    };
                    (
                        config_name_for_source_to_string_with_config,
                        logic_for_source_to_string_with_config,
                        logic_for_source_to_string_without_config,
                        logic_for_get_code_occurence,
                        logic_for_enum_with_deserialize,
                        logic_for_source_to_string_without_config_with_deserialize,
                        logic_for_get_code_occurence_with_deserialize
                    )
                },
                OriginOrWrapper::Wrapper => {
                    let config_name_for_source_to_string_with_config = {
                        let config_stringified = "config";
                        config_stringified.parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {config_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                    };
                    //
                    let logic_around_fields = vec_needed_info.iter().map(|(
                        variant_ident, 
                        error_field_name, 
                        first_field_type,
                        second_field_ident, 
                        second_field_type,
                        error_field_name_token_stream
                    )|{
                        match error_field_name {
                            ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                            ErrorFieldName::InnerError => {
                                let logic_for_source_to_string_with_config_handle = quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
                                    } => {
                                        use #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream;
                                        #error_field_name_token_stream.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
                                    },
                                };
                                let logic_for_source_to_string_without_config_handle = quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
                                    } => {
                                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                                        #error_field_name_token_stream.#to_string_without_config_token_stream()
                                    },
                                };
                                let logic_for_get_code_occurence_handle = quote::quote!{
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream: _unused_first_argument,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                };
                                let logic_for_enum_with_deserialize_handle = {
                                    let first_field_type_stringified = match first_field_type {
                                        syn::Type::Path(type_path_handle) => {
                                            let last_arg_option_lifetime = form_last_arg_lifetime(
                                                type_path_handle, 
                                                proc_macro_name, 
                                                &ident_stringified,
                                                lifetime_stringified,
                                            );
                                            let mut segments_stringified = type_path_handle.path.segments.iter()
                                            .fold(String::from(""), |mut acc, elem| {
                                                acc.push_str(&format!("{}::", elem.ident));
                                                acc
                                            });
                                            segments_stringified.pop();
                                            segments_stringified.pop();
                                            format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}")
                                        },
                                        _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path"),
                                    };
                                    let first_field_type_token_stream = first_field_type_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let second_field_ident_token_stream = form_code_occurence_deserialize(
                                        second_field_type, 
                                        proc_macro_name, 
                                        &ident_stringified, 
                                        with_deserialize_camel_case, 
                                        parse_proc_macro2_token_stream_failed_message,
                                        code_occurence_camel_case,
                                        lifetime_stringified
                                    );
                                    quote::quote!{
                                        #variant_ident {
                                            #[serde(borrow)]
                                            #error_field_name_token_stream: #first_field_type_token_stream,
                                            #[serde(borrow)]
                                            #second_field_ident: #second_field_ident_token_stream
                                        },
                                    }
                                };
                                let logic_for_source_to_string_without_config_with_deserialize_handle = quote::quote! {
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
                                    } => {
                                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream;
                                        #error_field_name_token_stream.#to_string_without_config_with_deserialize_token_stream()
                                    }
                                };
                                let logic_for_get_code_occurence_with_deserialize_handle = quote::quote!{
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream: _unused_first_argument,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                };
                                (
                                    logic_for_source_to_string_with_config_handle,
                                    logic_for_source_to_string_without_config_handle,
                                    logic_for_get_code_occurence_handle,
                                    logic_for_enum_with_deserialize_handle,
                                    logic_for_source_to_string_without_config_with_deserialize_handle,
                                    logic_for_get_code_occurence_with_deserialize_handle
                                )
                            },
                            ErrorFieldName::InnerErrors => {
                                let logic_for_source_to_string_with_config_handle = quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
                                    } => {
                                        use #crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_token_stream;
                                        #error_field_name_token_stream.#few_to_string_with_config_token_stream(config)
                                    },
                                };
                                let logic_for_source_to_string_without_config_handle = quote::quote! {
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
                                    } => {
                                        use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream;
                                        #error_field_name_token_stream.#few_to_string_without_config_token_stream()
                                    },
                                };
                                let logic_for_get_code_occurence_handle = quote::quote!{
                                    #ident::#variant_ident {
                                        #error_field_name_token_stream: _unused_first_argument,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                };
                                let logic_for_enum_with_deserialize_handle = {
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
                                                                                syn::GenericArgument::Type(type_handle) => {
                                                                                    match type_handle {
                                                                                        syn::Type::Path(type_path_handle) => {
                                                                                            let mut segments_stringified = type_path_handle.path.segments.iter()
                                                                                            .fold(String::from(""), |mut acc, elem| {
                                                                                                acc.push_str(&format!("{}::", elem.ident));
                                                                                                acc
                                                                                            });
                                                                                            segments_stringified.pop();
                                                                                            segments_stringified.pop();
                                                                                            acc.push_str(&format!("Vec<{segments_stringified}{with_deserialize_camel_case}<{lifetime_stringified}>>"))
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
                                                                                        syn::Type::Path(type_path_handle) => {
                                                                                            let last_arg_option_lifetime = form_last_arg_lifetime(
                                                                                                type_path_handle, 
                                                                                                proc_macro_name, 
                                                                                                &ident_stringified,
                                                                                                lifetime_stringified,
                                                                                            );
                                                                                            let mut segments_stringified = type_path_handle.path.segments.iter()
                                                                                            .fold(String::from(""), |mut acc, elem| {
                                                                                                acc.push_str(&format!("{}::", elem.ident));
                                                                                                acc
                                                                                            });
                                                                                            segments_stringified.pop();
                                                                                            segments_stringified.pop();
                                                                                            format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}")
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
                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let second_field_ident_token_stream = form_code_occurence_deserialize(
                                        second_field_type, 
                                        proc_macro_name, 
                                        &ident_stringified, 
                                        with_deserialize_camel_case, 
                                        parse_proc_macro2_token_stream_failed_message,
                                        code_occurence_camel_case,
                                        lifetime_stringified
                                    );
                                    quote::quote!{
                                        #variant_ident {
                                            #[serde(borrow)]
                                            #error_field_name_token_stream: #first_field_type_with_deserialize_token_stream,
                                            #[serde(borrow)]
                                            #second_field_ident: #second_field_ident_token_stream
                                        },
                                    }
                                };
                                let logic_for_source_to_string_without_config_with_deserialize_handle = quote::quote! {
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
                                    } => {
                                        use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream;
                                        #error_field_name_token_stream.#few_to_string_without_config_with_deserialize_token_stream()
                                    }
                                };
                                let logic_for_get_code_occurence_with_deserialize_handle = quote::quote!{
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream: _unused_first_argument,
                                        #second_field_ident,
                                    } => #second_field_ident,
                                };
                                (
                                    logic_for_source_to_string_with_config_handle,
                                    logic_for_source_to_string_without_config_handle,
                                    logic_for_get_code_occurence_handle,
                                    logic_for_enum_with_deserialize_handle,
                                    logic_for_source_to_string_without_config_with_deserialize_handle,
                                    logic_for_get_code_occurence_with_deserialize_handle
                                )
                            },
                        }
                    });
                    //
                    let logic_for_source_to_string_with_config = {
                        let generated_variants_logic = vec_needed_info.iter().map(|(
                            variant_ident, 
                            error_field_name, 
                            _first_field_type,
                            second_field_ident, 
                            _second_field_type,
                            error_field_name_token_stream
                        )|{
                            match error_field_name {
                                ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                                ErrorFieldName::InnerError => {
                                    quote::quote! {
                                        #ident::#variant_ident {
                                            #error_field_name_token_stream,
                                            #second_field_ident: _unused_second_argument,
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
                                            #second_field_ident: _unused_second_argument,
                                        } => {
                                            use #crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_token_stream;
                                            #error_field_name_token_stream.#few_to_string_with_config_token_stream(config)
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
                    };
                    let logic_for_source_to_string_without_config = {
                        let generated_variants_logic = vec_needed_info.iter().map(|(
                            variant_ident, 
                            error_field_name, 
                            _first_field_type,
                            second_field_ident, 
                            _second_field_type,
                            error_field_name_token_stream
                        )|{
                            match error_field_name {
                                ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                                ErrorFieldName::InnerError => {
                                    quote::quote! {
                                        #ident::#variant_ident {
                                            #error_field_name_token_stream,
                                            #second_field_ident: _unused_second_argument,
                                        } => {
                                            use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                                            #error_field_name_token_stream.#to_string_without_config_token_stream()
                                        },
                                    }
                                },
                                ErrorFieldName::InnerErrors => {
                                    quote::quote! {
                                        #ident::#variant_ident {
                                            #error_field_name_token_stream,
                                            #second_field_ident: _unused_second_argument,
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
                    };
                    let logic_for_get_code_occurence = {
                        let generated_variants_logic = vec_needed_info.iter().map(|(
                            variant_ident, 
                            error_field_name, 
                            _first_field_type,
                            second_field_ident, 
                            _second_field_type,
                            error_field_name_token_stream
                        )|{
                            match error_field_name {
                                ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                                ErrorFieldName::InnerError => {
                                    quote::quote!{
                                        #ident::#variant_ident {
                                            #error_field_name_token_stream: _unused_first_argument,
                                            #second_field_ident,
                                        } => #second_field_ident,
                                    }
                                },
                                ErrorFieldName::InnerErrors => {
                                    quote::quote!{
                                        #ident::#variant_ident {
                                            #error_field_name_token_stream: _unused_first_argument,
                                            #second_field_ident,
                                        } => #second_field_ident,
                                    }
                                },
                            }
                        });
                        quote::quote! {
                            #(#generated_variants_logic),*
                        }
                    };
                    let logic_for_enum_with_deserialize = {
                        let generated_variants_logic = vec_needed_info.iter().map(|(
                            variant_ident, 
                            error_field_name, 
                            first_field_type,
                            second_field_ident, 
                            second_field_type,
                            error_field_name_token_stream
                        )|{
                            match error_field_name {
                                ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                                ErrorFieldName::InnerError => {
                                    let first_field_type_stringified = match first_field_type {
                                        syn::Type::Path(type_path_handle) => {
                                            let last_arg_option_lifetime = form_last_arg_lifetime(
                                                type_path_handle, 
                                                proc_macro_name, 
                                                &ident_stringified,
                                                lifetime_stringified,
                                            );
                                            let mut segments_stringified = type_path_handle.path.segments.iter()
                                            .fold(String::from(""), |mut acc, elem| {
                                                acc.push_str(&format!("{}::", elem.ident));
                                                acc
                                            });
                                            segments_stringified.pop();
                                            segments_stringified.pop();
                                            format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}")
                                        },
                                        _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path"),
                                    };
                                    let first_field_type_token_stream = first_field_type_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let second_field_ident_token_stream = form_code_occurence_deserialize(
                                        second_field_type, 
                                        proc_macro_name, 
                                        &ident_stringified, 
                                        with_deserialize_camel_case, 
                                        parse_proc_macro2_token_stream_failed_message,
                                        code_occurence_camel_case,
                                        lifetime_stringified
                                    );
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
                                                                                syn::GenericArgument::Type(type_handle) => {
                                                                                    match type_handle {
                                                                                        syn::Type::Path(type_path_handle) => {
                                                                                            let mut segments_stringified = type_path_handle.path.segments.iter()
                                                                                            .fold(String::from(""), |mut acc, elem| {
                                                                                                acc.push_str(&format!("{}::", elem.ident));
                                                                                                acc
                                                                                            });
                                                                                            segments_stringified.pop();
                                                                                            segments_stringified.pop();
                                                                                            acc.push_str(&format!("Vec<{segments_stringified}{with_deserialize_camel_case}<{lifetime_stringified}>>"))
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
                                                                                        syn::Type::Path(type_path_handle) => {
                                                                                            let last_arg_option_lifetime = form_last_arg_lifetime(
                                                                                                type_path_handle, 
                                                                                                proc_macro_name, 
                                                                                                &ident_stringified,
                                                                                                lifetime_stringified,
                                                                                            );
                                                                                            let mut segments_stringified = type_path_handle.path.segments.iter()
                                                                                            .fold(String::from(""), |mut acc, elem| {
                                                                                                acc.push_str(&format!("{}::", elem.ident));
                                                                                                acc
                                                                                            });
                                                                                            segments_stringified.pop();
                                                                                            segments_stringified.pop();
                                                                                            format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}")
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
                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let second_field_ident_token_stream = form_code_occurence_deserialize(
                                        second_field_type, 
                                        proc_macro_name, 
                                        &ident_stringified, 
                                        with_deserialize_camel_case, 
                                        parse_proc_macro2_token_stream_failed_message,
                                        code_occurence_camel_case,
                                        lifetime_stringified
                                    );
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
                    };
                    let logic_for_source_to_string_without_config_with_deserialize = {
                        let generated_variants_logic = vec_needed_info.iter().map(|(
                            variant_ident, 
                            error_field_name, 
                            _first_field_type,
                            second_field_ident, 
                            _second_field_type,
                            error_field_name_token_stream
                        )|{
                            match error_field_name {
                                ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                                ErrorFieldName::InnerError => quote::quote! {
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
                                    } => {
                                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream;
                                        #error_field_name_token_stream.#to_string_without_config_with_deserialize_token_stream()
                                    }
                                },
                                ErrorFieldName::InnerErrors => quote::quote! {
                                    #ident_with_deserialize_token_stream::#variant_ident {
                                        #error_field_name_token_stream,
                                        #second_field_ident: _unused_second_argument,
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
                    };
                    let logic_for_get_code_occurence_with_deserialize = {
                        let generated_variants_logic = vec_needed_info.iter().map(|(
                            variant_ident, 
                            error_field_name, 
                            _first_field_type,
                            second_field_ident, 
                            _second_field_type,
                            error_field_name_token_stream
                        )|{
                            match error_field_name {
                                ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
                                ErrorFieldName::InnerError => {
                                    quote::quote!{
                                        #ident_with_deserialize_token_stream::#variant_ident {
                                            #error_field_name_token_stream: _unused_first_argument,
                                            #second_field_ident,
                                        } => #second_field_ident,
                                    }
                                },
                                ErrorFieldName::InnerErrors => {
                                    quote::quote!{
                                        #ident_with_deserialize_token_stream::#variant_ident {
                                            #error_field_name_token_stream: _unused_first_argument,
                                            #second_field_ident,
                                        } => #second_field_ident,
                                    }
                                },
                            }
                        });
                        quote::quote! {
                            #(#generated_variants_logic),*
                        }
                    };
                    (
                        config_name_for_source_to_string_with_config,
                        logic_for_source_to_string_with_config,
                        logic_for_source_to_string_without_config,
                        logic_for_get_code_occurence,
                        logic_for_enum_with_deserialize,
                        logic_for_source_to_string_without_config_with_deserialize,
                        logic_for_get_code_occurence_with_deserialize
                    )
                },
            };
            //
            // let config_name_for_source_to_string_with_config = match origin_or_wrapper {
            //     OriginOrWrapper::Origin => {
            //         let underscore_config_stringified = "_config";
            //         underscore_config_stringified.parse::<proc_macro2::TokenStream>()
            //         .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {underscore_config_stringified} {parse_proc_macro2_token_stream_failed_message}"))
            //     },
            //     OriginOrWrapper::Wrapper => {
            //         let config_stringified = "config";
            //         config_stringified.parse::<proc_macro2::TokenStream>()
            //         .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {config_stringified} {parse_proc_macro2_token_stream_failed_message}"))
            //     },
            // };
            // let logic_for_source_to_string_with_config = match &origin_or_wrapper {
            //     OriginOrWrapper::Origin => quote::quote! {
            //         use #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream;
            //         self.#source_to_string_without_config_token_stream()
            //     },
            //     OriginOrWrapper::Wrapper => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
            //                 ErrorFieldName::InnerError => {
            //                     quote::quote! {
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream,
            //                             #second_field_ident: _unused_second_argument,
            //                         } => {
            //                             use #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream;
            //                             #error_field_name_token_stream.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
            //                         },
            //                     }
            //                 },
            //                 ErrorFieldName::InnerErrors => {
            //                     quote::quote! {
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream,
            //                             #second_field_ident: _unused_second_argument,
            //                         } => {
            //                             use #crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_token_stream;
            //                             #error_field_name_token_stream.#few_to_string_with_config_token_stream(config)
            //                         },
            //                     }
            //                 },
            //             }
            //         });
            //         quote::quote! {
            //             match self {
            //                 #(#generated_variants_logic),*
            //             }
            //         }
            //     },
            // };
            // let logic_for_source_to_string_without_config = match &origin_or_wrapper {
            //     OriginOrWrapper::Origin => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => {
            //                     quote::quote! {
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream,
            //                             #second_field_ident: _unused_second_argument,
            //                         } => #error_field_name_token_stream.to_string(),
            //                     }
            //                 },
            //                 ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
            //                 ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            //     OriginOrWrapper::Wrapper => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
            //                 ErrorFieldName::InnerError => {
            //                     quote::quote! {
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream,
            //                             #second_field_ident: _unused_second_argument,
            //                         } => {
            //                             use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
            //                             #error_field_name_token_stream.#to_string_without_config_token_stream()
            //                         },
            //                     }
            //                 },
            //                 ErrorFieldName::InnerErrors => {
            //                     quote::quote! {
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream,
            //                             #second_field_ident: _unused_second_argument,
            //                         } => {
            //                             use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream;
            //                             #error_field_name_token_stream.#few_to_string_without_config_token_stream()
            //                         },
            //                     }
            //                 },
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            // };
            // let logic_for_get_code_occurence = match &origin_or_wrapper {
            //     OriginOrWrapper::Origin => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => {
            //                     quote::quote!{
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream: _unused_first_argument,
            //                             #second_field_ident,
            //                         } => #second_field_ident,
            //                     }
            //                 },
            //                 ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
            //                 ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            //     OriginOrWrapper::Wrapper => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
            //                 ErrorFieldName::InnerError => {
            //                     quote::quote!{
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream: _unused_first_argument,
            //                             #second_field_ident,
            //                         } => #second_field_ident,
            //                     }
            //                 },
            //                 ErrorFieldName::InnerErrors => {
            //                     quote::quote!{
            //                         #ident::#variant_ident {
            //                             #error_field_name_token_stream: _unused_first_argument,
            //                             #second_field_ident,
            //                         } => #second_field_ident,
            //                     }
            //                 },
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            // };
            // let logic_for_enum_with_deserialize = match &origin_or_wrapper {
            //     OriginOrWrapper::Origin => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             first_field_type,
            //             second_field_ident, 
            //             second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => {
            //                     let second_field_ident_token_stream = form_code_occurence_deserialize(
            //                         second_field_type, 
            //                         proc_macro_name, 
            //                         &ident_stringified, 
            //                         with_deserialize_camel_case, 
            //                         parse_proc_macro2_token_stream_failed_message,
            //                         code_occurence_camel_case,
            //                         lifetime_stringified
            //                     );
            //                     quote::quote!{
            //                         #variant_ident {
            //                             #error_field_name_token_stream: #first_field_type,
            //                             #[serde(borrow)]
            //                             #second_field_ident: #second_field_ident_token_stream
            //                         },
            //                     }
            //                 },
            //                 ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
            //                 ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            //     OriginOrWrapper::Wrapper => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             first_field_type,
            //             second_field_ident, 
            //             second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
            //                 ErrorFieldName::InnerError => {
            //                     let first_field_type_stringified = match first_field_type {
            //                         syn::Type::Path(type_path_handle) => {
            //                             let last_arg_option_lifetime = form_last_arg_lifetime(
            //                                 type_path_handle, 
            //                                 proc_macro_name, 
            //                                 &ident_stringified,
            //                                 lifetime_stringified,
            //                             );
            //                             let mut segments_stringified = type_path_handle.path.segments.iter()
            //                             .fold(String::from(""), |mut acc, elem| {
            //                                 acc.push_str(&format!("{}::", elem.ident));
            //                                 acc
            //                             });
            //                             segments_stringified.pop();
            //                             segments_stringified.pop();
            //                             format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}")
            //                         },
            //                         _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path"),
            //                     };
            //                     let first_field_type_token_stream = first_field_type_stringified
            //                     .parse::<proc_macro2::TokenStream>()
            //                     .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            //                     let second_field_ident_token_stream = form_code_occurence_deserialize(
            //                         second_field_type, 
            //                         proc_macro_name, 
            //                         &ident_stringified, 
            //                         with_deserialize_camel_case, 
            //                         parse_proc_macro2_token_stream_failed_message,
            //                         code_occurence_camel_case,
            //                         lifetime_stringified
            //                     );
            //                     quote::quote!{
            //                         #variant_ident {
            //                             #[serde(borrow)]
            //                             #error_field_name_token_stream: #first_field_type_token_stream,
            //                             #[serde(borrow)]
            //                             #second_field_ident: #second_field_ident_token_stream
            //                         },
            //                     }
            //                 },
            //                 ErrorFieldName::InnerErrors => {
            //                     let first_field_type_stringified = match first_field_type {
            //                         syn::Type::Path(type_path) => {
            //                             let supported_inner_errors_container =  match type_path.path.segments.last() {
            //                                 Some(path_segment) => {
            //                                     if path_segment.ident == "Vec" {
            //                                         SupportedInnerErrorsContainers::Vec
            //                                     }
            //                                     else if path_segment.ident == "HashMap" {
            //                                         SupportedInnerErrorsContainers::HashMap
            //                                     }
            //                                     else {
            //                                         SupportedInnerErrorsContainers::Other
            //                                     }
            //                                 },
            //                                 None => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() is None"),
            //                             };
            //                             let first_field_type_prep = match supported_inner_errors_container {
            //                                 SupportedInnerErrorsContainers::Vec => {
            //                                     let mut vec_checker: Option<()> = None;
            //                                     let type_path_path_segments_stringified = type_path.path.segments.iter()
            //                                     .fold(String::from(""), |mut acc, elem| {
            //                                         let elem_ident = &elem.ident;
            //                                         if *elem_ident == "Vec" {
            //                                             if vec_checker.is_some() {
            //                                                 panic!("{proc_macro_name} {ident_stringified} first_field_type detected more than one Vec inside type path");
            //                                             }
            //                                             match &elem.arguments {
            //                                                 syn::PathArguments::None => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::None for Vec"),
            //                                                 syn::PathArguments::AngleBracketed(angle_bracketed) => {
            //                                                     match angle_bracketed.args.len() == 1 {
            //                                                         true => {
            //                                                             match &angle_bracketed.args[0] {
            //                                                                 syn::GenericArgument::Type(type_handle) => {
            //                                                                     match type_handle {
            //                                                                         syn::Type::Path(type_path_handle) => {
            //                                                                             let mut segments_stringified = type_path_handle.path.segments.iter()
            //                                                                             .fold(String::from(""), |mut acc, elem| {
            //                                                                                 acc.push_str(&format!("{}::", elem.ident));
            //                                                                                 acc
            //                                                                             });
            //                                                                             segments_stringified.pop();
            //                                                                             segments_stringified.pop();
            //                                                                             acc.push_str(&format!("Vec<{segments_stringified}{with_deserialize_camel_case}<{lifetime_stringified}>>"))
            //                                                                         },
            //                                                                         _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for Vec"),
            //                                                                     }
            //                                                                 },
            //                                                                 _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for Vec"),
            //                                                             }
            //                                                         },
            //                                                         false => panic!("{proc_macro_name} {ident_stringified} works only with angle_bracketed.args.len() == 1 for Vec"),
            //                                                     }
            //                                                 },
            //                                                 syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::Parenthesized for Vec"),
            //                                             }
            //                                             vec_checker = Some(());
            //                                         }
            //                                         else {
            //                                             acc.push_str(&format!("{elem_ident}::"));
            //                                         }
            //                                         acc
            //                                     });
            //                                     type_path_path_segments_stringified
            //                                 },
            //                                 SupportedInnerErrorsContainers::HashMap => {
            //                                     let mut hashmap_checker: Option<()> = None;
            //                                     let type_path_path_segments_stringified = type_path.path.segments.iter()
            //                                     .fold(String::from(""), |mut acc, elem| {
            //                                         let elem_ident = &elem.ident;
            //                                         if *elem_ident == "HashMap" {
            //                                             if hashmap_checker.is_some() {
            //                                                 panic!("{proc_macro_name} {ident_stringified} first_field_type detected more than one HashMap inside type path");
            //                                             }
            //                                             match &elem.arguments {
            //                                                 syn::PathArguments::None => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::None for HashMap"),
            //                                                 syn::PathArguments::AngleBracketed(angle_bracketed_generic_arguments) => {
            //                                                     match angle_bracketed_generic_arguments.args.len() == 2 {
            //                                                         true => {
            //                                                             let hashmap_key = match &angle_bracketed_generic_arguments.args[0] {
            //                                                                 syn::GenericArgument::Lifetime(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
            //                                                                 syn::GenericArgument::Type(type_handle) => {
            //                                                                     match type_handle {
            //                                                                         syn::Type::Path(type_path_handle_two) => {
            //                                                                             let mut segments_stringified = type_path_handle_two.path.segments.iter()
            //                                                                             .fold(String::from(""), |mut acc, elem| {
            //                                                                                 acc.push_str(&format!("{}::", elem.ident));
            //                                                                                 acc
            //                                                                             });
            //                                                                             segments_stringified.pop();
            //                                                                             segments_stringified.pop();
            //                                                                             segments_stringified
            //                                                                         },
            //                                                                         _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for HashMap"),
            //                                                                     }
            //                                                                 },
            //                                                                 syn::GenericArgument::Const(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
            //                                                                 syn::GenericArgument::Binding(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
            //                                                                 syn::GenericArgument::Constraint(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap key"),
            //                                                             };
            //                                                             let hashmap_value = match &angle_bracketed_generic_arguments.args[1] {
            //                                                                 syn::GenericArgument::Lifetime(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
            //                                                                 syn::GenericArgument::Type(type_handle) => {
            //                                                                     match type_handle {
            //                                                                         syn::Type::Path(type_path_handle) => {
            //                                                                             let last_arg_option_lifetime = form_last_arg_lifetime(
            //                                                                                 type_path_handle, 
            //                                                                                 proc_macro_name, 
            //                                                                                 &ident_stringified,
            //                                                                                 lifetime_stringified,
            //                                                                             );
            //                                                                             let mut segments_stringified = type_path_handle.path.segments.iter()
            //                                                                             .fold(String::from(""), |mut acc, elem| {
            //                                                                                 acc.push_str(&format!("{}::", elem.ident));
            //                                                                                 acc
            //                                                                             });
            //                                                                             segments_stringified.pop();
            //                                                                             segments_stringified.pop();
            //                                                                             format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}")
            //                                                                         },
            //                                                                         _ => panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for HashMap"),
            //                                                                     }
            //                                                                 },
            //                                                                 syn::GenericArgument::Const(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
            //                                                                 syn::GenericArgument::Binding(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
            //                                                                 syn::GenericArgument::Constraint(_) => panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for HashMap value"),
            //                                                             };
            //                                                             acc.push_str(&format!("{elem_ident}<{hashmap_key},{hashmap_value}>"));
            //                                                         },
            //                                                         false => panic!("{proc_macro_name} {ident_stringified} works only with angle_bracketed_generic_arguments.args.len() == 2 for HashMap"),
            //                                                     }
            //                                                 },
            //                                                 syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} first_segment.arguments syn::PathArguments::Parenthesized for HashMap"),
            //                                             }
            //                                             hashmap_checker = Some(());
            //                                         }
            //                                         else {
            //                                             acc.push_str(&format!("{elem_ident}::"));
            //                                         }
            //                                         acc
            //                                     });
            //                                     type_path_path_segments_stringified
            //                                 },
            //                                 SupportedInnerErrorsContainers::Other => {
            //                                     let mut type_path_path_segments_stringified = type_path.path.segments.iter()
            //                                     .fold(String::from(""), |mut acc, elem| {
            //                                         let elem_ident = &elem.ident;
            //                                         acc.push_str(&format!("{elem_ident}::"));
            //                                         acc
            //                                     });
            //                                     type_path_path_segments_stringified.pop();
            //                                     type_path_path_segments_stringified.pop();
            //                                     type_path_path_segments_stringified
            //                                 },
            //                             };
            //                             first_field_type_prep
            //                         },
            //                         _ => panic!("{proc_macro_name} {ident_stringified} first_field_type supports only syn::Type::Path"),
            //                     };
            //                     let first_field_type_with_deserialize_token_stream = first_field_type_stringified
            //                     .parse::<proc_macro2::TokenStream>()
            //                     .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            //                     let second_field_ident_token_stream = form_code_occurence_deserialize(
            //                         second_field_type, 
            //                         proc_macro_name, 
            //                         &ident_stringified, 
            //                         with_deserialize_camel_case, 
            //                         parse_proc_macro2_token_stream_failed_message,
            //                         code_occurence_camel_case,
            //                         lifetime_stringified
            //                     );
            //                     quote::quote!{
            //                         #variant_ident {
            //                             #[serde(borrow)]
            //                             #error_field_name_token_stream: #first_field_type_with_deserialize_token_stream,
            //                             #[serde(borrow)]
            //                             #second_field_ident: #second_field_ident_token_stream
            //                         },
            //                     }
            //                 },
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            // };
            // let logic_for_source_to_string_without_config_with_deserialize = match &origin_or_wrapper {
            //     OriginOrWrapper::Origin => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => {
            //                     quote::quote! {
            //                         #ident_with_deserialize_token_stream::#variant_ident {
            //                             #error_field_name_token_stream,
            //                             #second_field_ident: _unused_second_argument,
            //                         } => #error_field_name_token_stream.to_string(),
            //                     }
            //                 },
            //                 ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
            //                 ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            //     OriginOrWrapper::Wrapper => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
            //                 ErrorFieldName::InnerError => quote::quote! {
            //                     #ident_with_deserialize_token_stream::#variant_ident {
            //                         #error_field_name_token_stream,
            //                         #second_field_ident: _unused_second_argument,
            //                     } => {
            //                         use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream;
            //                         #error_field_name_token_stream.#to_string_without_config_with_deserialize_token_stream()
            //                     }
            //                 },
            //                 ErrorFieldName::InnerErrors => quote::quote! {
            //                     #ident_with_deserialize_token_stream::#variant_ident {
            //                         #error_field_name_token_stream,
            //                         #second_field_ident: _unused_second_argument,
            //                     } => {
            //                         use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream;
            //                         #error_field_name_token_stream.#few_to_string_without_config_with_deserialize_token_stream()
            //                     }
            //                 },
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            // };
            // let logic_for_get_code_occurence_with_deserialize = match &origin_or_wrapper {
            //     OriginOrWrapper::Origin => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => {
            //                     quote::quote!{
            //                          #ident_with_deserialize_token_stream::#variant_ident {
            //                             #error_field_name_token_stream: _unused_first_argument,
            //                             #second_field_ident,
            //                         } => #second_field_ident,
            //                     }
            //                 },
            //                 ErrorFieldName::InnerError => panic!("{proc_macro_name} {ident_stringified} error field name is inner_error, but struct/enum field is Origin"),
            //                 ErrorFieldName::InnerErrors => panic!("{proc_macro_name} {ident_stringified} error field name is inner_errors, but struct/enum field is Origin"),
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            //     OriginOrWrapper::Wrapper => {
            //         let generated_variants_logic = vec_needed_info.iter().map(|(
            //             variant_ident, 
            //             error_field_name, 
            //             _first_field_type,
            //             second_field_ident, 
            //             _second_field_type,
            //             error_field_name_token_stream
            //         )|{
            //             match error_field_name {
            //                 ErrorFieldName::Error => panic!("{proc_macro_name} {ident_stringified} error field name is error, but struct/enum field is Wrapper"),
            //                 ErrorFieldName::InnerError => {
            //                     quote::quote!{
            //                         #ident_with_deserialize_token_stream::#variant_ident {
            //                             #error_field_name_token_stream: _unused_first_argument,
            //                             #second_field_ident,
            //                         } => #second_field_ident,
            //                     }
            //                 },
            //                 ErrorFieldName::InnerErrors => {
            //                     quote::quote!{
            //                         #ident_with_deserialize_token_stream::#variant_ident {
            //                             #error_field_name_token_stream: _unused_first_argument,
            //                             #second_field_ident,
            //                         } => #second_field_ident,
            //                     }
            //                 },
            //             }
            //         });
            //         quote::quote! {
            //             #(#generated_variants_logic),*
            //         }
            //     },
            // };
            quote::quote! {
                impl<#lifetime_token_stream, #config_generic_token_stream>
                    #crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream<
                        #lifetime_token_stream,
                        #config_generic_token_stream,
                    > for #ident<#lifetime_token_stream>
                    where #config_generic_token_stream: #crate_traits_fields_get_source_place_type_token_stream
                        + #crate_traits_fields_get_timezone_token_stream
                        + #crate_traits_get_server_address_get_server_address_token_stream,
                {
                    fn #source_to_string_with_config_token_stream(
                        &self,
                        #config_name_for_source_to_string_with_config: &#config_generic_token_stream
                    ) -> String {
                        #logic_for_source_to_string_with_config
                    }
                }
                impl<#lifetime_token_stream>
                    #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<
                        #lifetime_token_stream,
                    > for #ident<#lifetime_token_stream>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config
                        }
                    }
                }
                impl<#lifetime_token_stream> #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream<#lifetime_token_stream>
                    for #ident<#lifetime_token_stream>
                {
                    fn #get_code_occurence_token_stream(&self) -> &#crate_common_code_occurence_code_occurence_token_stream<#lifetime_token_stream> {
                        match self {
                            #logic_for_get_code_occurence
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
                pub enum #ident_with_deserialize_token_stream<#lifetime_token_stream> {
                    #logic_for_enum_with_deserialize
                }
                impl<#lifetime_token_stream> #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<#lifetime_token_stream> for #ident_with_deserialize_token_stream<#lifetime_token_stream>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config_with_deserialize
                        }
                    }
                }
                impl<#lifetime_token_stream> #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_token_stream<#lifetime_token_stream>
                    for #ident_with_deserialize_token_stream<#lifetime_token_stream>
                {
                    fn #get_code_occurence_with_deserialize_token_stream(
                        &self,
                    ) -> &#crate_common_code_occurence_code_occurence_with_deserialize_token_stream<#lifetime_token_stream> {
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
                    let variant_generated_logic = match first_field_type {
                        syn::Type::Path(type_path) => {
                            let last_segment_ident = type_path.path.segments.last()
                            .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} no last segment in type_path.path.segments"))
                            .ident.to_string();
                            match (last_segment_ident.contains(WRAPPER_NAME), last_segment_ident.contains(ORIGIN_NAME)) {
                                (true, true) => panic!("{proc_macro_name} {ident_stringified} last_segment_ident contains Wrapper and Origin"),
                                (true, false) => quote::quote! {
                                    i.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
                                },
                                (false, true) => quote::quote! {
                                    use #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_without_config_token_stream;
                                    i.#to_string_with_config_for_source_to_string_without_config_token_stream(config)
                                },
                                (false, false) => panic!("{proc_macro_name} {ident_stringified} last_segment_ident do not contain Wrapper or Origin"),
                            }
                        },
                        _ => panic!("{proc_macro_name} {ident_stringified} first_field_type supports only syn::Type::Path"),
                    };
                    quote::quote!{
                        #ident::#variant_ident(i) => {
                            #variant_generated_logic
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
                                let variant_type_with_deserialize_stringified = format!("{variant_type}{with_deserialize_camel_case}");
                                variant_type_with_deserialize_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {variant_type_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"))                                             
                            },
                            _ => panic!("{proc_macro_name} {ident_stringified} first_field_type supports only syn::Type::Path"),
                        };
                        quote::quote!{
                            #[serde(borrow)]
                            #variant_ident(#variant_type_with_deserialize_token_stream<#lifetime_token_stream>)
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
                impl<#lifetime_token_stream, #config_generic_token_stream>
                    #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream<
                    #lifetime_token_stream,
                    #config_generic_token_stream,
                    > for #ident<#lifetime_token_stream>
                where
                    #config_generic_token_stream: #crate_traits_fields_get_source_place_type_token_stream
                    + #crate_traits_fields_get_timezone_token_stream
                    + #crate_traits_get_server_address_get_server_address_token_stream,
                {
                    fn #to_string_with_config_for_source_to_string_with_config_token_stream(&self, config: &#config_generic_token_stream) -> String {
                        match self {
                            #logic_for_to_string_with_config_for_source_to_string_with_config
                        }
                    }
                }
                impl<#lifetime_token_stream> #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream<#lifetime_token_stream>
                    for #ident<#lifetime_token_stream>
                {
                    fn #to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_to_string_without_config
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)] 
                pub enum #ident_with_deserialize_token_stream<#lifetime_token_stream> {
                    #logic_for_enum_with_deserialize
                }
                impl<#lifetime_token_stream>
                    #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream<
                        #lifetime_token_stream,
                    > for #ident_with_deserialize_token_stream<#lifetime_token_stream>
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
        impl<#lifetime_token_stream> std::fmt::Display for #ident<#lifetime_token_stream> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                write!(f, "{}", self.#to_string_without_config_token_stream())
            }
        }
        #generated_impl_with_deserialize_alternatives
        impl<#lifetime_token_stream> std::fmt::Display for #ident_with_deserialize_token_stream<#lifetime_token_stream> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream;
                write!(f, "{}", self.#to_string_without_config_with_deserialize_token_stream())
            }
        }
    }.into()
}

fn form_last_arg_lifetime(
    type_path_handle: &syn::TypePath, 
    proc_macro_name: &str, 
    ident_stringified: &String,
    lifetime_stringified: &str,
) -> String {
    match type_path_handle.path.segments.last() {
        Some(path_segment) => {
            match &path_segment.arguments {
                syn::PathArguments::None => String::from(""),
                syn::PathArguments::AngleBracketed(angle_bracketed_generic_argument) => {
                    if let false = angle_bracketed_generic_argument.args.len() == 1 {
                        panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified angle_bracketed_generic_argument.args.len() != 1");
                    }
                    match &angle_bracketed_generic_argument.args[0] {
                        syn::GenericArgument::Lifetime(_) => format!("<{lifetime_stringified}>"),
                        syn::GenericArgument::Type(_) => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() angle_bracketed_generic_argument.args[0] unexpected syn::GenericArgument::Type"),
                        syn::GenericArgument::Const(_) => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() angle_bracketed_generic_argument.args[0] unexpected syn::GenericArgument::Const"),
                        syn::GenericArgument::Binding(_) => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() angle_bracketed_generic_argument.args[0] unexpected syn::GenericArgument::Binding"),
                        syn::GenericArgument::Constraint(_) => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() angle_bracketed_generic_argument.args[0] unexpected syn::GenericArgument::Constraint"),
                    }
                },
                syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
            }
        },
        None => panic!("{proc_macro_name} {ident_stringified} first_field_type_stringified type_path.path.segments.last() is None"),
    }
}

fn form_code_occurence_deserialize(
    second_field_type: &syn::Type, 
    proc_macro_name: &str, 
    ident_stringified: &String, 
    with_deserialize_camel_case: &str,
    parse_proc_macro2_token_stream_failed_message: &str,
    code_occurence_camel_case: &str,
    lifetime_stringified: &str
) -> proc_macro2::TokenStream {
    let second_field_ident_prep = match second_field_type {
        syn::Type::Path(type_path) => {
            match type_path.path.segments.last() {
                Some(path_segment) => {
                    if let false = path_segment.ident == code_occurence_camel_case {
                        panic!("{proc_macro_name} {ident_stringified} second_field_ident type_path.path.segments.last() != {code_occurence_camel_case}");
                    }
                },
                None => panic!("{proc_macro_name} {ident_stringified} second_field_ident type_path.path.segments.last() is None"),
            }
            let mut code_occurence_checker: Option<()> = None;
            let second_field_ident_segments_stringified = type_path.path.segments.iter()
            .fold(String::from(""), |mut acc, path_segment| {
                let path_segment_ident = &path_segment.ident;
                if *path_segment_ident == code_occurence_camel_case {
                    if code_occurence_checker.is_some() {
                        panic!("{proc_macro_name} {ident_stringified} second_field_ident detected more than one {code_occurence_camel_case} inside type path");
                    }
                    let last_arg_option_lifetime = form_last_arg_lifetime(
                        type_path, 
                        proc_macro_name, 
                        ident_stringified,
                        lifetime_stringified,
                    );
                    acc.push_str(&format!("{path_segment_ident}{with_deserialize_camel_case}{last_arg_option_lifetime}"));
                    code_occurence_checker = Some(());
                }
                else {
                    acc.push_str(&format!("{path_segment_ident}::"));
                }
                acc
            });
            if code_occurence_checker.is_none() {
                panic!("{proc_macro_name} {ident_stringified} no {code_occurence_camel_case} detected inside second_field_ident type path");
            }
            second_field_ident_segments_stringified
        },
        _ => panic!("{proc_macro_name} {ident_stringified} second_field_type supports only syn::Type::Path"),
    };
    second_field_ident_prep
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {second_field_ident_prep} {parse_proc_macro2_token_stream_failed_message}"))
}