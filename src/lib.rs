#![deny(
    // clippy::indexing_slicing,
    // clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

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

#[derive(
    Clone
)]
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

enum ErrorOrCodeOccurence {
    Error {
        attribute: Attribute,
        // generic_types: GenericTypes,
        supported_container: SupportedContainer,
    },
    CodeOccurence {
        field_type: String,
        field_lifetime: Lifetime
    }
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
//todo - atrributes must be marked by proc_macro - if it would be 2 or more macro what uses same attribute name - incorrect generation logic

#[derive(
    Debug,
    strum_macros::EnumIter,
    strum_macros::Display,
    enum_extension::EnumExtension
)]
enum Attribute {
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

impl Attribute {
    fn from_ident(value: proc_macro2::Ident, proc_macro_name: String, ident_stringified: String) -> Self {
        let array = Attribute::into_array();
        let mut handle = None;
        for a in array {
            if let true = value == a.to_lower_snake_case() {
                handle = Some(a);
                break;
            }
        }
       match handle {
        Some(attr) => attr,
        None => panic!("{proc_macro_name} {ident_stringified} attribute must be equal one of the supported attributes"),
      }
    }
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
    //todo in WithDeserialize in case of Display foreign type must be &str intead of String
    //todo add to panic message file line column or occurence
    let proc_macro_name = "ImplErrorOccurence";
    let ast: syn::DeriveInput =
        syn::parse(input).unwrap_or_else(|_| panic!("{proc_macro_name} syn::parse(input) failed"));
    let ident = &ast.ident;
    let ident_stringified = ident.to_string();
    let parse_proc_macro2_token_stream_failed_message = ".parse::<proc_macro2::TokenStream>() failed";
    let trait_lifetime_stringified = "'error_occurence_proc_macro_reserved_lifetime_name";
    let trait_lifetime_token_stream = trait_lifetime_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {trait_lifetime_stringified} {parse_proc_macro2_token_stream_failed_message}"));
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
    use convert_case::Casing;
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
    let code_occurence_lower_case_token_stream = code_occurence_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
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
    // todo few_to_string_without_config became VecToStringWithoutConfigToString and HashmapToStringWithoutConfigToString

    // let few_to_string_without_config_stringified = format!("few_{to_string_without_config_lower_case}");
    // let few_to_string_without_config_token_stream = 
    // few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
        // .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {few_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    // let few_to_string_without_config_with_deserialize_stringified = format!("{few_to_string_without_config_stringified}_{with_deserialize_lower_case}");
    // let few_to_string_without_config_with_deserialize_token_stream = 
    // few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        // .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {few_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    // let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{few_to_string_without_config_stringified}::Few{to_string_without_config_camel_case}");
    // let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_token_stream = 
    // crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
    //     .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    // let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified = format!("{crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_stringified}{with_deserialize_camel_case}");
    // let crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_token_stream = 
    // crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified.parse::<proc_macro2::TokenStream>()
    //     .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_without_config_few_to_string_without_config_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
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
    // todo few_to_string_with_config bacame VecToStringWithConfigToString and HashMapImplDisplayToStringWithConfigToString

    // let few_to_string_with_config_stringified = format!("few_{to_string_with_config_lower_case}");
    // let few_to_string_with_config_token_stream = 
    // few_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        // .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {few_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    // let crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{few_to_string_with_config_stringified}::Few{to_string_with_config_camel_case}");
    // let crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_token_stream = 
    // crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        // .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_few_to_string_with_config_few_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
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
        let lifetimes_stringified = ast.generics.params.iter()
        .fold(String::from(""), |mut acc, gen_param| {
            if let syn::GenericParam::Lifetime(lifetime_deref) = gen_param {
                acc.push_str(&format!("'{},", lifetime_deref.lifetime.ident));
                acc
            }
            else {
                panic!("{proc_macro_name} {ident_stringified} only works with syn::GenericParam::Lifetime");
            }
        });
        if let true = lifetimes_stringified.contains(trait_lifetime_stringified) {
            panic!("{proc_macro_name} {ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
        }
        lifetimes_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {lifetimes_stringified} {parse_proc_macro2_token_stream_failed_message}"))
    };
    let supported_enum_variant = {
        let mut all_equal: Option<SuportedEnumVariant> = None;
        let named_or_unnamed_error_name = "only works with enums where all variants are syn::Fields::Named or all variants are syn::Fields::Unnamed";
        if let true = &data_enum.variants.is_empty() {
            panic!("{proc_macro_name} {ident_stringified} enum variants are empty");
        }
        data_enum.variants.iter().for_each(|variant|{
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
        });
        if let Some(supported_enum_variant) = all_equal {
            supported_enum_variant
        }
        else {
            panic!("{proc_macro_name} {ident_stringified} only works with enums where variants named first field name is member of {:?}", ErrorFieldName::to_all_variants_lower_case_string_vec());
        }
    };
    //todo should implement named\unnamed variation or not?
    let generated_impl_with_deserialize_alternatives = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            let variants_vec = data_enum.variants.iter().map(|variant| {
                let variant_fields_vec = if let syn::Fields::Named(fields_named) = &variant.fields {
                    let suported_enum_variant_named_syn_fields_named = "SuportedEnumVariant::Named syn::Fields::Named";
                    fields_named.named.iter().map(|field|{
                        let field_ident = field.ident.clone().unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} field.ident is None"));
                        let error_or_code_occurence = match field_ident == *code_occurence_lower_case {
                            true => {
                                let (code_occurence_type_stringified, code_occurence_lifetime) = {
                                    let mut code_occurence_type_option = None;
                                    fields_named.named.iter().for_each(|named|{
                                        let named_field_ident = named.ident.clone()
                                        .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} {suported_enum_variant_named_syn_fields_named} named_field_ident is None"));
                                        if named_field_ident == *code_occurence_lower_case {
                                            match code_occurence_type_option {
                                                Some(_) => panic!("{proc_macro_name} {ident_stringified} field must contain only one {code_occurence_lower_case} field"),
                                                None => {
                                                    if let syn::Type::Path(type_path) = &named.ty {
                                                        let lifetime_handle =  form_last_arg_lifetime(
                                                            type_path, 
                                                            proc_macro_name, 
                                                            &ident_stringified,
                                                            first_field_type_stringified_name,
                                                        );
                                                        let mut code_occurence_type_repeat_checker: Option<()> = None;
                                                        let mut code_occurence_segments_stringified = type_path.path.segments.iter()
                                                        .fold(String::from(""), |mut acc, path_segment| {
                                                            let path_segment_ident = &path_segment.ident;
                                                            match *path_segment_ident == code_occurence_camel_case {
                                                                true => {
                                                                    if code_occurence_type_repeat_checker.is_some() {
                                                                        panic!("{proc_macro_name} {ident_stringified} code_occurence_ident detected more than one {code_occurence_camel_case} inside type path");
                                                                    }
                                                                    acc.push_str(&path_segment_ident.to_string());
                                                                    code_occurence_type_repeat_checker = Some(());
                                                                },
                                                                false => acc.push_str(&format!("{path_segment_ident}::")),
                                                            }
                                                            acc
                                                        });
                                                        if code_occurence_type_repeat_checker.is_none() {
                                                            panic!("{proc_macro_name} {ident_stringified} no {code_occurence_camel_case} named field");
                                                        }
                                                        code_occurence_type_option = Some(
                                                            (
                                                                code_occurence_segments_stringified,
                                                                lifetime_handle,
                                                            )
                                                        )
                                                      }
                                                    else {
                                                        panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} supports only syn::Type::Path");
                                                      }
                                                 },
                                            }
                                        }
                                    });
                                    if let Some(code_occurence_type_info) = code_occurence_type_option {
                                        code_occurence_type_info
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} code_occurence_type_option is None");
                                    }
                                };
                                ErrorOrCodeOccurence::CodeOccurence {
                                    field_type: code_occurence_type_stringified,
                                    field_lifetime: code_occurence_lifetime
                                }
                            },
                            false => {
                                let mut option_attribute = None;
                                field.attrs.iter().for_each(|attr|{
                                    if let true = attr.path.segments.len() == 1 {
                                        if let true = attr.path.segments[0].ident == to_string_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::ToString);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == display_foreign_type_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::DisplayForeignType);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == error_occurence_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::ErrorOccurence);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == vec_to_string_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::VecToString);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == vec_display_foreign_type_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::VecDisplayForeignType);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == vec_error_occurence_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::VecErrorOccurence);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == hashmap_key_to_string_value_to_string_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::HashMapKeyToStringValueToString);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == hashmap_key_to_string_value_display_foreign_type_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::HashMapKeyToStringValueDisplayForeignType);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == hashmap_key_to_string_value_error_occurence_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::HashMapKeyToStringValueErrorOccurence);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == hashmap_key_display_foreign_type_value_to_string_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::HashMapKeyDisplayForeignTypeValueToString);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == hashmap_key_display_foreign_type_value_display_foreign_type_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::HashMapKeyDisplayForeignTypeValueDisplayForeignType);
                                            }
                                        }
                                        else if let true = attr.path.segments[0].ident == hashmap_key_display_foreign_type_value_error_occurence_stringified {
                                            if let true = option_attribute.is_some() {
                                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                                            }
                                            else {
                                                option_attribute = Some(Attribute::HashMapKeyDisplayForeignTypeValueErrorOccurence);
                                            }
                                        }//other attributes are not for this proc_macro
                                    }//other attributes are not for this proc_macro
                                });
                                let supported_container = if let syn::Type::Path(type_path) = &field.ty {
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
                                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type1");
                                                }
                                            }
                                            else {
                                                panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args.len() == 1 ###");
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
                                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type2");
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
                                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type3");
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
                                    panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} supports only syn::Type::Path");
                                };
                                ErrorOrCodeOccurence::Error {
                                    attribute: option_attribute.unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} option attribute is none")),
                                    supported_container,
                                }
                            },
                        };
                        (
                            field_ident,
                            error_or_code_occurence,
                        )
                    })
                    .collect::<Vec<(
                        proc_macro2::Ident,
                        ErrorOrCodeOccurence
                    )>>()
                }
                else {
                    panic!("{proc_macro_name} {ident_stringified} expected fields would be named");
                };
                (
                    &variant.ident, 
                    variant_fields_vec,
                )
            })
            .collect::<Vec<(
                &proc_macro2::Ident, 
                 Vec<(
                    proc_macro2::Ident,
                    ErrorOrCodeOccurence
                )>
            )>>();
            let mut logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_enum_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_to_string_without_config_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            variants_vec.iter().for_each(|(
                variant_ident, 
                fields_vec
            )|{
                let mut enum_fields_logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_enum_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_source_to_string_without_config_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_get_code_occurence_with_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut format_logic_for_source_to_string_without_config: Vec<&str> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_without_config_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_without_config_with_deserialize_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_into_serialize_deserialize_version_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                fields_vec.iter().enumerate().for_each(|(index, (field_ident, error_or_code_occurence))|{
                    let unused_argument_handle_stringified = format!("_unused_argument_{index}");
                    let unused_argument_handle_token_stream = unused_argument_handle_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {unused_argument_handle_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                    match error_or_code_occurence {
                        ErrorOrCodeOccurence::Error { 
                            attribute, 
                            supported_container,
                        } => {
                            let ( 
                                logic_for_source_to_string_without_config_for_attribute,
                                logic_for_source_to_string_without_config_with_deserialize_for_attribute,
                                logic_for_into_serialize_deserialize_version_for_attribute,
                                field_type_with_deserialize_token_stream,
                                serde_borrow_attribute_token_stream
                            ) = match attribute {
                                Attribute::ToString => {
                                    let (serde_borrow_attribute_handle, path_token_stream) = if let SupportedContainer::Path { path, should_add_serde_borrow } = supported_container {
                                        let serde_borrow_attribute_handle = match should_add_serde_borrow {
                                            Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                            Lifetime::NotSpecified => quote::quote!{},
                                        };
                                        let path_stringified = format!("{path}{should_add_serde_borrow}");
                                        let path_token_stream = path_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        (serde_borrow_attribute_handle, path_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::ToString is not a SupportedContainer::Path");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                #field_ident.lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            { 
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                #field_ident.lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident
                                            }
                                        },
                                        path_token_stream,
                                        serde_borrow_attribute_handle
                                    )
                                },
                                Attribute::DisplayForeignType => {
                                    if let SupportedContainer::Path { path, should_add_serde_borrow } = supported_container {}
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::DisplayForeignType is not a SupportedContainer::Path");
                                    }
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::display_foreign_type::DisplayForeignType;
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                #field_ident.display_foreign_type().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            { 
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                #field_ident.lines_space_backslash() 
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::display_foreign_type::DisplayForeignType;
                                                #field_ident.display_foreign_type()
                                            }
                                        },
                                        quote::quote! {
                                            &'static str
                                        },
                                        quote::quote! {},
                                    )
                                },
                                Attribute::ErrorOccurence => {
                                    let (serde_borrow_attribute_handle, path_with_deserialize_token_stream) = if let SupportedContainer::Path { path, should_add_serde_borrow } = supported_container {
                                        let serde_borrow_attribute_handle = match should_add_serde_borrow {
                                            Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                            Lifetime::NotSpecified => quote::quote!{},
                                        };
                                        let path_with_deserialize_stringified = format!("{path}{with_deserialize_camel_case}{should_add_serde_borrow}");
                                        let path_with_deserialize_token_stream = path_with_deserialize_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        (serde_borrow_attribute_handle, path_with_deserialize_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::ErrorOccurence is not a SupportedContainer::Path");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                                                #field_ident.to_string_without_config().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                                                #field_ident.to_string_without_config_with_deserialize().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_serialize_deserialize_version()
                                            }
                                        },
                                        path_with_deserialize_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                                Attribute::VecToString => {
                                    let (serde_borrow_attribute_handle, path_token_stream) = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                                        let serde_borrow_attribute_handle = match element_lifetime {
                                            Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                            Lifetime::NotSpecified => quote::quote!{},
                                        };
                                        let path_stringified = format!("{path}<{element_path}{element_lifetime}>");
                                        let path_token_stream = path_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        (serde_borrow_attribute_handle, path_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::VecToString is not a SupportedContainer::Vec");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                 use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                                                #field_ident.vec_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                                                #field_ident.vec_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident
                                            }
                                        },
                                        path_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                                Attribute::VecDisplayForeignType => {
                                    if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {}
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::VecDisplayForeignType is not a SupportedContainer::Vec");
                                    }
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                                                #field_ident.vec_display_foreign_type_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                                                #field_ident.vec_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::vec_display_foreign_type_into_vec_static_str::VecDisplayForeignTypeIntoVecStaticStr;
                                                #field_ident.vec_display_foreign_type_into_vec_static_str()
                                            }
                                        },
                                        quote::quote! {
                                            std::vec::Vec<&'static str>
                                        },
                                        quote::quote! {},
                                    )
                                },
                                Attribute::VecErrorOccurence => {
                                    let (serde_borrow_attribute_handle, path_with_deserialize_token_stream) = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                                        let serde_borrow_attribute_handle = match element_lifetime {
                                            Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                            Lifetime::NotSpecified => quote::quote!{},
                                        };
                                        let path_with_deserialize_stringified = format!("{path}<{element_path}{with_deserialize_camel_case}{element_lifetime}>");
                                        let path_with_deserialize_token_stream = path_with_deserialize_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        (serde_borrow_attribute_handle, path_with_deserialize_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::VecErrorOccurence is not a SupportedContainer::Vec");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToString;
                                                #field_ident.vec_to_string_without_config_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToStringWithDeserialize;
                                                #field_ident.vec_to_string_without_config_to_string_with_deserialize().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|i| i.into_serialize_deserialize_version())
                                                .collect()
                                            }
                                        },
                                        path_with_deserialize_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                                Attribute::HashMapKeyToStringValueToString => {
                                    let (serde_borrow_attribute_handle, path_token_stream) = if let SupportedContainer::HashMap { path, key_segments_stringified, key_lifetime_enum, value_segments_stringified, value_lifetime_enum } = supported_container {
                                        let serde_borrow_attribute_handle = match (key_lifetime_enum, value_lifetime_enum) {
                                            (Lifetime::Specified(_), Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
                                            (Lifetime::Specified(_), Lifetime::NotSpecified) => quote::quote!{#[serde(borrow)]},
                                            (Lifetime::NotSpecified, Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
                                            (Lifetime::NotSpecified, Lifetime::NotSpecified) => quote::quote!{},
                                        };
                                        let path_stringified = format!("{path}<{key_segments_stringified}{key_lifetime_enum}, {value_segments_stringified}{value_lifetime_enum}>");
                                        let path_token_stream = path_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        (serde_borrow_attribute_handle, path_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::HashMapKeyToStringValueToString is not a SupportedContainer::Vec");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                                #field_ident.hashmap_display_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                                #field_ident.hashmap_display_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident
                                            }
                                        },
                                        path_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                                Attribute::HashMapKeyToStringValueDisplayForeignType => {
                                    let (serde_borrow_attribute_handle, path_token_stream) = if let SupportedContainer::HashMap { 
                                        path, 
                                        key_segments_stringified, 
                                        key_lifetime_enum, 
                                        value_segments_stringified, 
                                        value_lifetime_enum 
                                    } = supported_container {
                                        let path_stringified = format!("{path}<{key_segments_stringified}{key_lifetime_enum},&'static str>");
                                        let path_token_stream = path_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        let serde_borrow_attribute_handle = match key_lifetime_enum {
                                            Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                            Lifetime::NotSpecified => quote::quote!{},
                                        };
                                        (serde_borrow_attribute_handle, path_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::HashMapKeyToStringValueDisplayForeignType is not a SupportedContainer::HashMap");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_to_string::HashMapDisplayDisplayForeignTypeToString;
                                                #field_ident.hashmap_display_display_foreign_type_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                                #field_ident.hashmap_display_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_into_hashmap_display_static_str::HashmapDisplayDisplayForeignTypeIntoHashmapDisplayStaticStr;
                                                #field_ident.hashmap_display_display_foreign_type_into_hashmap_display_static_str()
                                            }
                                        },
                                        path_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                                Attribute::HashMapKeyToStringValueErrorOccurence => {
                                    let (serde_borrow_attribute_handle, path_with_deserialize_token_stream) = if let SupportedContainer::HashMap { path, key_segments_stringified, key_lifetime_enum, value_segments_stringified, value_lifetime_enum } = supported_container {
                                        let serde_borrow_attribute_handle = match (key_lifetime_enum, value_lifetime_enum) {
                                            (Lifetime::Specified(_), Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
                                            (Lifetime::Specified(_), Lifetime::NotSpecified) => quote::quote!{#[serde(borrow)]},
                                            (Lifetime::NotSpecified, Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
                                            (Lifetime::NotSpecified, Lifetime::NotSpecified) => quote::quote!{},
                                        };
                                        let path_with_deserialize_stringified = format!("{path}<{key_segments_stringified}{key_lifetime_enum}, {value_segments_stringified}{with_deserialize_camel_case}{value_lifetime_enum}>");
                                        let path_with_deserialize_token_stream = path_with_deserialize_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        (serde_borrow_attribute_handle, path_with_deserialize_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::HashMapKeyToStringValueErrorOccurence is not a SupportedContainer::HashMap");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToString;
                                                #field_ident.hashmap_display_to_string_without_config_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                                                #field_ident.hashmap_display_to_string_without_config_to_string_with_deserialize().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|(k, v)| (k, { v.into_serialize_deserialize_version() }))
                                                .collect()
                                            }
                                        },
                                        path_with_deserialize_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                                Attribute::HashMapKeyDisplayForeignTypeValueToString => {
                                    let (hashmap_token_stream, serde_borrow_attribute_handle) = if let SupportedContainer::HashMap { 
                                        path, 
                                        key_segments_stringified, 
                                        key_lifetime_enum, 
                                        value_segments_stringified, 
                                        value_lifetime_enum 
                                    } = supported_container {
                                        let hashmap_stringified = format!("{path}<&'static str,{value_segments_stringified}{value_lifetime_enum}>");
                                        let hashmap_token_stream = hashmap_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        let serde_borrow_attribute_handle = match value_lifetime_enum {
                                            Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                            Lifetime::NotSpecified => quote::quote!{},
                                        };
                                        (hashmap_token_stream, serde_borrow_attribute_handle)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::HashMapKeyDisplayForeignTypeValueToString is not a SupportedContainer::HashMap");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_to_string::HashMapDisplayForeignTypeDisplayToString;
                                                #field_ident.hashmap_display_foreign_type_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                                #field_ident.hashmap_display_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_into_hashmap_static_str_display::HashmapDisplayForeignTypeDisplayIntoHashMapStaticStrDisplay;
                                                #field_ident.hashmap_display_foreign_type_display_into_hashmap_static_str_display()
                                            }
                                        },
                                        hashmap_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                                Attribute::HashMapKeyDisplayForeignTypeValueDisplayForeignType => {
                                    let hashmap_token_stream = if let SupportedContainer::HashMap { 
                                        path, 
                                        key_segments_stringified, 
                                        key_lifetime_enum, 
                                        value_segments_stringified, 
                                        value_lifetime_enum 
                                    } = supported_container {
                                        let hashmap_stringified = format!("{path}<&'static str,&'static str>");
                                        hashmap_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::HashMapKeyDisplayForeignTypeValueDisplayForeignType is not a SupportedContainer::HashMap");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                                                #field_ident.hashmap_display_foreign_type_display_foreign_type_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                                #field_ident.hashmap_display_display_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_into_hashmap_static_str_static_str::HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStaticStrStaticStr;
                                                #field_ident.hashmap_display_foreign_type_display_foreign_type_into_hashmap_static_str_static_str()
                                            }
                                        },
                                        hashmap_token_stream,
                                        quote::quote! {},
                                    )
                                },
                                Attribute::HashMapKeyDisplayForeignTypeValueErrorOccurence => {
                                    let (serde_borrow_attribute_handle, path_with_deserialize_token_stream) = if let SupportedContainer::HashMap { 
                                        path, 
                                        key_segments_stringified, 
                                        key_lifetime_enum, 
                                        value_segments_stringified, 
                                        value_lifetime_enum 
                                    } = supported_container {
                                        let serde_borrow_attribute_handle = match value_lifetime_enum {
                                            Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                            Lifetime::NotSpecified => quote::quote!{},
                                        };
                                        let path_with_deserialize_stringified = format!("{path}<&'static str, {value_segments_stringified}{with_deserialize_camel_case}{value_lifetime_enum}>");
                                        let path_with_deserialize_token_stream = path_with_deserialize_stringified
                                        .parse::<proc_macro2::TokenStream>()
                                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {path_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                        (serde_borrow_attribute_handle, path_with_deserialize_token_stream)
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} Attribute::HashMapKeyDisplayForeignTypeValueErrorOccurence is not a SupportedContainer::HashMap");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_without_config_to_string::HashMapDisplayForeignTypeToStringWithoutConfigToString;
                                                #field_ident.hashmap_display_foreign_type_to_string_without_config_to_string().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                                                use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                                                #field_ident.hashmap_display_to_string_without_config_to_string_with_deserialize().lines_space_backslash()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|(k, v)| {
                                                    (
                                                        {
                                                            use crate::traits::display_foreign_type::DisplayForeignType;
                                                            k.display_foreign_type()
                                                        },
                                                        { v.into_serialize_deserialize_version() },
                                                    )
                                                })
                                                .collect()
                                            }
                                        },
                                        path_with_deserialize_token_stream,
                                        serde_borrow_attribute_handle,
                                    )
                                },
                            };
                            enum_fields_logic_for_source_to_string_with_config.push(quote::quote! {
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config.push(quote::quote! {
                                #field_ident
                            });
                            enum_fields_logic_for_get_code_occurence.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_enum_with_deserialize.push(quote::quote!{
                                #serde_borrow_attribute_token_stream
                                #field_ident: #field_type_with_deserialize_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config_with_deserialize.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_get_code_occurence_with_deserialize.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_into_serialize_deserialize_version.push(quote::quote!{
                                #field_ident
                            });
                            format_logic_for_source_to_string_without_config.push("{}");
                            fields_logic_for_source_to_string_without_config_for_attribute.push(logic_for_source_to_string_without_config_for_attribute);
                            fields_logic_for_source_to_string_without_config_with_deserialize_for_attribute.push(logic_for_source_to_string_without_config_with_deserialize_for_attribute);
                            fields_logic_for_into_serialize_deserialize_version_for_attribute.push(quote::quote!{
                                #field_ident: #logic_for_into_serialize_deserialize_version_for_attribute
                            });
                        },
                        ErrorOrCodeOccurence::CodeOccurence { 
                            field_type,
                            field_lifetime,
                         } => {
                            let serde_borrow_attribute_token_stream = match field_lifetime {
                                Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
                                Lifetime::NotSpecified => quote::quote!{},
                            };
                            let code_occurence_type_with_deserialize_stringified = format!("{field_type}{with_deserialize_camel_case}{field_lifetime}");
                            let code_occurence_type_with_deserialize_token_stream = code_occurence_type_with_deserialize_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {code_occurence_type_with_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            enum_fields_logic_for_source_to_string_with_config.push(quote::quote! {
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config.push(quote::quote! {
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_get_code_occurence.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_enum_with_deserialize.push(quote::quote!{
                                #serde_borrow_attribute_token_stream
                                #field_ident: #code_occurence_type_with_deserialize_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config_with_deserialize.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_get_code_occurence_with_deserialize.push(quote::quote!{
                                 #field_ident
                            });
                            enum_fields_logic_for_into_serialize_deserialize_version.push(quote::quote!{
                                #field_ident
                            });
                            fields_logic_for_into_serialize_deserialize_version_for_attribute.push(quote::quote!{
                                #field_ident: #field_ident.into_serialize_deserialize_version()
                            });
                        },
                    }
                });
                let enum_fields_logic_for_source_to_string_with_config_iter = enum_fields_logic_for_source_to_string_with_config.iter();
                let enum_fields_logic_for_source_to_string_without_config_iter = enum_fields_logic_for_source_to_string_without_config.iter();
                let enum_fields_logic_for_get_code_occurence_iter = enum_fields_logic_for_get_code_occurence.iter();
                let enum_fields_logic_for_enum_with_deserialize_iter = enum_fields_logic_for_enum_with_deserialize.iter();
                let enum_fields_logic_for_source_to_string_without_config_with_deserialize_iter = enum_fields_logic_for_source_to_string_without_config_with_deserialize.iter();
                let enum_fields_logic_for_get_code_occurence_with_deserialize_iter = enum_fields_logic_for_get_code_occurence_with_deserialize.iter();
                let enum_fields_logic_for_into_serialize_deserialize_version_iter = enum_fields_logic_for_into_serialize_deserialize_version.iter();
                let mut format_logic_for_source_to_string_without_config_stringified = format_logic_for_source_to_string_without_config.iter()
                .fold(String::from(""), |mut acc, path_segment| {
                    acc.push_str(path_segment);
                    acc
                });
                let start_scope_stringified = "{{";
                let end_scope_stringified = "}}";
                let format_logic_for_source_to_string_without_config_handle_stringified = format!("\"{start_scope_stringified}\n{format_logic_for_source_to_string_without_config_stringified}{end_scope_stringified}\"");
                let format_logic_for_source_to_string_without_config_handle_token_stream = format_logic_for_source_to_string_without_config_handle_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {format_logic_for_source_to_string_without_config_handle_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                let fields_logic_for_source_to_string_without_config_for_attribute_iter = fields_logic_for_source_to_string_without_config_for_attribute.iter();
                let fields_logic_for_source_to_string_without_config_with_deserialize_for_attribute_iter = fields_logic_for_source_to_string_without_config_with_deserialize_for_attribute.iter();
                let fields_logic_for_into_serialize_deserialize_version_for_attribute_iter = fields_logic_for_into_serialize_deserialize_version_for_attribute.iter();
                logic_for_source_to_string_with_config.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_with_config_iter),*
                    } => {
                        use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
                        self.source_to_string_without_config()
                    }
                });
                logic_for_source_to_string_without_config.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_without_config_iter),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_without_config_handle_token_stream
                            ,
                            #(#fields_logic_for_source_to_string_without_config_for_attribute_iter),*
                        )
                    }
                });
                logic_for_get_code_occurence.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_get_code_occurence_iter),*
                    } => {
                        code_occurence
                    }
                });
                logic_for_enum_with_deserialize.push(quote::quote! {
                    #variant_ident {
                        #(#enum_fields_logic_for_enum_with_deserialize_iter),*
                    }
                });
                logic_for_source_to_string_without_config_with_deserialize.push(quote::quote! {
                    #ident_with_deserialize_token_stream::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_without_config_with_deserialize_iter),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_without_config_handle_token_stream
                            ,
                            #(#fields_logic_for_source_to_string_without_config_with_deserialize_for_attribute_iter),*
                        )
                    }
                });
                logic_for_get_code_occurence_with_deserialize.push(quote::quote! {
                    #ident_with_deserialize_token_stream::#variant_ident {
                        #(#enum_fields_logic_for_get_code_occurence_with_deserialize_iter),*
                    } => {
                        code_occurence
                    }
                });
                logic_for_into_serialize_deserialize_version.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_into_serialize_deserialize_version_iter),*
                    } => {
                        #ident_with_deserialize_token_stream::#variant_ident {
                            #(#fields_logic_for_into_serialize_deserialize_version_for_attribute_iter),*
                        }
                    }
                });
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
                        #config_generic_token_stream
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
                //dublicate inside names and unnamed
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
            }
        },
        SuportedEnumVariant::Unnamed => {
            let vec_variants_and_variants_types = data_enum.variants.iter().map(|variant| {
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
                let mut option_attribute = None;
                variant.attrs.iter().for_each(|attr|{
                    if let true = attr.path.segments.len() == 1 {
                        if let true = attr.path.segments[0].ident == to_string_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::ToString);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == display_foreign_type_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::DisplayForeignType);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == error_occurence_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::ErrorOccurence);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == vec_to_string_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::VecToString);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == vec_display_foreign_type_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::VecDisplayForeignType);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == vec_error_occurence_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::VecErrorOccurence);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == hashmap_key_to_string_value_to_string_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::HashMapKeyToStringValueToString);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == hashmap_key_to_string_value_display_foreign_type_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::HashMapKeyToStringValueDisplayForeignType);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == hashmap_key_to_string_value_error_occurence_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::HashMapKeyToStringValueErrorOccurence);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == hashmap_key_display_foreign_type_value_to_string_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::HashMapKeyDisplayForeignTypeValueToString);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == hashmap_key_display_foreign_type_value_display_foreign_type_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::HashMapKeyDisplayForeignTypeValueDisplayForeignType);
                            }
                        }
                        else if let true = attr.path.segments[0].ident == hashmap_key_display_foreign_type_value_error_occurence_stringified {
                            if let true = option_attribute.is_some() {
                                panic!("{proc_macro_name} {ident_stringified} two or more supported attributes!");
                            }
                            else {
                                option_attribute = Some(Attribute::HashMapKeyDisplayForeignTypeValueErrorOccurence);
                            }
                        }//other attributes are not for this proc_macro
                    }//other attributes are not for this proc_macro
                });
                let attribute = option_attribute.unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} option attribute is none"));
                (&variant.ident, type_handle, attribute)
            }).collect::<Vec<(&proc_macro2::Ident, &syn::Type, Attribute)>>();
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(vec_variants_and_variants_types.len());
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
                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type5");
                                }
                            }
                            else {
                                panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args.len() == 1 @@");
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
                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type6");
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
                                    panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type7");
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
                let (
                    logic_for_to_string_with_config_for_source_to_string_with_config_inner,
                    logic_for_to_string_without_config_inner,
                    logic_for_enum_with_deserialize_inner,
                    logic_for_to_string_without_config_with_deserialize_inner,
                    logic_for_into_serialize_deserialize_version_inner,
                ) = match attributes {
                    Attribute::ToString => {
                        let (type_token_stringified, serde_borrow_option_token_stream) = if let SupportedContainer::Path { path, should_add_serde_borrow } = supported_container {
                            (
                                format!("{path}{should_add_serde_borrow}"),
                                match should_add_serde_borrow {
                                    Lifetime::Specified(lifetime_specified) => {
                                        if let false = lifetimes_for_serialize_deserialize.contains(&lifetime_specified) {
                                            lifetimes_for_serialize_deserialize.push(lifetime_specified);
                                        };
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
                    Attribute::DisplayForeignType => {
                        (
                            quote::quote!{
                                use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                i.#display_foreign_type_token_stream().to_string()
                            },
                            quote::quote!{
                                use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                i.#display_foreign_type_token_stream().to_string()
                            },
                            quote::quote!{
                                #variant_ident(&'static str)
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
                    Attribute::ErrorOccurence => {
                        let (type_token_stringified, serde_borrow_option_token_stream) = if let SupportedContainer::Path { path, should_add_serde_borrow } = supported_container {
                            (
                                format!("{path}{with_deserialize_camel_case}{should_add_serde_borrow}"),
                                match should_add_serde_borrow {
                                    Lifetime::Specified(lifetime_specified) => {
                                        if let false = lifetimes_for_serialize_deserialize.contains(&lifetime_specified) {
                                            lifetimes_for_serialize_deserialize.push(lifetime_specified);
                                        };
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
                    Attribute::VecToString => {
                        let type_token_stringified = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                            if let Lifetime::Specified(lifetime_specified) = element_lifetime.clone() {
                                if let false = lifetimes_for_serialize_deserialize.contains(&lifetime_specified) {
                                    lifetimes_for_serialize_deserialize.push(lifetime_specified);
                                };
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
                                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                                i.vec_display_to_string()
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                                i.vec_display_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                                i.vec_display_to_string()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident(i)
                            },
                        )
                    }
                    Attribute::VecDisplayForeignType => {
                        let type_token_stringified = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                            format!("{path}<&'static str>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{vec_display_foreign_type_stringified}] only supports std::vec::Vec");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                                i.vec_display_foreign_type_to_string()
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::vec_display_foreign_type_to_string::VecDisplayForeignTypeToString;
                                i.vec_display_foreign_type_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::vec_display_to_string::VecDisplayToString;
                                i.vec_display_to_string()
                            },
                            quote::quote!{
                                 #ident_with_deserialize_token_stream::#variant_ident({
                                    use crate::traits::error_logs_logic::vec_display_foreign_type_into_vec_static_str::VecDisplayForeignTypeIntoVecStaticStr;
                                    i.vec_display_foreign_type_into_vec_static_str()
                                 })
                            },
                        )
                    }
                    Attribute::VecErrorOccurence => {
                        let type_token_stringified = if let SupportedContainer::Vec { path, element_path, element_lifetime } = supported_container {
                            if let Lifetime::Specified(lifetime_specified) = element_lifetime.clone() {
                                if let false = lifetimes_for_serialize_deserialize.contains(&lifetime_specified) {
                                    lifetimes_for_serialize_deserialize.push(lifetime_specified);
                                };
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
                                use crate::traits::error_logs_logic::vec_to_string_with_config_to_string::VecToStringWithConfigToString;
                                i.vec_to_string_with_config_to_string(config)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToString;
                                i.vec_to_string_without_config_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::vec_to_string_without_config_to_string::VecToStringWithoutConfigToStringWithDeserialize;
                                i.vec_to_string_without_config_to_string_with_deserialize()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    i
                                    .into_iter()
                                    .map(|e| e.into_serialize_deserialize_version())
                                    .collect()
                                })
                            },
                        )
                    }
                    Attribute::HashMapKeyToStringValueToString => {
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
                                (Lifetime::Specified(key_lifetime_specified), Lifetime::Specified(value_lifetime_specified)) => {
                                    if let false = lifetimes_for_serialize_deserialize.contains(&key_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(key_lifetime_specified.to_string());
                                    };
                                    if let false = lifetimes_for_serialize_deserialize.contains(&value_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(value_lifetime_specified.to_string());
                                    };
                                },
                                (Lifetime::Specified(key_lifetime_specified), Lifetime::NotSpecified) => {
                                    if let false = lifetimes_for_serialize_deserialize.contains(&key_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(key_lifetime_specified.to_string());
                                    };
                                },
                                (Lifetime::NotSpecified, Lifetime::Specified(value_lifetime_specified)) => {
                                    if let false = lifetimes_for_serialize_deserialize.contains(&value_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(value_lifetime_specified.to_string());
                                    };
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
                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                i.hashmap_display_display_to_string()
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                i.hashmap_display_display_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                i.hashmap_display_display_to_string()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident(i)
                            },
                        )
                    },
                    Attribute::HashMapKeyToStringValueDisplayForeignType => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            if let Lifetime::Specified(key_lifetime_specified) = key_lifetime_enum.clone() {
                                if let false = lifetimes_for_serialize_deserialize.contains(&key_lifetime_specified) {
                                    lifetimes_for_serialize_deserialize.push(key_lifetime_specified);
                                };
                            }
                            format!("{path}<{key_segments_stringified}{key_lifetime_enum},&'static str>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_to_string_value_display_foreign_type_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_to_string::HashMapDisplayDisplayForeignTypeToString;
                                i.hashmap_display_display_foreign_type_to_string()
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_to_string::HashMapDisplayDisplayForeignTypeToString;
                                i.hashmap_display_display_foreign_type_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                i.hashmap_display_display_to_string()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    use crate::traits::error_logs_logic::hashmap_display_display_foreign_type_into_hashmap_display_static_str::HashmapDisplayDisplayForeignTypeIntoHashmapDisplayStaticStr;
                                    i.hashmap_display_display_foreign_type_into_hashmap_display_static_str()
                                })
                            },
                        )
                    },
                    Attribute::HashMapKeyToStringValueErrorOccurence => {
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
                                (Lifetime::Specified(key_lifetime_specified), Lifetime::Specified(value_lifetime_specified)) => {
                                    if let false = lifetimes_for_serialize_deserialize.contains(&key_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(key_lifetime_specified.to_string());
                                    };
                                    if let false = lifetimes_for_serialize_deserialize.contains(&value_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(value_lifetime_specified.to_string());
                                    };
                                },
                                (Lifetime::Specified(key_lifetime_specified), Lifetime::NotSpecified) => {
                                    if let false = lifetimes_for_serialize_deserialize.contains(&key_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(key_lifetime_specified.to_string());
                                    };
                                },
                                (Lifetime::NotSpecified, Lifetime::Specified(value_lifetime_specified)) => {
                                    if let false = lifetimes_for_serialize_deserialize.contains(&value_lifetime_specified) {
                                        lifetimes_for_serialize_deserialize.push(value_lifetime_specified.to_string());
                                    };
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
                                use crate::traits::error_logs_logic::hashmap_display_to_string_with_config_to_string::HashMapDisplayToStringWithConfigToString;
                                i.hashmap_display_to_string_with_config_to_string(config)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToString;
                                i.hashmap_display_to_string_without_config_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                                i.hashmap_display_to_string_without_config_to_string_with_deserialize()
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
                    Attribute::HashMapKeyDisplayForeignTypeValueToString => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            if let Lifetime::Specified(value_lifetime_specified) = value_lifetime_enum.clone() {
                                if let false = lifetimes_for_serialize_deserialize.contains(&value_lifetime_specified) {
                                    lifetimes_for_serialize_deserialize.push(value_lifetime_specified);
                                };
                            }
                            format!("{path}<&'static str,{value_segments_stringified}{value_lifetime_enum}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_display_foreign_type_value_to_string_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_to_string::HashMapDisplayForeignTypeDisplayToString;
                                i.hashmap_display_foreign_type_display_to_string()
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_to_string::HashMapDisplayForeignTypeDisplayToString;
                                i.hashmap_display_foreign_type_display_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                i.hashmap_display_display_to_string()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_into_hashmap_static_str_display::HashmapDisplayForeignTypeDisplayIntoHashMapStaticStrDisplay;
                                    i.hashmap_display_foreign_type_display_into_hashmap_static_str_display()
                                })
                            },
                        )
                    },
                    Attribute::HashMapKeyDisplayForeignTypeValueDisplayForeignType => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            format!("{path}<&'static str,&'static str>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_display_foreign_type_value_display_foreign_type_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                                i.hashmap_display_foreign_type_display_foreign_type_to_string()
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_to_string::HashMapDisplayForeignTypeDisplayForeignTypeToString;
                                i.hashmap_display_foreign_type_display_foreign_type_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_display_to_string::HashmapDisplayDisplayToString;
                                i.hashmap_display_display_to_string()
                            },
                            quote::quote!{
                                #ident_with_deserialize_token_stream::#variant_ident({
                                    use crate::traits::error_logs_logic::hashmap_display_foreign_type_display_foreign_type_into_hashmap_static_str_static_str::HashmapDisplayForeignTypeDisplayForeignTypeIntoHashMapStaticStrStaticStr;
                                    i.hashmap_display_foreign_type_display_foreign_type_into_hashmap_static_str_static_str()
                                })
                            },
                        )
                    },
                    Attribute::HashMapKeyDisplayForeignTypeValueErrorOccurence => {
                        let type_token_stringified = if let 
                        SupportedContainer::HashMap { 
                            path,
                            key_segments_stringified, 
                            key_lifetime_enum,
                            value_segments_stringified, 
                            value_lifetime_enum,
                        }
                         = supported_container {
                            if let Lifetime::Specified(value_lifetime_specified) = value_lifetime_enum.clone() {
                                if let false = lifetimes_for_serialize_deserialize.contains(&value_lifetime_specified) {
                                    lifetimes_for_serialize_deserialize.push(value_lifetime_specified);
                                };
                            }
                            format!("{path}<&'static str,{value_segments_stringified}{with_deserialize_camel_case}{value_lifetime_enum}>")
                        }
                        else {
                            panic!("{proc_macro_name} {ident_stringified} attribute #[{hashmap_key_display_foreign_type_value_error_occurence_stringified}] only supports std::collections::HashMap");
                        };
                        let type_token_stream = type_token_stringified
                        .parse::<proc_macro2::TokenStream>()
                        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_token_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                        (
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_with_config_to_string::HashMapDisplayForeignTypeToStringWithConfigToString;
                                i.hashmap_display_foreign_type_to_string_with_config_to_string(config)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_foreign_type_to_string_without_config_to_string::HashMapDisplayForeignTypeToStringWithoutConfigToString;
                                i.hashmap_display_foreign_type_to_string_without_config_to_string()
                            },
                            quote::quote!{
                                #variant_ident(#type_token_stream)
                            },
                            quote::quote!{
                                use crate::traits::error_logs_logic::hashmap_display_to_string_without_config_to_string::HashmapDisplayToStringWithoutConfigToStringWithDeserialize;
                                i.hashmap_display_to_string_without_config_to_string_with_deserialize()
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
            let lifetimes_for_serialize_deserialize_token_stream = {
                if let true = lifetimes_for_serialize_deserialize.contains(&trait_lifetime_stringified.to_string()) {
                    panic!("{proc_macro_name} {ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
                };
                let lifetimes_for_serialize_deserialize_stringified = lifetimes_for_serialize_deserialize
                .iter()
                .fold(String::from(""), |mut acc, gen_param| {
                    acc.push_str(&format!("'{gen_param},"));
                    acc
                });
                lifetimes_for_serialize_deserialize_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {lifetimes_for_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"))
            };
            quote::quote! {
                impl<
                    #trait_lifetime_token_stream,
                    #generics
                    #config_generic_token_stream,
                >
                    #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream<
                        #trait_lifetime_token_stream,
                        #config_generic_token_stream,
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
                    #trait_lifetime_token_stream,
                    #generics
                > #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream<
                    #trait_lifetime_token_stream,
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
                pub enum #ident_with_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                    #logic_for_enum_with_deserialize
                }
                impl<
                    #trait_lifetime_token_stream,
                    #lifetimes_for_serialize_deserialize_token_stream
                >
                    #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream<
                        #trait_lifetime_token_stream
                    > 
                    for #ident_with_deserialize_token_stream<
                        #lifetimes_for_serialize_deserialize_token_stream
                    >
                {
                    fn #to_string_without_config_with_deserialize_token_stream(&self) -> String {
                        match self {
                            #logic_for_to_string_without_config_with_deserialize
                        }
                    }
                }
                impl<#generics> #ident<#generics> {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                        match self {
                            #logic_for_into_serialize_deserialize_version
                        }
                    }
                }
                //dublicate inside names and unnamed
                impl<#generics> std::fmt::Display for #ident<#generics> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                        write!(f, "{}", self.#to_string_without_config_token_stream())
                    }
                }
                impl<#lifetimes_for_serialize_deserialize_token_stream> std::fmt::Display for #ident_with_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_deserialize_token_stream;
                        write!(f, "{}", self.#to_string_without_config_with_deserialize_token_stream())
                    }
                }
            }
        },
    };
    let uuu = quote::quote! {
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
                match &angle_bracketed_generic_argument.args[0] {
                    syn::GenericArgument::Lifetime(lfmt) => Lifetime::Specified(lfmt.ident.to_string()),
                    syn::GenericArgument::Type(_) => Lifetime::NotSpecified,
                    _ => panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] supports only syn::GenericArgument::Lifetime and syn::GenericArgument::Type")
                }
            },
            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
        }
    }
    else {
        panic!("{proc_macro_name} {ident_stringified} {first_field_type_stringified_name} type_path.path.segments.last() is None");
    }
}