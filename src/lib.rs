#![deny(
    // clippy::indexing_slicing,
    // clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]
use convert_case::Casing;

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

enum SupportedContainer {
    Vec{
        path: String,
        element_path: String,
        element_lifetime: Lifetime,
    },
    HashMap{
        path: String,
        key_segments_stringified: String, 
        key_lifetime_enum: Lifetime,
        value_segments_stringified: String, 
        value_lifetime_enum: Lifetime
    },
    Path{
        path: String, 
        should_add_serde_borrow: Lifetime,
    },
}

enum SupportedInnerErrorsContainers {
    Vec,
    HashMap,
}

enum Lifetime {
    Specified(String),
    NotSpecified,
}

impl std::fmt::Display for Lifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lifetime::Specified(l) => write!(f, "<'{l}>"),
            Lifetime::NotSpecified => write!(f, ""),
        }
    }
}

enum Attributes {
    ToString,
    DisplayForeignType,
    ErrorOccurence,
    VecToString,
    VecDisplayForeignType,
    VecErrorOccurence,
    HashMapKeyToStringValueToString,
    HashMapKeyToStringValueDisplayForeignType,
    HashMapKeyToStringValueErrorOccurence,
    HashMapKeyDisplayForeignTypeValueToString,
    HashMapKeyDisplayForeignTypeValueDisplayForeignType,
    HashMapKeyDisplayForeignTypeValueErrorOccurence,
}

#[proc_macro_derive(
    ImplErrorOccurence, 
    attributes(
        to_string, 
        display_foreign_type,
        error_occurence,
        vec_to_string,
        vec_display_foreign_type,
        vec_error_occurence,
        hashmap_key_to_string_value_to_string,
        hashmap_key_to_string_value_display_foreign_type,
        hashmap_key_to_string_value_error_occurence,
        hashmap_key_display_foreign_type_value_to_string,
        hashmap_key_display_foreign_type_value_display_foreign_type,
        hashmap_key_display_foreign_type_value_error_occurence,
    )
)]
pub fn derive_impl_error_occurence(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let proc_macro_name = "ImplErrorOccurence";
    let ast: syn::DeriveInput =
        syn::parse(input).unwrap_or_else(|_| panic!("{proc_macro_name} syn::parse(input) failed"));
    let ident = &ast.ident;
    let ident_stringified = ident.to_string();
    let parse_proc_macro2_token_stream_failed_message = ".parse::<proc_macro2::TokenStream>() failed";
    let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
    let lifetime_token_stream = lifetime_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {lifetime_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let vec_name = "Vec";
    let hashmap_name = "HashMap";
    let to_string_stringified = "to_string";
    let to_string_token_stream = 
    to_string_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let display_foreign_type_stringified = "display_foreign_type";
    let display_foreign_type_token_stream = display_foreign_type_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {display_foreign_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let error_occurence_stringified = "error_occurence";
    let vec_to_string_stringified = "vec_to_string";
    let vec_display_foreign_type_stringified = "vec_display_foreign_type";
    let vec_error_occurence_stringified = "vec_error_occurence";
    let hashmap_key_to_string_value_to_string_stringified = "hashmap_key_to_string_value_to_string";
    let hashmap_key_to_string_value_display_foreign_type_stringified = "hashmap_key_to_string_value_display_foreign_type";
    let hashmap_key_to_string_value_error_occurence_stringified = "hashmap_key_to_string_value_error_occurence";
    let hashmap_key_display_foreign_type_value_to_string_stringified = "hashmap_key_display_foreign_type_value_to_string";
    let hashmap_key_display_foreign_type_value_display_foreign_type_stringified = "hashmap_key_display_foreign_type_value_display_foreign_type";
    let hashmap_key_display_foreign_type_value_error_occurence_stringified = "hashmap_key_display_foreign_type_value_error_occurence";
    let with_deserialize_camel_case = "WithDeserialize";
    let with_deserialize_lower_case = with_deserialize_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let ident_with_deserialize_stringified = format!("{ident}{with_deserialize_camel_case}");
    let ident_with_deserialize_token_stream = ident_with_deserialize_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let config_generic_token_stream = "ConfigGeneric"
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_with_config_camel_case = "ToStringWithConfig";
    let to_string_with_config_lower_case = to_string_with_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_with_config_camel_case = format!("Source{to_string_with_config_camel_case}");
    let to_string_without_config_camel_case = "ToStringWithoutConfig";
    let to_string_without_config_lower_case = to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_without_config_camel_case = format!("Source{to_string_without_config_camel_case}");
    let source_to_string_without_config_lower_case = source_to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_without_config_token_stream = 
    source_to_string_without_config_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {source_to_string_without_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let code_occurence_camel_case = "CodeOccurence";
    let code_occurence_lower_case = code_occurence_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let get_code_occurence_lower_case = format!("get_{code_occurence_lower_case}");
    let crate_traits_stringified = "crate::traits::";
    let crate_traits_display_foreign_type_display_foreign_type_stringified = format!("{crate_traits_stringified}{display_foreign_type_stringified}::DisplayForeignType");
    let crate_traits_display_foreign_type_display_foreign_type_token_stream = crate_traits_display_foreign_type_display_foreign_type_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_display_foreign_type_display_foreign_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_fields_stringified = format!("{crate_traits_stringified}fields::");
    let crate_traits_error_logs_logic_stringified = format!("{crate_traits_stringified}error_logs_logic::");
    let first_field_type_name = "first_field_type";
    let first_field_type_stringified_name = "first_field_type_stringified";
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{to_string_without_config_lower_case}::{to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified}{with_deserialize_camel_case}");
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_fields_get_source_place_type_stringified = format!("{crate_traits_fields_stringified}GetSourcePlaceType");
    let crate_traits_fields_get_source_place_type_token_stream = 
    crate_traits_fields_get_source_place_type_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_fields_get_source_place_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_fields_get_timezone_stringified = format!("{crate_traits_fields_stringified}GetTimezone");
    let crate_traits_fields_get_timezone_token_stream = 
    crate_traits_fields_get_timezone_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_fields_get_timezone_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_get_server_address_get_server_address_stringified = format!("{crate_traits_stringified}get_server_address::GetServerAddress");
    let crate_traits_get_server_address_get_server_address_token_stream = 
    crate_traits_get_server_address_get_server_address_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_get_server_address_get_server_address_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{source_to_string_without_config_lower_case}::{source_to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream = 
    crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{to_string_with_config_lower_case}::{to_string_with_config_camel_case}For{source_to_string_with_config_camel_case}");
    let crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let few_to_string_without_config_stringified = format!("few_{to_string_without_config_lower_case}");
    let few_to_string_without_config_token_stream = 
    few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {few_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let few_to_string_without_config_with_deserialize_stringified = format!("{few_to_string_without_config_stringified}_{with_deserialize_lower_case}");
    let few_to_string_without_config_with_deserialize_token_stream = 
    few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {few_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{few_to_string_without_config_stringified}::Few{to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream = 
    crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified}{with_deserialize_camel_case}");
    let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream = 
    crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let source_to_string_with_config_stringified = format!("source_{to_string_with_config_lower_case}");
    let source_to_string_with_config_token_stream = 
    source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_with_config_for_source_to_string_with_config_stringified = format!("{to_string_with_config_lower_case}_for_{source_to_string_with_config_stringified}");
    let to_string_with_config_for_source_to_string_with_config_token_stream = 
    to_string_with_config_for_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {to_string_with_config_for_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_without_config_token_stream = 
    to_string_without_config_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {to_string_without_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_without_config_with_deserialize_stringified = format!("{to_string_without_config_lower_case}_{with_deserialize_lower_case}");
    let to_string_without_config_with_deserialize_token_stream = 
    to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified = format!("{crate_traits_error_logs_logic_stringified}{get_code_occurence_lower_case}::Get{code_occurence_camel_case}");
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified}{with_deserialize_camel_case}");
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_code_occurence_code_occurence_stringified = format!("crate::common::{code_occurence_lower_case}::{code_occurence_camel_case}");
    let crate_common_code_occurence_code_occurence_token_stream = 
    crate_common_code_occurence_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_common_code_occurence_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_code_occurence_code_occurence_with_deserialize_stringified = format!("{crate_common_code_occurence_code_occurence_stringified}{with_deserialize_camel_case}");
    let crate_common_code_occurence_code_occurence_with_deserialize_token_stream = 
    crate_common_code_occurence_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_common_code_occurence_code_occurence_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let get_code_occurence_token_stream = 
    get_code_occurence_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {get_code_occurence_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let get_code_occurence_with_deserialize_stringified = format!("{get_code_occurence_lower_case}_{with_deserialize_lower_case}");
    let get_code_occurence_with_deserialize_token_stream = 
    get_code_occurence_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {get_code_occurence_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let few_to_string_with_config_stringified = format!("few_{to_string_with_config_lower_case}");
    let few_to_string_with_config_token_stream = 
    few_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {few_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{few_to_string_with_config_stringified}::Few{to_string_with_config_camel_case}");
    let crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{source_to_string_with_config_stringified}::{source_to_string_with_config_camel_case}");
    let crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let into_serialize_deserialize_version_stringified = "into_serialize_deserialize_version";
    let into_serialize_deserialize_version_token_stream = into_serialize_deserialize_version_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {into_serialize_deserialize_version_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    }
    else {
        panic!("{proc_macro_name} {ident_stringified} only works with syn::Data::Enum");
    };
    let generics = {
        let mut lifetimes_stringified = String::from("");
        let lifetime_iterator_token_stream = ast.generics.params.iter().map(|gen_param|{
            if let syn::GenericParam::Lifetime(lifetime_deref) = gen_param {
                let lifetime_ident_stringified = format!("'{},", lifetime_deref.lifetime.ident);
                lifetime_ident_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {lifetime_ident_stringified} {parse_proc_macro2_token_stream_failed_message}"))
            }
            else {
                panic!("{proc_macro_name} {ident_stringified} only works with syn::GenericParam::Lifetime");
            }
        });
        quote::quote!{
            #(#lifetime_iterator_token_stream)*
        }
    };
    let mut all_equal: Option<SuportedEnumVariant> = None;
    let named_or_unnamed_error_name = "only works with enums where all variants are syn::Fields::Named or all variants are syn::Fields::Unnamed";
    if let true = &data_enum.variants.is_empty() {
        panic!("{proc_macro_name} {ident_stringified} enum variants are empty");
    }
    for variant in &data_enum.variants {
        match &variant.fields {
            syn::Fields::Named(_) => {
                match &all_equal {
                    Some(supported_variant) => {
                        if let SuportedEnumVariant::Unnamed = supported_variant {
                            panic!("{proc_macro_name} {ident_stringified} {named_or_unnamed_error_name}");
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
                        if let SuportedEnumVariant::Named = supported_variant {
                            panic!("{proc_macro_name} {ident_stringified} {named_or_unnamed_error_name}");
                        }
                    },
                    None => {
                        all_equal = Some(SuportedEnumVariant::Unnamed);
                    },
                }
            },
            syn::Fields::Unit => panic!("{proc_macro_name} {ident_stringified} {named_or_unnamed_error_name}"),
        }
    }
    let supported_enum_variant = if let Some(supported_enum_variant) = all_equal {
        supported_enum_variant
    }
    else {
        panic!("{proc_macro_name} {ident_stringified} only works with enums where variants named first field name is member of {:?}", ErrorFieldName::to_all_variants_lower_case_string_vec());
    };
    let generated_impl_with_deserialize_alternatives = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            let vec_needed_info = data_enum.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let needed_info = if let syn::Fields::Named(fields_named) = &variant.fields {
                    let suported_enum_variant_named_syn_fields_named = "SuportedEnumVariant::Named syn::Fields::Named";
                    let named = &fields_named.named;
                    if let false = named.len() == 2 {
                        panic!("{proc_macro_name} {ident_stringified} only works on named fields with length of 2");
                    }
                    let first_field = &named[0];
                    let first_field_ident =
                        first_field.ident.clone()
                        .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} {suported_enum_variant_named_syn_fields_named} first_field_ident is None"));
                    let (error_field_name, is_display_foreign_type_option) = if first_field_ident == *"error" {
                        //todo - this must be check on 0, 1, 2 attr elements
                        let is_display_foreign_type_option = if first_field.attrs.is_empty() {
                            Some(false)
                        }
                        else if first_field.attrs.len() == 1 {
                            let attribute = first_field.attrs.get(0).unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} {suported_enum_variant_named_syn_fields_named} cannot get error attributes"));
                            if let true = attribute.path.segments.len() != 1 {
                                panic!("{proc_macro_name} {ident_stringified} error attribute.path.segments.len() != 1");
                            }
                            if let true = attribute.path.segments[0].ident == display_foreign_type_stringified {
                                Some(true)
                            }
                            else {
                                panic!("{proc_macro_name} {ident_stringified} attribute.path.segments[0].ident != {display_foreign_type_stringified}");
                            }
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute for error field must be #[{display_foreign_type_stringified}] or nothing")
                        };
                        (ErrorFieldName::Error, is_display_foreign_type_option)
                    } else if first_field_ident == *"inner_error" {
                        (ErrorFieldName::InnerError, None)
                    } else if first_field_ident == *"inner_errors" {
                        (ErrorFieldName::InnerErrors, None)
                    } else {
                        panic!("{proc_macro_name} {ident_stringified} only works with enums where variants named first field name is member of {:?}", ErrorFieldName::to_all_variants_lower_case_string_vec());
                    };
                    let second_field = &named[1];
                    let second_field_ident =
                        second_field.ident.clone()
                        .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} {suported_enum_variant_named_syn_fields_named} second_field_ident is None"));
                    if second_field_ident != *code_occurence_lower_case {
                        panic!("{proc_macro_name} {ident_stringified} only works on enums where variants named second field name == {code_occurence_lower_case}");
                    }
                    let error_field_name_stringified = error_field_name.to_lower_snake_case();
                    let error_field_name_token_stream = error_field_name_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {error_field_name_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                    (error_field_name, &first_field.ty, second_field_ident, &second_field.ty, error_field_name_token_stream, is_display_foreign_type_option)
                }
                else {
                    panic!("{proc_macro_name} {ident_stringified} expected fields would be named");
                };
                (variant_ident, needed_info.0, needed_info.1, needed_info.2, needed_info.3, needed_info.4, needed_info.5)
            }).collect::<Vec<(&proc_macro2::Ident, ErrorFieldName, &syn::Type, proc_macro2::Ident, &syn::Type, proc_macro2::TokenStream, Option<bool>)>>();
            let mut logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
            let mut logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
            let mut logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
            let mut logic_for_enum_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
            let mut logic_for_source_to_string_without_config_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
            let mut logic_for_get_code_occurence_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_needed_info.len());
            vec_needed_info.iter().for_each(|(
                variant_ident, 
                error_field_name, 
                first_field_type,
                second_field_ident, 
                second_field_type,
                error_field_name_token_stream,
                is_display_foreign_type_option
            )|{
                let second_field_ident_token_stream = if let syn::Type::Path(type_path) = second_field_type {
                    if let Some(path_segment) = type_path.path.segments.last() {
                        if let false = path_segment.ident == code_occurence_camel_case {
                            panic!("{proc_macro_name} {ident_stringified} second_field_ident type_path.path.segments.last() != {code_occurence_camel_case}");
                        }
                    }
                    else {
                        panic!("{proc_macro_name} {ident_stringified} second_field_ident type_path.path.segments.last() is None");
                    };
                    let mut code_occurence_checker: Option<()> = None;
                    let second_field_ident_segments_stringified = type_path.path.segments.iter()
                    .fold(String::from(""), |mut acc, path_segment| {
                        let path_segment_ident = &path_segment.ident;
                        match *path_segment_ident == code_occurence_camel_case {
                            true => {
                                if code_occurence_checker.is_some() {
                                    panic!("{proc_macro_name} {ident_stringified} second_field_ident detected more than one {code_occurence_camel_case} inside type path");
                                }
                                let last_arg_option_lifetime = form_last_arg_lifetime(
                                type_path, 
                                    proc_macro_name, 
                                    &ident_stringified,
                                    first_field_type_stringified_name,
                                ).to_string();
                                acc.push_str(&format!("{path_segment_ident}{with_deserialize_camel_case}{last_arg_option_lifetime}"));
                                code_occurence_checker = Some(());
                                },
                            false => acc.push_str(&format!("{path_segment_ident}::")),
                        }
                        acc
                    });
                    if code_occurence_checker.is_none() {
                        panic!("{proc_macro_name} {ident_stringified} no {code_occurence_camel_case} detected inside second_field_ident type path");
                    }
                    second_field_ident_segments_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {second_field_ident_segments_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                }
                else {
                    panic!("{proc_macro_name} {ident_stringified} second_field_type supports only syn::Type::Path");
                };
                match error_field_name {
                    ErrorFieldName::Error => {
                        logic_for_source_to_string_with_config.push(quote::quote! {
                            #ident::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                use #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream;
                                self.#source_to_string_without_config_token_stream()
                            }
                        });
                        let is_display_foreign_type = is_display_foreign_type_option
                        .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} is_display_foreign_type_option unexpected logic"));
                        let to_string_or_display_foreign_type_method_token_stream = match is_display_foreign_type {
                            true => quote::quote!{
                                use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                #error_field_name_token_stream.#display_foreign_type_token_stream()
                            },
                            false => quote::quote!{
                                #error_field_name_token_stream.#to_string_token_stream()
                            },
                        };
                        logic_for_source_to_string_without_config.push(quote::quote! {
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                #to_string_or_display_foreign_type_method_token_stream
                            }
                        });
                        logic_for_get_code_occurence.push(quote::quote!{
                            #ident::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident
                        });
                        logic_for_enum_with_deserialize.push({
                            quote::quote!{
                                #variant_ident {
                                    #error_field_name_token_stream: String,//#first_field_type,
                                    #[serde(borrow)]
                                    #second_field_ident: #second_field_ident_token_stream
                                }
                            }
                        });
                        logic_for_source_to_string_without_config_with_deserialize.push(quote::quote! {
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => #error_field_name_token_stream.to_string()
                        });
                        logic_for_get_code_occurence_with_deserialize.push(quote::quote!{
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident
                        });
                        logic_for_into_serialize_deserialize_version.push(quote::quote!{
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident,
                            } => {
                                #ident_with_deserialize_token_stream::#variant_ident {
                                    #error_field_name_token_stream: {
                                        #to_string_or_display_foreign_type_method_token_stream
                                    },
                                    #second_field_ident: #second_field_ident.#into_serialize_deserialize_version_token_stream(),
                                }
                            }
                        });
                    },
                    ErrorFieldName::InnerError => {
                        logic_for_source_to_string_with_config.push(quote::quote! {
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                use #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream;
                                #error_field_name_token_stream.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
                            }
                        });
                        logic_for_source_to_string_without_config.push(quote::quote! {
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                                #error_field_name_token_stream.#to_string_without_config_token_stream()
                            }
                        });
                        logic_for_get_code_occurence.push(quote::quote!{
                            #ident::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident
                        });
                        logic_for_enum_with_deserialize.push({
                            let first_field_type_token_stream = if let syn::Type::Path(type_path_handle) = first_field_type {
                                let last_arg_option_lifetime = form_last_arg_lifetime(
                                    type_path_handle, 
                                    proc_macro_name, 
                                    &ident_stringified,
                                    first_field_type_stringified_name
                                ).to_string();
                                let mut segments_stringified = type_path_handle.path.segments.iter()
                                .fold(String::from(""), |mut acc, elem| {
                                    acc.push_str(&format!("{}::", elem.ident));
                                    acc
                                });
                                segments_stringified.pop();
                                segments_stringified.pop();
                                let first_field_type_stringified = format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}");
                                first_field_type_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                            }
                            else {
                                panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path");
                            };
                            quote::quote!{
                                #variant_ident {
                                    #[serde(borrow)]
                                    #error_field_name_token_stream: #first_field_type_token_stream,
                                    #[serde(borrow)]
                                    #second_field_ident: #second_field_ident_token_stream
                                }
                            }
                        });
                        logic_for_source_to_string_without_config_with_deserialize.push(quote::quote! {
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream;
                                #error_field_name_token_stream.#to_string_without_config_with_deserialize_token_stream()
                            }
                        });
                        logic_for_get_code_occurence_with_deserialize.push(quote::quote!{
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident
                        });
                        logic_for_into_serialize_deserialize_version.push(quote::quote!{
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident,
                            } => #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream: #error_field_name_token_stream.#into_serialize_deserialize_version_token_stream(),
                                #second_field_ident: #second_field_ident.#into_serialize_deserialize_version_token_stream(),
                            }
                        });
                    },
                    ErrorFieldName::InnerErrors => {
                        let (supported_inner_errors_container, type_path) = 
                        if let syn::Type::Path(type_path) = first_field_type {
                            let path_segment = type_path.path.segments.last()
                            .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() is None"));
                            if path_segment.ident == vec_name {
                                (SupportedInnerErrorsContainers::Vec, type_path)
                            }
                            else if path_segment.ident == hashmap_name {
                                (SupportedInnerErrorsContainers::HashMap, type_path)
                            }
                            else {
                                panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() is not {vec_name} or {hashmap_name}")
                            }
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} {first_field_type_name} supports only syn::Type::Path");
                        };
                        let (first_field_logic_into_serialize_deserialize_version_token_stream, first_field_type_with_deserialize_token_stream) =  match supported_inner_errors_container {
                            SupportedInnerErrorsContainers::Vec => {
                                let mut vec_checker: Option<()> = None;
                                let type_path_path_segments_stringified = type_path.path.segments.iter()
                                .fold(String::from(""), |mut acc, elem| {
                                    let elem_ident = &elem.ident;
                                    if *elem_ident == vec_name {
                                        if vec_checker.is_some() {
                                            panic!("{proc_macro_name} {ident_stringified} {first_field_type_name} detected more than one {vec_name} inside type path");
                                        }
                                        if let syn::PathArguments::AngleBracketed(angle_bracketed) = &elem.arguments {
                                            if let true = angle_bracketed.args.len() == 1 {
                                                if let syn::GenericArgument::Type(type_handle) = &angle_bracketed.args[0] {
                                                    if let syn::Type::Path(type_path_handle) = type_handle {
                                                        let mut segments_stringified = type_path_handle.path.segments.iter()
                                                        .fold(String::from(""), |mut acc, elem| {
                                                            acc.push_str(&format!("{}::", elem.ident));
                                                            acc
                                                        });
                                                        segments_stringified.pop();
                                                        segments_stringified.pop();
                                                        acc.push_str(&format!("{vec_name}<{segments_stringified}{with_deserialize_camel_case}<{lifetime_stringified}>>"))
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for {vec_name}");
                                                    }
                                                }
                                                else {
                                                    panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for {vec_name}");
                                                }
                                            }
                                            else {
                                                panic!("{proc_macro_name} {ident_stringified} works only with angle_bracketed.args.len() == 1 for {vec_name}");
                                            }
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} first_segment.arguments works only with syn::PathArguments::AngleBracketed for {vec_name}")
                                        }
                                        vec_checker = Some(());
                                    }
                                    else {
                                        acc.push_str(&format!("{elem_ident}::"));
                                    }
                                    acc
                                });
                                (
                                    quote::quote!{
                                        #error_field_name_token_stream
                                        .into_iter()
                                        .map(|e| e.#into_serialize_deserialize_version_token_stream())
                                        .collect()
                                    },
                                    type_path_path_segments_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_path_path_segments_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                )
                            },
                            SupportedInnerErrorsContainers::HashMap => {
                                let mut hashmap_checker: Option<()> = None;
                                let type_path_path_segments_stringified = type_path.path.segments.iter()
                                .fold(String::from(""), |mut acc, elem| {
                                    let elem_ident = &elem.ident;
                                    if *elem_ident == hashmap_name {
                                        if hashmap_checker.is_some() {
                                            panic!("{proc_macro_name} {ident_stringified} {first_field_type_name} detected more than one {hashmap_name} inside type path");
                                        }
                                        if let syn::PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = &elem.arguments {
                                            if let true = angle_bracketed_generic_arguments.args.len() == 2 {
                                                let hashmap_key = if let syn::GenericArgument::Type(type_handle) = &angle_bracketed_generic_arguments.args[0] {
                                                    if let syn::Type::Path(type_path_handle_two) = type_handle {
                                                        let mut segments_stringified = type_path_handle_two.path.segments.iter()
                                                        .fold(String::from(""), |mut acc, elem| {
                                                            acc.push_str(&format!("{}::", elem.ident));
                                                            acc
                                                        });
                                                        segments_stringified.pop();
                                                        segments_stringified.pop();
                                                        segments_stringified
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for {hashmap_name}");
                                                    }
                                                }
                                                else {
                                                    panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for {hashmap_name} key");
                                                };
                                                let hashmap_value = if let syn::GenericArgument::Type(type_handle) = &angle_bracketed_generic_arguments.args[1] {
                                                    if let syn::Type::Path(type_path_handle) = type_handle {
                                                        let last_arg_option_lifetime = form_last_arg_lifetime(
                                                            type_path_handle, 
                                                            proc_macro_name, 
                                                            &ident_stringified,
                                                            first_field_type_stringified_name
                                                        ).to_string();
                                                        let mut segments_stringified = type_path_handle.path.segments.iter()
                                                        .fold(String::from(""), |mut acc, elem| {
                                                            acc.push_str(&format!("{}::", elem.ident));
                                                            acc
                                                        });
                                                        segments_stringified.pop();
                                                        segments_stringified.pop();
                                                        format!("{segments_stringified}{with_deserialize_camel_case}{last_arg_option_lifetime}")
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name} {ident_stringified} works only with syn::Type::Path for {hashmap_name}");
                                                    }
                                                }
                                                else {
                                                    panic!("{proc_macro_name} {ident_stringified} works only with syn::GenericArgument::Type for {hashmap_name} value");
                                                };
                                                acc.push_str(&format!("{elem_ident}<{hashmap_key},{hashmap_value}>"));
                                            }
                                            else {
                                                panic!("{proc_macro_name} {ident_stringified} works only with angle_bracketed_generic_arguments.args.len() == 2 for {hashmap_name}");
                                            }
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} first_segment.arguments works only with syn::PathArguments::AngleBracketed for {hashmap_name}");
                                        }
                                        hashmap_checker = Some(());
                                    }
                                    else {
                                        acc.push_str(&format!("{elem_ident}::"));
                                    }
                                    acc
                                });
                                (
                                    quote::quote!{
                                        #error_field_name_token_stream
                                        .into_iter()
                                        .map(|(k, v)| (k, v.#into_serialize_deserialize_version_token_stream()))
                                        .collect()
                                    },
                                    type_path_path_segments_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_path_path_segments_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                )
                            },
                        };
                        logic_for_source_to_string_with_config.push(quote::quote! {
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                use #crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_token_stream;
                                #error_field_name_token_stream.#few_to_string_with_config_token_stream(config)
                            }
                        });
                        logic_for_source_to_string_without_config.push(quote::quote! {
                            #ident::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream;
                                #error_field_name_token_stream.#few_to_string_without_config_token_stream()
                            }
                        });
                        logic_for_get_code_occurence.push(quote::quote!{
                            #ident::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident
                        });
                        logic_for_enum_with_deserialize.push({
                            quote::quote!{
                                #variant_ident {
                                    #[serde(borrow)]
                                    #error_field_name_token_stream: #first_field_type_with_deserialize_token_stream,
                                    #[serde(borrow)]
                                    #second_field_ident: #second_field_ident_token_stream
                                }
                            }
                        });
                        logic_for_source_to_string_without_config_with_deserialize.push(quote::quote! {
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream,
                                #second_field_ident: _unused_second_argument,
                            } => {
                                use #crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream;
                                #error_field_name_token_stream.#few_to_string_without_config_with_deserialize_token_stream()
                            }
                        });
                        logic_for_get_code_occurence_with_deserialize.push(quote::quote!{
                            #ident_with_deserialize_token_stream::#variant_ident {
                                #error_field_name_token_stream: _unused_first_argument,
                                #second_field_ident,
                            } => #second_field_ident
                        });
                        logic_for_into_serialize_deserialize_version.push({
                            quote::quote!{
                                #ident::#variant_ident {
                                    #error_field_name_token_stream,
                                    #second_field_ident,
                                } => #ident_with_deserialize_token_stream::#variant_ident {
                                    #error_field_name_token_stream: #first_field_logic_into_serialize_deserialize_version_token_stream,
                                    #second_field_ident: #second_field_ident.#into_serialize_deserialize_version_token_stream(),
                                }
                            }
                        });
                    },
                }
            });
            let logic_for_source_to_string_with_config_iter = logic_for_source_to_string_with_config.iter();
            let logic_for_source_to_string_without_config_iter = logic_for_source_to_string_without_config.iter();
            let logic_for_get_code_occurence_iter = logic_for_get_code_occurence.iter();
            let logic_for_enum_with_deserialize_iter = logic_for_enum_with_deserialize.iter();
            let logic_for_source_to_string_without_config_with_deserialize_iter = logic_for_source_to_string_without_config_with_deserialize.iter();
            let logic_for_get_code_occurence_with_deserialize_iter = logic_for_get_code_occurence_with_deserialize.iter();
            let logic_for_into_serialize_deserialize_version_iter = logic_for_into_serialize_deserialize_version.iter();
            let logic_for_source_to_string_with_config = quote::quote! {
                #(#logic_for_source_to_string_with_config_iter),*
            };
            let logic_for_source_to_string_without_config = quote::quote! {
                #(#logic_for_source_to_string_without_config_iter),*
            };
            let logic_for_get_code_occurence = quote::quote! {
                #(#logic_for_get_code_occurence_iter),*
            };
            let logic_for_enum_with_deserialize = quote::quote! {
                #(#logic_for_enum_with_deserialize_iter),*
            };
            let logic_for_source_to_string_without_config_with_deserialize = quote::quote! {
                #(#logic_for_source_to_string_without_config_with_deserialize_iter),*
            };
            let logic_for_get_code_occurence_with_deserialize = quote::quote! {
                #(#logic_for_get_code_occurence_with_deserialize_iter),*
            };
            let logic_for_into_serialize_deserialize_version = quote::quote! {
                #(#logic_for_into_serialize_deserialize_version_iter),*
            };
            quote::quote! {
                impl<#generics #config_generic_token_stream>
                    #crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream<
                        #generics
                        #config_generic_token_stream,
                    > for #ident<#generics>
                    where #config_generic_token_stream: #crate_traits_fields_get_source_place_type_token_stream
                        + #crate_traits_fields_get_timezone_token_stream
                        + #crate_traits_get_server_address_get_server_address_token_stream,
                {
                    fn #source_to_string_with_config_token_stream(
                        &self,
                        config: &#config_generic_token_stream //unknown which arg there would be
                    ) -> String {
                        match self {
                            #logic_for_source_to_string_with_config
                        }
                    }
                }
                impl<#generics>
                    #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<
                        #generics
                    > for #ident<#generics>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config
                        }
                    }
                }
                impl<#generics> #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream<#generics>
                    for #ident<#generics>
                {
                    fn #get_code_occurence_token_stream(&self) -> &#crate_common_code_occurence_code_occurence_token_stream<#generics> {
                        match self {
                            #logic_for_get_code_occurence
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
                pub enum #ident_with_deserialize_token_stream<#generics> {
                    #logic_for_enum_with_deserialize
                }
                impl<#generics> #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<#generics> for #ident_with_deserialize_token_stream<#generics>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config_with_deserialize
                        }
                    }
                }
                impl<#generics> #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_deserialize_token_stream<#generics>
                    for #ident_with_deserialize_token_stream<#generics>
                {
                    fn #get_code_occurence_with_deserialize_token_stream(
                        &self,
                    ) -> &#crate_common_code_occurence_code_occurence_with_deserialize_token_stream<#generics> {
                        match self {
                            #logic_for_get_code_occurence_with_deserialize
                        }
                    }
                }
                impl<#generics> #ident<#generics> {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_deserialize_token_stream<#generics> {
                        match self {
                            #logic_for_into_serialize_deserialize_version
                        }
                    }
                }
            }
        },
        SuportedEnumVariant::Unnamed => {
            let vec_variants_and_variants_types = data_enum.variants.iter().map(|variant| {
                let attributes = match variant.attrs.len() {
                    1 => {
                        let first_attribute = variant.attrs.get(0).unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} cannot get first variant attribute"));
                        if let true = first_attribute.path.segments.len() != 1 {
                            panic!("{proc_macro_name} {ident_stringified} first attribute.path.segments.len() != 1");
                        }
                        if let true = first_attribute.path.segments[0].ident == to_string_stringified {
                            Attributes::ToString
                        }
                        else if let true = first_attribute.path.segments[0].ident == display_foreign_type_stringified {
                            Attributes::DisplayForeignType
                        }
                        else if let true = first_attribute.path.segments[0].ident == error_occurence_stringified {
                            Attributes::ErrorOccurence
                        }
                        else if let true = first_attribute.path.segments[0].ident == vec_to_string_stringified {
                            Attributes::VecToString
                        }
                        else if let true = first_attribute.path.segments[0].ident == vec_display_foreign_type_stringified {
                            Attributes::VecDisplayForeignType
                        }
                        else if let true = first_attribute.path.segments[0].ident == vec_error_occurence_stringified {
                            Attributes::VecErrorOccurence
                        }
                        else if let true = first_attribute.path.segments[0].ident == hashmap_key_to_string_value_to_string_stringified {
                            Attributes::HashMapKeyToStringValueToString
                        }
                        else if let true = first_attribute.path.segments[0].ident == hashmap_key_to_string_value_display_foreign_type_stringified {
                            Attributes::HashMapKeyToStringValueDisplayForeignType
                        }
                        else if let true = first_attribute.path.segments[0].ident == hashmap_key_to_string_value_error_occurence_stringified {
                            Attributes::HashMapKeyToStringValueErrorOccurence
                        }
                        else if let true = first_attribute.path.segments[0].ident == hashmap_key_display_foreign_type_value_to_string_stringified {
                            Attributes::HashMapKeyDisplayForeignTypeValueToString
                        }
                        else if let true = first_attribute.path.segments[0].ident == hashmap_key_display_foreign_type_value_display_foreign_type_stringified {
                            Attributes::HashMapKeyDisplayForeignTypeValueDisplayForeignType
                        }
                        else if let true = first_attribute.path.segments[0].ident == hashmap_key_display_foreign_type_value_error_occurence_stringified {
                            Attributes::HashMapKeyDisplayForeignTypeValueErrorOccurence
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} first_attribute.path.segments[0].ident must be equal one of the supported attributes");
                        }
                    }
                    _ => {
                        panic!("{proc_macro_name} {ident_stringified} must contain attribute");
                    } 
                };
                let type_handle = if let syn::Fields::Unnamed(fields_unnamed) = &variant.fields {
                    let unnamed = &fields_unnamed.unnamed;
                    if let false = unnamed.len() == 1 {
                        panic!("{proc_macro_name} {ident_stringified} SuportedEnumVariant::Unnamed variant fields unnamed len != 1");
                    }
                    &unnamed[0].ty
                }
                else {
                    panic!("{proc_macro_name} {ident_stringified} only works with named fields");
                };
                (&variant.ident, type_handle, attributes)
            }).collect::<Vec<(&proc_macro2::Ident, &syn::Type, Attributes)>>();
            let mut is_lifetime_need_for_serialize_deserialize = false;
            let mut logic_for_to_string_with_config_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_enum_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_to_string_without_config_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            vec_variants_and_variants_types.iter().for_each(|(
                variant_ident, 
                first_field_type, 
                attributes
            )|{
                let supported_container = if let syn::Type::Path(type_path) = first_field_type {
                    let path = &type_path.path;
                    let path_segment = type_path.path.segments.last()
                    .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} type_path.path.segments.last() is None"));
                    if path_segment.ident == vec_name {
                        let mut segments_stringified = type_path.path.segments.iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&format!("{}::", elem.ident));
                            acc
                        });
                        segments_stringified.pop();
                        segments_stringified.pop();
                        let (element_path_stringified, element_lifetime_enum) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
                            if let true = angle_brackets_generic_arguments.args.len() == 1 {
                                if let syn::GenericArgument::Type(type_handle) = &angle_brackets_generic_arguments.args[0] {
                                    if let syn::Type::Path(type_path) = type_handle {
                                        let element_last_arg_option_lifetime = form_last_arg_lifetime(
                                            type_path, 
                                            proc_macro_name, 
                                            &ident_stringified,
                                            first_field_type_stringified_name
                                        );
                                        let mut element_segments_stringified = type_path.path.segments.iter()
                                        .fold(String::from(""), |mut acc, elem| {
                                            acc.push_str(&format!("{}::", elem.ident));
                                            acc
                                        });
                                        element_segments_stringified.pop();
                                        element_segments_stringified.pop();
                                        (element_segments_stringified, element_last_arg_option_lifetime)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} type_handle supports only syn::Type::Path");
                                    }
                                }
                                else {
                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type");
                                }
                            }
                            else {
                                panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args.len() == 1");
                            }
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} path_segment.arguments supports only syn::PathArguments::AngleBracketed");
                        };
                        SupportedContainer::Vec{
                            path: segments_stringified,
                            element_path: element_path_stringified,
                            element_lifetime: element_lifetime_enum,
                        }
                    }
                    else if path_segment.ident == hashmap_name {
                        let mut segments_stringified = type_path.path.segments.iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&format!("{}::", elem.ident));
                            acc
                        });
                        segments_stringified.pop();
                        segments_stringified.pop();
                        let (key_segments_stringified, key_lifetime_enum, value_segments_stringified, value_lifetime_enum) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
                            if let true = angle_brackets_generic_arguments.args.len() == 2 {
                                let (key_segments_stringified, key_lifetime_enum) = if let syn::GenericArgument::Type(type_handle) = &angle_brackets_generic_arguments.args[0] {
                                    if let syn::Type::Path(type_path) = type_handle {
                                        let key_last_arg_option_lifetime = form_last_arg_lifetime(
                                            type_path, 
                                            proc_macro_name, 
                                            &ident_stringified,
                                            first_field_type_stringified_name
                                        );
                                        let mut key_segments_stringified = type_path.path.segments.iter()
                                        .fold(String::from(""), |mut acc, elem| {
                                            acc.push_str(&format!("{}::", elem.ident));
                                            acc
                                        });
                                        key_segments_stringified.pop();
                                        key_segments_stringified.pop();
                                        (key_segments_stringified, key_last_arg_option_lifetime)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} type_handle supports only syn::Type::Path");
                                    }
                                }
                                else {
                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type");
                                };
                                let (value_segments_stringified, value_lifetime_enum) = if let syn::GenericArgument::Type(type_handle) = &angle_brackets_generic_arguments.args[1] {
                                    if let syn::Type::Path(type_path) = type_handle {
                                        let value_last_arg_option_lifetime = form_last_arg_lifetime(
                                            type_path, 
                                            proc_macro_name, 
                                            &ident_stringified,
                                            first_field_type_stringified_name
                                        );
                                        let mut value_segments_stringified = type_path.path.segments.iter()
                                        .fold(String::from(""), |mut acc, elem| {
                                            acc.push_str(&format!("{}::", elem.ident));
                                            acc
                                        });
                                        value_segments_stringified.pop();
                                        value_segments_stringified.pop();
                                        (value_segments_stringified, value_last_arg_option_lifetime)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} type_handle supports only syn::Type::Path");
                                    }
                                }
                                else {
                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type");
                                };
                                (key_segments_stringified, key_lifetime_enum, value_segments_stringified, value_lifetime_enum)
                            }
                            else {
                                panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args.len() == 2");
                            }
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} path_segment.arguments supports only syn::PathArguments::AngleBracketed");
                        };
                        // let hashmap_path_stringified = format!("{segments_stringified}<{key_value_stringified}>");
                        // let hashmap_path_token_stream = hashmap_path_stringified.parse::<proc_macro2::TokenStream>()
                        // .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_path_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        SupportedContainer::HashMap{
                            path: segments_stringified,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum
                        }
                    }
                    else {
                        let last_arg_option_lifetime = form_last_arg_lifetime(
                            type_path, 
                            proc_macro_name, 
                            &ident_stringified,
                            first_field_type_stringified_name
                        );
                        let mut segments_stringified = type_path.path.segments.iter()
                        .fold(String::from(""), |mut acc, elem| {
                            acc.push_str(&format!("{}::", elem.ident));
                            acc
                        });
                        segments_stringified.pop();
                        segments_stringified.pop();
                        SupportedContainer::Path{
                            path: segments_stringified, 
                            should_add_serde_borrow: last_arg_option_lifetime,
                        }
                    }
                }
                else {
                    panic!("{proc_macro_name} {ident_stringified} {first_field_type_name} supports only syn::Type::Path")
                };
                // logic_for_to_string_with_config_for_source_to_string_with_config
                // logic_for_to_string_without_config
                // logic_for_enum_with_deserialize
                // logic_for_to_string_without_config_with_deserialize
                // logic_for_into_serialize_deserialize_version
                let (
                    logic_for_to_string_with_config_for_source_to_string_with_config_inner,
                    logic_for_to_string_without_config_inner,
                    logic_for_enum_with_deserialize_inner,
                    logic_for_to_string_without_config_with_deserialize_inner,
                    logic_for_into_serialize_deserialize_version_inner,
                ) = match attributes {
                    Attributes::ToString => {
                        let (type_token_stringified, serde_borrow_option_token_stream) = if let SupportedContainer::Path { path, should_add_serde_borrow } = supported_container {
                            (
                                format!("{path}{should_add_serde_borrow}"),
                                match should_add_serde_borrow {
                                    Lifetime::Specified(_) => {
                                        is_lifetime_need_for_serialize_deserialize = true;
                                        quote::quote!{#[serde(borrow)]}
                                    },
                                    Lifetime::NotSpecified => quote::quote!{},
                                }
                            )
                        }
                        else {
                             panic!("{proc_macro_name} {ident_stringified} attribute #[{to_string_stringified}] supports only Path");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                i.#to_string_token_stream()
                            },
                            quote::quote!{
                                i.#to_string_token_stream()
                            },
                            quote::quote!{
                                #serde_borrow_option_token_stream
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.#to_string_token_stream()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident(i)
                            },
                        )
                    },
                    Attributes::DisplayForeignType => {
                        (
                            quote::quote!{
                                use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                i.#display_foreign_type_token_stream()
                            },
                            quote::quote!{
                                use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                i.#display_foreign_type_token_stream()
                            },
                            quote::quote!{
                                #variant_ident(String)
                            },
                            quote::quote!{
                                i.#to_string_token_stream()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                    i.#display_foreign_type_token_stream()
                                })
                            },
                        )
                    },
                    Attributes::ErrorOccurence => {
                        let (type_token_stringified, serde_borrow_option_token_stream) = if let SupportedContainer::Path { path, should_add_serde_borrow } = supported_container {
                            (
                                format!("{path}{with_deserialize_camel_case}{should_add_serde_borrow}"),
                                match should_add_serde_borrow {
                                    Lifetime::Specified(_) => {
                                        is_lifetime_need_for_serialize_deserialize = true;
                                        quote::quote!{#[serde(borrow)]}
                                    },
                                    Lifetime::NotSpecified => quote::quote!{},
                                }
                            )
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{error_occurence_stringified}] only support Path");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                i.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
                            },
                            quote::quote!{
                                i.#to_string_without_config_token_stream()
                            },
                            quote::quote!{
                                #serde_borrow_option_token_stream
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.#to_string_without_config_with_deserialize_token_stream()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident(i.#into_serialize_deserialize_version_token_stream())
                            },
                        )

                    },
                    Attributes::VecToString => {
                        let type_token_stringified = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                            if let Lifetime::Specified(_) = element_lifetime {
                                is_lifetime_need_for_serialize_deserialize = true;
                            }
                            format!("{path}<{element_path}{element_lifetime}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{vec_to_string_stringified}] only supports std::vec::Vec");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element = element.to_string().lines().fold(String::from(""), |mut acc, line| {
                                        acc.push_str(&format!(" {}\n", line));
                                        acc
                                    });
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element =
                                        element
                                        .to_string()
                                        .lines()
                                        .fold(String::from(""), |mut acc, line| {
                                            acc.push_str(&format!(" {}\n", line));
                                            acc
                                        });
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element =
                                        element
                                        .to_string()
                                        .lines()
                                        .fold(String::from(""), |mut acc, line| {
                                            acc.push_str(&format!(" {}\n", line));
                                            acc
                                        });
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident(i)
                            },
                        )
                    }
                    Attributes::VecDisplayForeignType => {
                        let type_token_stringified = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                            format!("{path}<String>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{vec_display_foreign_type_stringified}] only supports std::vec::Vec");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element = element.display_foreign_type().lines().fold(String::from(""), |mut acc, line| {
                                        acc.push_str(&format!(" {}\n", line));
                                        acc
                                    });
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element = element.display_foreign_type().lines().fold(
                                        String::from(""),
                                        |mut acc, line| {
                                            acc.push_str(&format!(" {}\n", line));
                                            acc
                                        },
                                    );
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element =
                                        element.lines().fold(String::from(""), |mut acc, line| {
                                            acc.push_str(&format!(" {}\n", line));
                                            acc
                                        });
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                 #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|e| {
                                        use crate::traits::display_foreign_type::DisplayForeignType;
                                        e.display_foreign_type()
                                    })
                                    .collect()
                                 })
                            },
                        )
                    }
                    Attributes::VecErrorOccurence => {
                        let type_token_stringified = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                            if let Lifetime::Specified(_) = element_lifetime {
                                is_lifetime_need_for_serialize_deserialize = true;
                            }
                            format!("{path}<{element_path}{with_deserialize_camel_case}{element_lifetime}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{vec_error_occurence_stringified}] only supports std::vec::Vec");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element = element.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String::from(""), |mut acc, line| {
                                        acc.push_str(&format!(" {}\n", line));
                                        acc
                                    });
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element = element.to_string_without_config().lines().fold(
                                        String::from(""),
                                        |mut acc, line| {
                                            acc.push_str(&format!(" {}\n", line));
                                            acc
                                        },
                                    );
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                let stringified_vec = i.iter().fold(String::from(""), |mut acc, element| {
                                    let stringified_element = element
                                        .to_string_without_config_with_deserialize()
                                        .lines()
                                        .fold(String::from(""), |mut acc, line| {
                                            acc.push_str(&format!(" {}\n", line));
                                            acc
                                        });
                                    acc.push_str(&stringified_element);
                                    acc
                                });
                                format!("[\n{}]", stringified_vec)
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|e| {
                                        use crate::traits::display_foreign_type::DisplayForeignType;
                                        e.into_serialize_deserialize_version()
                                    })
                                    .collect()
                                })
                            },
                        )
                    }
                    Attributes::HashMapKeyToStringValueToString => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            match (&key_lifetime_enum, &value_lifetime_enum) {
                                (Lifetime::Specified(_), Lifetime::Specified(_)) => {
                                    is_lifetime_need_for_serialize_deserialize = true;
                                },
                                (Lifetime::Specified(_), Lifetime::NotSpecified) => {
                                    is_lifetime_need_for_serialize_deserialize = true;
                                },
                                (Lifetime::NotSpecified, Lifetime::Specified(_)) => {
                                    is_lifetime_need_for_serialize_deserialize = true;
                                },
                                (Lifetime::NotSpecified, Lifetime::NotSpecified) => (),
                            }
                            format!("{path}<{key_segments_stringified}{key_lifetime_enum},{value_segments_stringified}{value_lifetime_enum}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_to_string_value_to_string_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.to_string().lines().fold(String::from(""), |mut accc, line| {
                                        accc.push_str(&format!(" {}\n", line));
                                        accc
                                    });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value =
                                        value
                                        .to_string()
                                        .lines()
                                        .fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value =
                                        value
                                        .to_string()
                                        .lines()
                                        .fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident(i)
                            },
                        )
                    },
                    Attributes::HashMapKeyToStringValueDisplayForeignType => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            if let Lifetime::Specified(_) = key_lifetime_enum {
                                is_lifetime_need_for_serialize_deserialize = true;
                            }
                            format!("{path}<{key_segments_stringified}{key_lifetime_enum},String>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_to_string_value_display_foreign_type_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.display_foreign_type().lines().fold(String::from(""), |mut accc, line| {
                                        accc.push_str(&format!(" {}\n", line));
                                        accc
                                    });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.display_foreign_type().lines().fold(
                                        String::from(""),
                                        |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        },
                                    );
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value =
                                        value.lines().fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|(k, v)| {
                                        use crate::traits::display_foreign_type::DisplayForeignType;
                                        (k, v.display_foreign_type())
                                    })
                                    .collect()
                                })
                            },
                        )
                    },
                    Attributes::HashMapKeyToStringValueErrorOccurence => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            match (&key_lifetime_enum, &value_lifetime_enum) {
                                (Lifetime::Specified(_), Lifetime::Specified(_)) => {
                                    is_lifetime_need_for_serialize_deserialize = true;
                                },
                                (Lifetime::Specified(_), Lifetime::NotSpecified) => {
                                    is_lifetime_need_for_serialize_deserialize = true;
                                },
                                (Lifetime::NotSpecified, Lifetime::Specified(_)) => {
                                    is_lifetime_need_for_serialize_deserialize = true;
                                },
                                (Lifetime::NotSpecified, Lifetime::NotSpecified) => (),
                            }
                            format!("{path}<{key_segments_stringified}{key_lifetime_enum},{value_segments_stringified}{with_deserialize_camel_case}{value_lifetime_enum}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_to_string_value_error_occurence_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String::from(""), |mut accc, line| {
                                        accc.push_str(&format!(" {}\n", line));
                                        accc
                                    });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.to_string_without_config().lines().fold(
                                        String::from(""),
                                        |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        },
                                    );
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value
                                        .to_string_without_config_with_deserialize()
                                        .lines()
                                        .fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|(k, v)| (k, v.into_serialize_deserialize_version()))
                                    .collect()
                                })
                            },
                        )
                    }
                    Attributes::HashMapKeyDisplayForeignTypeValueToString => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            if let Lifetime::Specified(_) = value_lifetime_enum {
                                is_lifetime_need_for_serialize_deserialize = true;
                            }
                            format!("{path}<String,{value_segments_stringified}{value_lifetime_enum}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_display_foreign_type_value_to_string_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.to_string().lines().fold(String::from(""), |mut accc, line| {
                                        accc.push_str(&format!(" {}\n", line));
                                        accc
                                    });
                                    acc.push_str(&format!("{} [\n{}]\n", key.display_foreign_type(), stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value =
                                        value
                                        .to_string()
                                        .lines()
                                        .fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!(
                                        "{} [\n{}]\n",
                                        key.display_foreign_type(),
                                        stringified_value
                                    ));
                                    acc
                                })
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value =
                                        value
                                        .to_string()
                                        .lines()
                                        .fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|(k, v)| {
                                        use crate::traits::display_foreign_type::DisplayForeignType;
                                        (k.display_foreign_type(), v)
                                    })
                                    .collect()
                                })
                            },
                        )
                    },
                    Attributes::HashMapKeyDisplayForeignTypeValueDisplayForeignType => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            format!("{path}<String,String>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_display_foreign_type_value_display_foreign_type_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.display_foreign_type().lines().fold(String::from(""), |mut accc, line| {
                                        accc.push_str(&format!(" {}\n", line));
                                        accc
                                    });
                                    acc.push_str(&format!("{} [\n{}]\n", key.display_foreign_type(), stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.display_foreign_type().lines().fold(
                                        String::from(""),
                                        |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        },
                                    );
                                    acc.push_str(&format!(
                                        "{} [\n{}]\n",
                                        key.display_foreign_type(),
                                        stringified_value
                                    ));
                                    acc
                                })
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value =
                                        value.lines().fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|(k, v)| {
                                        use crate::traits::display_foreign_type::DisplayForeignType;
                                        (k.display_foreign_type(), v.display_foreign_type())
                                    })
                                    .collect()
                                })
                            },
                        )
                    },
                    Attributes::HashMapKeyDisplayForeignTypeValueErrorOccurence => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            if let Lifetime::Specified(_) = value_lifetime_enum {
                                is_lifetime_need_for_serialize_deserialize = true;
                            }
                            format!("{path}<String,{value_segments_stringified}{with_deserialize_camel_case}{value_lifetime_enum}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_display_foreign_type_value_error_occurence_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.to_string_with_config_for_source_to_string_with_config(config).lines().fold(String::from(""), |mut accc, line| {
                                        accc.push_str(&format!(" {}\n", line));
                                        accc
                                    });
                                    acc.push_str(&format!("{} [\n{}]\n", key.display_foreign_type(), stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                use crate::traits::display_foreign_type::DisplayForeignType;
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value.to_string_without_config().lines().fold(
                                        String::from(""),
                                        |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        },
                                    );
                                    acc.push_str(&format!(
                                        "{} [\n{}]\n",
                                        key.display_foreign_type(),
                                        stringified_value
                                    ));
                                    acc
                                })
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                i.iter().fold(String::from(""), |mut acc, (key, value)| {
                                    let stringified_value = value
                                        .to_string_without_config_with_deserialize()
                                        .lines()
                                        .fold(String::from(""), |mut accc, line| {
                                            accc.push_str(&format!(" {}\n", line));
                                            accc
                                        });
                                    acc.push_str(&format!("{} [\n{}]\n", key, stringified_value));
                                    acc
                                })
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|(k, v)| {
                                        use crate::traits::display_foreign_type::DisplayForeignType;
                                        (
                                            k.display_foreign_type(),
                                            v.into_serialize_deserialize_version(),
                                        )
                                    })
                                    .collect()
                                })
                            },
                        )
                    },
                };
                logic_for_to_string_with_config_for_source_to_string_with_config.push({
                    quote::quote!{
                        #ident::#variant_ident(i) => {
                            #logic_for_to_string_with_config_for_source_to_string_with_config_inner
                        }
                    }
                });
                logic_for_to_string_without_config.push(quote::quote!{
                    #ident::#variant_ident(i) => {
                        #logic_for_to_string_without_config_inner
                    }
                });
                logic_for_enum_with_deserialize.push({
                    let variant_type_with_deserialize_token_stream = if let syn::Type::Path(type_path) = first_field_type {
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
                    }
                    else {
                        panic!("{proc_macro_name} {ident_stringified} {first_field_type_name} supports only syn::Type::Path")
                    };
                    quote::quote!{
                        #logic_for_enum_with_deserialize_inner
                    }
                });
                logic_for_to_string_without_config_with_deserialize.push(quote::quote!{
                    #ident_with_deserialize_token_stream::#variant_ident(i) => {
                        #logic_for_to_string_without_config_with_deserialize_inner
                    }
                });
                logic_for_into_serialize_deserialize_version.push(quote::quote!{
                     #ident::#variant_ident(i) => {
                        #logic_for_into_serialize_deserialize_version_inner
                     }
                });
            });
            let logic_for_to_string_with_config_for_source_to_string_with_config_generated = logic_for_to_string_with_config_for_source_to_string_with_config.iter();
            let logic_for_to_string_without_config_generated = logic_for_to_string_without_config.iter();
            let logic_for_enum_with_deserialize_generated = logic_for_enum_with_deserialize.iter();
            let logic_for_to_string_without_config_with_deserialize_generated = logic_for_to_string_without_config_with_deserialize.iter();
            let logic_for_into_serialize_deserialize_version_generated = logic_for_into_serialize_deserialize_version.iter();
            let logic_for_to_string_with_config_for_source_to_string_with_config = quote::quote! {
                #(#logic_for_to_string_with_config_for_source_to_string_with_config_generated),*
            };
            let logic_for_to_string_without_config = quote::quote! {
                #(#logic_for_to_string_without_config_generated),*
            };
            let logic_for_enum_with_deserialize = quote::quote! {
                #(#logic_for_enum_with_deserialize_generated),*
            };
            let logic_for_to_string_without_config_with_deserialize = quote::quote! {
                #(#logic_for_to_string_without_config_with_deserialize_generated),*
            };
            let logic_for_into_serialize_deserialize_version = quote::quote! {
                #(#logic_for_into_serialize_deserialize_version_generated),*
            };
            let lifetime_deserialize_token_stream = match is_lifetime_need_for_serialize_deserialize {
                true => quote::quote!{ #generics },//todo put all generic lifetimes here is wrong, must add one by one
                false => quote::quote!{},
            };
            quote::quote! {
//
// impl<'a, ConfigGeneric> 
//     crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig<
//         'a,
//         ConfigGeneric,
//     > 
//     for 
//     TestErrorEnum<> 
// where 
//     ConfigGeneric: 
//         crate::traits::fields::GetSourcePlaceType
//         + crate::traits::fields::GetTimezone
//         + crate::traits::get_server_address::GetServerAddress,
// {
//     fn to_string_with_config_for_source_to_string_with_config(&self, config: &ConfigGeneric) -> String
//     { 
//         match self { 
//             TestErrorEnum::ToString(i) => {
//                 i.to_string() 
//             } 
//         } 
//     }
// }
// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<'a>
//     for TestErrorEnum
// {
//     fn to_string_without_config(&self) -> String {
//         match self {
//             TestErrorEnum::ToString(i) => i.to_string(),
//         }
//     }
// }
// impl<'a> crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<'a>
//     for TestErrorEnumWithDeserialize
// {
//     fn to_string_without_config_with_deserialize(&self) -> String {
//         match self {
//             TestErrorEnumWithDeserialize::ToString(i) => i.to_string(),
//         }
//     }
// }
//
                impl<#generics #config_generic_token_stream>//todo - add let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
                    #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream<
                        #generics
                        #config_generic_token_stream,
                        //todo - add let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
                    > for #ident<#generics>
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
                impl<
                    #generics
                    //todo - add let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
                > #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream<
                    #generics
                    //todo - add let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
                >
                    for #ident<#generics>
                {
                    fn #to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_to_string_without_config
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)] 
                pub enum #ident_with_deserialize_token_stream<#lifetime_deserialize_token_stream> {
                    #logic_for_enum_with_deserialize
                }
                impl< 
                    #lifetime_deserialize_token_stream
                    //todo - change lifetime_deserialize_token_stream let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
                >
                    #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream<
                        #lifetime_deserialize_token_stream
                        //todo - change lifetime_deserialize_token_stream to let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
                    > 
                    for #ident_with_deserialize_token_stream<
                        #lifetime_deserialize_token_stream
                        //todo - add  AND CHECK IF ITS IN THE lifetime_deserialize_token_stream let lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
                    >
                {
                    fn #to_string_without_config_with_deserialize_token_stream(&self) -> String {
                        match self {
                            #logic_for_to_string_without_config_with_deserialize
                        }
                    }
                }
                impl<#generics> #ident<#generics> {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_deserialize_token_stream<#lifetime_deserialize_token_stream> {
                        match self {
                            #logic_for_into_serialize_deserialize_version
                        }
                    }
                }
            }
        },
    };
    let uuu = quote::quote! {
        impl<#generics> std::fmt::Display for #ident<#generics> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                write!(f, "{}", self.#to_string_without_config_token_stream())
            }
        }
        impl<#generics> std::fmt::Display for #ident_with_deserialize_token_stream<#generics> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream;
                write!(f, "{}", self.#to_string_without_config_with_deserialize_token_stream())
            }
        }
        #generated_impl_with_deserialize_alternatives
    };
    println!("{uuu}");
    uuu.into()
}

fn form_last_arg_lifetime(
    type_path_handle: &syn::TypePath, 
    proc_macro_name: &str, 
    ident_stringified: &String,
    first_field_type_stringified_name: &str,
) -> Lifetime {
    if let Some(path_segment) = type_path_handle.path.segments.last() {
        match &path_segment.arguments {
            syn::PathArguments::None => Lifetime::NotSpecified,
            syn::PathArguments::AngleBracketed(angle_bracketed_generic_argument) => {
                if let false = angle_bracketed_generic_argument.args.len() == 1 {
                    panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} angle_bracketed_generic_argument.args.len() != 1");
                }
                if let syn::GenericArgument::Lifetime(lfmt) = &angle_bracketed_generic_argument.args[0] {
                    Lifetime::Specified(lfmt.ident.to_string())
                }
                else {
                    panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] supports only syn::GenericArgument::Lifetime");
                }
            },
            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
        }
    }
    else {
        panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() is None");
    }
}