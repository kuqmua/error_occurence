#![deny(
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(
    ErrorOccurence, 
    attributes(
        eo_display, 
        eo_display_foreign_type,
        eo_error_occurence,
        eo_vec_display,
        eo_vec_display_foreign_type,
        eo_vec_error_occurence,
        eo_hashmap_key_display_value_display,
        eo_hashmap_key_display_value_display_foreign_type,
        eo_hashmap_key_display_value_error_occurence,
        eo_hashmap_key_display_foreign_type_value_display,
        eo_hashmap_key_display_foreign_type_value_display_foreign_type,
        eo_hashmap_key_display_foreign_type_value_error_occurence,
    )
)]
pub fn derive_error_occurence(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    std::panic::set_hook(Box::new(|panic_info| {
        if let Some(location) = panic_info.location() {
            eprintln!("ErrorOccurence panic occurred in {}:{}:{}", location.file(), location.line(), location.column());
        } else {
            eprintln!("ErrorOccurence panic occurred but can't get location information...");
        }
    }));
    

    let vec_camel_case = "Vec";
    let vec_lower_case = vec_camel_case.to_lowercase();
    let hashmap_camel_case = "HashMap";
    let hashmap_lower_case = hashmap_camel_case.to_case(convert_case::Case::Flat);
    let display_camel_case = "Display";
    let display_lower_case = display_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let foreign_type_camel_case = "ForeignType";

    let with_serialize_deserialize_camel_case = "WithSerializeDeserialize";
    let occurence_camel_case = "Occurence";
    let occurence_lower_case = occurence_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let error_camel_case = "Error";
    let error_lower_case = error_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let proc_macro_name = format!("{error_camel_case}{occurence_camel_case}");
    let error_occurence_lower_case = proc_macro_name.to_case(convert_case::Case::Snake).to_lowercase();
    let ast: syn::DeriveInput =
        syn::parse(input).unwrap_or_else(|_| panic!("{proc_macro_name} let ast: syn::DeriveInput = syn::parse(input) failed"));
    let ident = &ast.ident;
    let ident_stringified = ident.to_string();
    let parse_proc_macro2_token_stream_failed_message = ".parse::<proc_macro2::TokenStream>() failed";
    let trait_lifetime_stringified = format!("'{error_occurence_lower_case}_proc_macro_reserved_lifetime_name");
    let trait_lifetime_token_stream = trait_lifetime_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {trait_lifetime_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let code_occurence_camel_case = format!("Code{occurence_camel_case}");
    let code_occurence_lower_case = code_occurence_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let display_foreign_type_camel_case = format!("{display_camel_case}{foreign_type_camel_case}");
    let display_foreign_type_lower_case = display_foreign_type_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let display_foreign_type_lower_case_token_stream = 
    display_foreign_type_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {display_foreign_type_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let key_stringified = "key";
    let value_stringified = "value"; 
    let attribute_prefix_stringified = "eo_";
    let attribute_display_stringified = format!("{attribute_prefix_stringified}{display_lower_case}");
    let attribute_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{display_foreign_type_lower_case}");
    let attribute_error_occurence_stringified = format!("{attribute_prefix_stringified}{error_occurence_lower_case}");
    let attribute_vec_display_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_lower_case}");
    let attribute_vec_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_foreign_type_lower_case}");
    let attribute_vec_error_occurence_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{error_occurence_lower_case}");
    let attribute_hashmap_key_display_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_stringified}_{display_lower_case}_{value_stringified}_{display_lower_case}");
    let attribute_hashmap_key_display_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_stringified}_{display_lower_case}_{value_stringified}_{display_foreign_type_lower_case}");
    let attribute_hashmap_key_display_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_stringified}_{display_lower_case}_{value_stringified}_{error_occurence_lower_case}");
    let attribute_hashmap_key_display_foreign_type_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_stringified}_{display_foreign_type_lower_case}_{value_stringified}_{display_lower_case}");
    let attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_stringified}_{display_foreign_type_lower_case}_{value_stringified}_{display_foreign_type_lower_case}");
    let attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_stringified}_{display_foreign_type_lower_case}_{value_stringified}_{error_occurence_lower_case}");
    let into_camel_case = "Into";
    let into_lower_case = into_camel_case.to_lowercase();
    let string_camel_case = "String";
    let string_lower_case = string_camel_case.to_lowercase();
    let to_string_camel_case = format!("To{string_camel_case}");
    let to_string_lower_case = to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let to_string_token_stream = to_string_lower_case
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    use convert_case::Casing;
    let with_serialize_deserialize_lower_case = with_serialize_deserialize_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let ident_with_serialize_deserialize_stringified = format!("{ident}{with_serialize_deserialize_camel_case}");
    let ident_with_serialize_deserialize_token_stream = ident_with_serialize_deserialize_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {ident_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let config_camel_case = "Config";
    let config_generic_camel_case = format!("{config_camel_case}Generic");
    let config_generic_token_stream = config_generic_camel_case
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {config_generic_camel_case} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_with_config_camel_case = format!("{to_string_camel_case}With{config_camel_case}");
    let to_string_with_config_lower_case = to_string_with_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_camel_case = "Source";
    let source_lower_case = source_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_with_config_camel_case = format!("{source_camel_case}{to_string_with_config_camel_case}");
    let to_string_without_config_camel_case = format!("{to_string_camel_case}Without{config_camel_case}");
    let to_string_without_config_lower_case = to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_without_config_camel_case = format!("{source_camel_case}{to_string_without_config_camel_case}");
    let source_to_string_without_config_lower_case = source_to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_without_config_token_stream = 
    source_to_string_without_config_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {source_to_string_without_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let get_code_occurence_lower_case = format!("get_{code_occurence_lower_case}");
    let crate_traits_stringified = "crate::traits::";
    let crate_traits_fields_stringified = format!("{crate_traits_stringified}fields::");
    let crate_traits_error_logs_logic_stringified = format!("{crate_traits_stringified}error_logs_logic::");
    let lines_space_backslash_camel_case = "LinesSpaceBackslash";
    let lines_space_backslash_lower_case = lines_space_backslash_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let lines_space_backslash_lower_case_token_stream = 
    lines_space_backslash_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {lines_space_backslash_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_stringified = format!("{crate_traits_error_logs_logic_stringified}{lines_space_backslash_lower_case}::{lines_space_backslash_camel_case}");
    let crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream = 
    crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_display_foreign_type_display_foreign_type_stringified = format!("{crate_traits_stringified}{display_foreign_type_lower_case}::{display_foreign_type_camel_case}");
    let crate_traits_display_foreign_type_display_foreign_type_token_stream = 
    crate_traits_display_foreign_type_display_foreign_type_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_display_foreign_type_display_foreign_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let vec_display_to_string_camel_case = format!("{vec_camel_case}{display_camel_case}{to_string_camel_case}");
    let vec_display_to_string_lower_case = vec_display_to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let vec_display_to_string_lower_case_token_stream = 
    vec_display_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {vec_display_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_vec_display_to_string_vec_display_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{vec_display_to_string_lower_case}::{vec_display_to_string_camel_case}");
    let crate_traits_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream = 
    crate_traits_error_logs_logic_vec_display_to_string_vec_display_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_vec_display_to_string_vec_display_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let vec_display_foreign_type_to_string_camel_case = format!("{vec_camel_case}{display_foreign_type_camel_case}{to_string_camel_case}");
    let vec_display_foreign_type_to_string_lower_case = vec_display_foreign_type_to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let vec_display_foreign_type_to_string_lower_case_token_stream = 
    vec_display_foreign_type_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {vec_display_foreign_type_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{vec_display_foreign_type_to_string_lower_case}::{vec_display_foreign_type_to_string_camel_case}");
    let crate_traits_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream = 
    crate_traits_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let vec_display_foreign_type_into_vec_string_camel_case = format!("{vec_camel_case}{display_foreign_type_camel_case}{into_camel_case}{vec_camel_case}{string_camel_case}");
    let vec_display_foreign_type_into_vec_string_lower_case = vec_display_foreign_type_into_vec_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let vec_display_foreign_type_into_vec_string_lower_case_token_stream = 
    vec_display_foreign_type_into_vec_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {vec_display_foreign_type_into_vec_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{vec_display_foreign_type_into_vec_string_lower_case}::{vec_display_foreign_type_into_vec_string_camel_case}");
    let crate_traits_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_token_stream = 
    crate_traits_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let vec_to_string_without_config_to_string_camel_case = format!("{vec_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}");
    let vec_to_string_without_config_to_string_lower_case = vec_to_string_without_config_to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let vec_to_string_without_config_to_string_lower_case_token_stream = 
    vec_to_string_without_config_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {vec_to_string_without_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{vec_to_string_without_config_to_string_lower_case}::{vec_to_string_without_config_to_string_camel_case}");
    let crate_traits_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_token_stream = 
    crate_traits_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let vec_to_string_without_config_to_string_with_serialize_deserialize_camel_case = format!("{vec_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}{with_serialize_deserialize_camel_case}");
    let vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case = vec_to_string_without_config_to_string_with_serialize_deserialize_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream = 
    vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_stringified = format!("{crate_traits_error_logs_logic_stringified}{vec_to_string_without_config_to_string_lower_case}::{vec_to_string_without_config_to_string_with_serialize_deserialize_camel_case}");
    let crate_traits_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_token_stream = 
    crate_traits_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_display_to_string_camel_case = format!("{hashmap_camel_case}{display_camel_case}{display_camel_case}{to_string_camel_case}");
    let hashmap_display_display_to_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{display_lower_case}_{to_string_lower_case}");
    let hashmap_display_display_to_string_lower_case_token_stream = 
    hashmap_display_display_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_display_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_display_to_string_lower_case}::{hashmap_display_display_to_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_display_foreign_type_to_string_camel_case = format!("{hashmap_camel_case}{display_camel_case}{display_foreign_type_camel_case}{to_string_camel_case}");
    let hashmap_display_display_foreign_type_to_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{display_foreign_type_lower_case}_{to_string_lower_case}");
    let hashmap_display_display_foreign_type_to_string_lower_case_token_stream = 
    hashmap_display_display_foreign_type_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_display_foreign_type_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_display_foreign_type_to_string_lower_case}::{hashmap_display_display_foreign_type_to_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_display_foreign_type_into_hashmap_display_string_camel_case = format!("{hashmap_camel_case}{display_camel_case}{display_foreign_type_camel_case}{into_camel_case}{hashmap_camel_case}{display_camel_case}{string_camel_case}");
    let hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{display_foreign_type_lower_case}_{into_lower_case}_{hashmap_lower_case}_{display_lower_case}_{string_lower_case}");
    let hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case_token_stream = 
    hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case}::{hashmap_display_display_foreign_type_into_hashmap_display_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_to_string_without_config_to_string_camel_case = format!("{hashmap_camel_case}{display_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}");
    let hashmap_display_to_string_without_config_to_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{to_string_without_config_lower_case}_{to_string_lower_case}");
    let hashmap_display_to_string_without_config_to_string_lower_case_token_stream = 
    hashmap_display_to_string_without_config_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_to_string_without_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_to_string_without_config_to_string_lower_case}::{hashmap_display_to_string_without_config_to_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_camel_case = format!("{hashmap_camel_case}{display_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}{with_serialize_deserialize_camel_case}");
    let hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{to_string_without_config_lower_case}_{to_string_lower_case}_{with_serialize_deserialize_lower_case}");
    let hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream = 
    hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_to_string_without_config_to_string_lower_case}::{hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_foreign_type_display_to_string_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{display_camel_case}{to_string_camel_case}");
    let hashmap_display_foreign_type_display_to_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_lower_case}_{to_string_lower_case}");
    let hashmap_display_foreign_type_display_to_string_lower_case_token_stream = 
    hashmap_display_foreign_type_display_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_foreign_type_display_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_foreign_type_display_to_string_lower_case}::{hashmap_display_foreign_type_display_to_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_foreign_type_display_into_hashmap_string_display_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{display_camel_case}{into_camel_case}{hashmap_camel_case}{string_camel_case}{display_camel_case}");
    let hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_lower_case}_{into_lower_case}_{hashmap_lower_case}_{string_lower_case}_{display_lower_case}");
    let hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case_token_stream = 
    hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case}::{hashmap_display_foreign_type_display_into_hashmap_string_display_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_foreign_type_display_foreign_type_to_string_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{display_foreign_type_camel_case}{to_string_camel_case}");
    let hashmap_display_foreign_type_display_foreign_type_to_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_foreign_type_lower_case}_{to_string_lower_case}");
    let hashmap_display_foreign_type_display_foreign_type_to_string_lower_case_token_stream = 
    hashmap_display_foreign_type_display_foreign_type_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_foreign_type_display_foreign_type_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_foreign_type_display_foreign_type_to_string_lower_case}::{hashmap_display_foreign_type_display_foreign_type_to_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{display_foreign_type_camel_case}{into_camel_case}{hashmap_camel_case}{string_camel_case}{string_camel_case}");
    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_foreign_type_lower_case}_{into_lower_case}_{hashmap_lower_case}_{string_lower_case}_{string_lower_case}");
    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case_token_stream = 
    hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case}::{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let hashmap_display_foreign_type_to_string_without_config_to_string_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}");
    let hashmap_display_foreign_type_to_string_without_config_to_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{to_string_without_config_lower_case}_{to_string_lower_case}");
    let hashmap_display_foreign_type_to_string_without_config_to_string_lower_case_token_stream = 
    hashmap_display_foreign_type_to_string_without_config_to_string_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {hashmap_display_foreign_type_to_string_without_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified = format!("{crate_traits_error_logs_logic_stringified}{hashmap_display_foreign_type_to_string_without_config_to_string_lower_case}::{hashmap_display_foreign_type_to_string_without_config_to_string_camel_case}");
    let crate_traits_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_token_stream = 
    crate_traits_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{to_string_without_config_lower_case}::{to_string_without_config_camel_case}");
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_stringified = format!("{crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_stringified}{with_serialize_deserialize_camel_case}");
    let crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream = crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_fields_get_source_place_type_stringified = format!("{crate_traits_fields_stringified}Get{source_camel_case}PlaceType");
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
    let source_to_string_with_config_stringified = format!("{source_lower_case}_{to_string_with_config_lower_case}");
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
    let to_string_without_config_with_serialize_deserialize_stringified = format!("{to_string_without_config_lower_case}_{with_serialize_deserialize_lower_case}");
    let to_string_without_config_with_serialize_deserialize_token_stream = 
    to_string_without_config_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {to_string_without_config_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified = format!("{crate_traits_error_logs_logic_stringified}{get_code_occurence_lower_case}::Get{code_occurence_camel_case}");
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_stringified = format!("{crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_stringified}{with_serialize_deserialize_camel_case}");
    let crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_token_stream = 
    crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_code_occurence_code_occurence_stringified = format!("crate::common::{code_occurence_lower_case}::{code_occurence_camel_case}");
    let crate_common_code_occurence_code_occurence_token_stream = 
    crate_common_code_occurence_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_common_code_occurence_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_code_occurence_code_occurence_with_serialize_deserialize_stringified = format!("{crate_common_code_occurence_code_occurence_stringified}{with_serialize_deserialize_camel_case}");
    let crate_common_code_occurence_code_occurence_with_serialize_deserialize_token_stream = 
    crate_common_code_occurence_code_occurence_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_common_code_occurence_code_occurence_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let get_code_occurence_token_stream = 
    get_code_occurence_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {get_code_occurence_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let get_code_occurence_with_serialize_deserialize_stringified = format!("{get_code_occurence_lower_case}_{with_serialize_deserialize_lower_case}");
    let get_code_occurence_with_serialize_deserialize_token_stream = 
    get_code_occurence_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {get_code_occurence_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified = format!("{crate_traits_error_logs_logic_stringified}{source_to_string_with_config_stringified}::{source_to_string_with_config_camel_case}");

    
    
    //
    let crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream = 
    crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let into_serialize_deserialize_version_stringified = format!("{into_lower_case}_serialize_deserialize_version");
    let into_serialize_deserialize_version_token_stream = into_serialize_deserialize_version_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {into_serialize_deserialize_version_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let supported_container_double_dot_double_dot = "SupportedContainer::";
    let only_supports_supported_container_stringified = format!("only supports {supported_container_double_dot_double_dot}");
    //
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    }
    else {
        panic!("{proc_macro_name} {ident_stringified} only works with syn::Data::Enum");
    };
    let generics_len = ast.generics.params.len();
    //its really hard to support more than 1 lifetimes coz dont know how many generics would be in the WithSerializeDeserialize inner error_occurence variants and fields
    if generics_len != 1 {
        panic!("{proc_macro_name} {ident_stringified} generics_len != 1");
    }
    let generics = {
        let mut lifetimes_stringified = ast.generics.params.iter()
        .fold(String::from(""), |mut acc, gen_param| {
            if let syn::GenericParam::Lifetime(lifetime_deref) = gen_param {
                acc.push_str(&format!("'{},", lifetime_deref.lifetime.ident));
                acc
            }
            else {
                panic!("{proc_macro_name} {ident_stringified} only works with syn::GenericParam::Lifetime");
            }
        });
        lifetimes_stringified.pop();
        if let true = lifetimes_stringified.contains(&trait_lifetime_stringified) {
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
            panic!("{proc_macro_name} {ident_stringified} only works with enums where all variants are named or unnamed");
        }
    };
    match supported_enum_variant {
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
                                                        let vec_lifetime =  form_last_arg_lifetime_vec(
                                                            type_path, 
                                                            &proc_macro_name, 
                                                            &ident_stringified,
                                                        );
                                                        let code_occurence_segments_stringified = {
                                                            let mut code_occurence_type_repeat_checker = false;
                                                            let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                                            .fold(String::from(""), |mut acc, path_segment| {
                                                                let path_segment_ident = &path_segment.ident;
                                                                match *path_segment_ident == code_occurence_camel_case {
                                                                    true => {
                                                                        if code_occurence_type_repeat_checker {
                                                                            panic!("{proc_macro_name} {ident_stringified} code_occurence_ident detected more than one {code_occurence_camel_case} inside type path");
                                                                        }
                                                                        acc.push_str(&path_segment_ident.to_string());
                                                                        code_occurence_type_repeat_checker = true;
                                                                    },
                                                                    false => acc.push_str(&format!("{path_segment_ident}::")),
                                                                }
                                                                acc
                                                            });
                                                            if !code_occurence_type_repeat_checker {
                                                                panic!("{proc_macro_name} {ident_stringified} no {code_occurence_camel_case} named field");
                                                            }
                                                            code_occurence_segments_stringified_handle
                                                        };
                                                        code_occurence_type_option = Some(
                                                            (
                                                                code_occurence_segments_stringified,
                                                                vec_lifetime,
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
                                    vec_lifetime: code_occurence_lifetime
                                }
                            },
                            false => {
                                let attribute = {
                                    let mut option_attribute = None;
                                    field.attrs.iter().for_each(|attr|{
                                        if let true = attr.path.segments.len() == 1 {
                                            let two_or_more_supported_attributes_error_message = "two or more supported attributes!";
                                            if let true = attr.path.segments[0].ident == attribute_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_value_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayValueDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_value_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayValueDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_value_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayValueErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{proc_macro_name} {ident_stringified} {two_or_more_supported_attributes_error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence);
                                                }
                                            }
                                            //other attributes are not for this proc_macro
                                        }//other attributes are not for this proc_macro
                                    });
                                    option_attribute.unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} option attribute is none"))
                                };
                                let error_message = "supports only syn::Type::Path and syn::Type::Reference";
                                let str_stringified = "str";
                                let supported_container = match &field.ty {
                                    syn::Type::Array(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::BareFn(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Group(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::ImplTrait(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Infer(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Macro(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Never(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Paren(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Path(type_path) => {
                                        let path_segment = type_path.path.segments.last()
                                        .unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} type_path.path.segments.last() is None"));
                                        if path_segment.ident == vec_camel_case {
                                            let mut segments_stringified = type_path.path.segments.iter()
                                            .fold(String::from(""), |mut acc, elem| {
                                                acc.push_str(&format!("{}::", elem.ident));
                                                acc
                                            });
                                            segments_stringified.pop();
                                            segments_stringified.pop();
                                            let vec_element_type = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
                                                if let true = angle_brackets_generic_arguments.args.len() == 1 {
                                                    if let syn::GenericArgument::Type(type_handle) = &angle_brackets_generic_arguments.args[0] {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => {
                                                                let vec_lifetime = form_last_arg_lifetime_vec(
                                                                    type_path, 
                                                                    &proc_macro_name, 
                                                                    &ident_stringified,
                                                                );
                                                                let mut element_segments_stringified = type_path.path.segments.iter()
                                                                .fold(String::from(""), |mut acc, elem| {
                                                                    acc.push_str(&format!("{}::", elem.ident));
                                                                    acc
                                                                });
                                                                element_segments_stringified.pop();
                                                                element_segments_stringified.pop();
                                                                VecElementType::Path{
                                                                    element_path: element_segments_stringified,
                                                                    vec_lifetime
                                                                }
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments[0].ident.clone()
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference type_reference.elem supports only syn::Type::Path");
                                                                };
                                                                if let true = &reference_ident.to_string() == str_stringified {
                                                                    VecElementType::Reference {
                                                                        reference_ident,
                                                                        lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference lifetime is None")).ident
                                                                    }
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name} {ident_stringified} &reference_ident.to_string() != str");
                                                                }
                                                            },
                                                            _ => panic!("{proc_macro_name} {ident_stringified} type_handle supports only syn::Type::Path and syn::Type::Reference"),
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
                                                vec_element_type
                                            }
                                        }
                                        else if path_segment.ident == hashmap_camel_case {
                                            let mut segments_stringified = type_path.path.segments.iter()
                                            .fold(String::from(""), |mut acc, elem| {
                                                acc.push_str(&format!("{}::", elem.ident));
                                                acc
                                            });
                                            segments_stringified.pop();
                                            segments_stringified.pop();
                                            let (
                                                hashmap_key_type,
                                                value_segments_stringified, 
                                                vec_value_lifetime
                                            ) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = &path_segment.arguments {
                                                if let true = angle_brackets_generic_arguments.args.len() == 2 {
                                                    let
                                                    hashmap_key_type 
                                                    = if let syn::GenericArgument::Type(type_handle) = &angle_brackets_generic_arguments.args[0] {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => {
                                                                let vec_lifetime = form_last_arg_lifetime_vec(
                                                                    type_path, 
                                                                    &proc_macro_name, 
                                                                    &ident_stringified,
                                                                );
                                                                let key_segments_stringified = {
                                                                    let mut key_segments_stringified = type_path.path.segments.iter()
                                                                    .fold(String::from(""), |mut acc, elem| {
                                                                        acc.push_str(&format!("{}::", elem.ident));
                                                                        acc
                                                                    });
                                                                    key_segments_stringified.pop();
                                                                    key_segments_stringified.pop();
                                                                    key_segments_stringified
                                                                };
                                                                HashMapKeyType::Path{
                                                                    key_segments_stringified,
                                                                    vec_lifetime
                                                                }
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments[0].ident.clone()
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference type_reference.elem supports only syn::Type::Path");
                                                                };
                                                                if let true = &reference_ident.to_string() == str_stringified {
                                                                    HashMapKeyType::Reference {
                                                                        reference_ident,
                                                                        lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference lifetime is None")).ident
                                                                    }
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name} {ident_stringified} &reference_ident.to_string() != str");
                                                                }
                                                            },
                                                            _ => panic!("{proc_macro_name} {ident_stringified} type_handle supports only syn::Type::Path and syn::Type::Reference"),
                                                        }
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type2");
                                                    };
                                                    let (value_segments_stringified, value_lifetime_enum) = if let syn::GenericArgument::Type(type_handle) = &angle_brackets_generic_arguments.args[1] {
                                                        if let syn::Type::Path(type_path) = type_handle {
                                                            let vec_lifetime = form_last_arg_lifetime_vec(
                                                                type_path, 
                                                                &proc_macro_name, 
                                                                &ident_stringified,
                                                            );
                                                            let mut value_segments_stringified = type_path.path.segments.iter()
                                                            .fold(String::from(""), |mut acc, elem| {
                                                                acc.push_str(&format!("{}::", elem.ident));
                                                                acc
                                                            });
                                                            value_segments_stringified.pop();
                                                            value_segments_stringified.pop();
                                                            (value_segments_stringified, vec_lifetime)
                                                        }
                                                        else {
                                                            panic!("{proc_macro_name} {ident_stringified} type_handle supports only syn::Type::Path");
                                                        }
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name} {ident_stringified} angle_brackets_generic_arguments.args[0] supports only syn::GenericArgument::Type3");
                                                    };
                                                    (
                                                        hashmap_key_type,
                                                        value_segments_stringified, 
                                                        value_lifetime_enum
                                                    )
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
                                                hashmap_key_type,
                                                value_segments_stringified, 
                                                vec_value_lifetime
                                            }
                                        }
                                        else {
                                            let vec_lifetime = form_last_arg_lifetime_vec(
                                                type_path, 
                                                &proc_macro_name, 
                                                &ident_stringified,
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
                                                vec_lifetime,
                                            }
                                        }
                                    },
                                    syn::Type::Ptr(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Reference(type_reference) => {
                                        let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem.clone() {
                                            if let true = type_path.path.segments.len() == 1 {
                                                type_path.path.segments[0].ident.clone()
                                            }
                                            else {
                                                panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference type_path.path.segments.len() != 1");
                                            }
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference type_reference.elem supports only syn::Type::Path");
                                        };
                                        if let true = &reference_ident.to_string() == str_stringified {
                                             SupportedContainer::Reference{
                                                reference_ident,
                                                lifetime_ident: type_reference.lifetime.clone().unwrap_or_else(|| panic!("{proc_macro_name} {ident_stringified} syn::Type::Reference lifetime is None")).ident,
                                            }
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} &reference_ident.to_string() != str");
                                        }
                                    },
                                    syn::Type::Slice(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::TraitObject(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Tuple(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    syn::Type::Verbatim(_) => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                    _ => panic!("{proc_macro_name} {ident_stringified} {code_occurence_lower_case} {error_message}"),
                                };
                                ErrorOrCodeOccurence::Error {
                                    attribute,
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
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(generics_len);
            let mut logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_to_string_without_config_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            variants_vec.iter().for_each(|(
                variant_ident, 
                fields_vec
            )|{
                let mut enum_fields_logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_source_to_string_without_config_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_get_code_occurence_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut enum_fields_logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut format_logic_for_source_to_string_without_config: Vec<&str> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_without_config_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_into_serialize_deserialize_version_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_compile_time_check_error_occurence_members_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                fields_vec.into_iter().enumerate().for_each(|(index, (field_ident, error_or_code_occurence))|{
                    let unused_argument_handle_stringified = format!("_unused_argument_{index}");
                    let unused_argument_handle_token_stream = unused_argument_handle_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {unused_argument_handle_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                    match error_or_code_occurence {
                        ErrorOrCodeOccurence::Error { 
                            attribute, 
                            supported_container,
                        } => {
                            let field_name_with_field_value_token_stream = {
                                let field_name_with_field_value_stringified = format!("\"{field_ident}: {{}}\"");
                                field_name_with_field_value_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {field_name_with_field_value_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                            };
                            let std_string_string_stringified = format!("std::{string_lower_case}::{string_camel_case}");
                            let std_string_string_token_stream = std_string_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {std_string_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let ( 
                                logic_for_source_to_string_without_config_for_attribute,
                                logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute,
                                logic_for_into_serialize_deserialize_version_for_attribute,
                                field_type_with_serialize_deserialize_token_stream,
                                serde_borrow_attribute_token_stream,
                                enum_fields_logic_for_compile_time_check_error_occurence_members_used_unused,
                                logic_for_compile_time_check_error_occurence_members_for_attribute
                            ) = match attribute {
                                NamedAttribute::EoDisplay => {
                                    match supported_container {
                                        SupportedContainer::Path { path, vec_lifetime } => {
                                            let (type_token_stream, serde_borrow_token_stream) = (
                                                {
                                                    let type_stringified = format!("{path}{}", vec_lifetime_to_string(vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name,
                                                    &ident_stringified
                                                )
                                            );
                                            (
                                                quote::quote! {
                                                    {
                                                        use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    { 
                                                        use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    {
                                                        #field_ident
                                                    }
                                                },
                                                type_token_stream,
                                                serde_borrow_token_stream, 
                                                quote::quote! {
                                                    #field_ident: #unused_argument_handle_token_stream
                                                },
                                                proc_macro2::TokenStream::new(),
                                            )
                                        },
                                        SupportedContainer::Reference{ reference_ident, lifetime_ident } => {
                                            let type_token_stream = {
                                                let type_stringified = format!("&\'{lifetime_ident} {reference_ident}");
                                                type_stringified.parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                            };
                                            (
                                                quote::quote! {
                                                    {
                                                        use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    { 
                                                        use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    {
                                                        #field_ident
                                                    }
                                                },
                                                type_token_stream,
                                                quote::quote!{#[serde(borrow)]},
                                                quote::quote! {
                                                    #field_ident: #unused_argument_handle_token_stream
                                                },
                                                proc_macro2::TokenStream::new(),
                                            )
                                        },
                                        _ => panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_display_stringified}] only supports {supported_container_double_dot_double_dot}Path and {supported_container_double_dot_double_dot}Reference"),
                                    }
                                },
                                NamedAttribute::EoDisplayForeignType => {
                                    if let SupportedContainer::Path { path: _path, vec_lifetime: _vec_lifetime } = supported_container {}
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_display_foreign_type_stringified}] {only_supports_supported_container_stringified}Path");
                                    }
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#display_foreign_type_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            { 
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident
                                                )
                                                .#lines_space_backslash_lower_case_token_stream() 
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                                #field_ident.#display_foreign_type_lower_case_token_stream().#to_string_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            #std_string_string_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoErrorOccurence => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::Path { path, vec_lifetime } = supported_container {
                                        (
                                            {
                                                let type_stringified = format!("{path}{with_serialize_deserialize_camel_case}{}", vec_lifetime_to_string(vec_lifetime));
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                            }, 
                                            get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                vec_lifetime, 
                                                &mut lifetimes_for_serialize_deserialize,
                                                &trait_lifetime_stringified,
                                                &proc_macro_name,
                                                &ident_stringified
                                            )
                                        )
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_error_occurence_stringified}] {only_supports_supported_container_stringified}Path");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#to_string_without_config_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#to_string_without_config_with_serialize_deserialize_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.#into_serialize_deserialize_version_token_stream()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            {
                                                use crate::traits::error_logs_logic::error_occurence_named::ErrorOccurenceNamed;
                                                #field_ident.error_occurence_named();
                                            }
                                        },
                                    )
                                },
                                NamedAttribute::EoVecDisplay => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::Vec { 
                                        path, 
                                        vec_element_type 
                                    } = supported_container {
                                        match vec_element_type {
                                            VecElementType::Path { element_path, vec_lifetime } => (
                                                {
                                                    let type_stringified = format!("{path}<{element_path}{}>", vec_lifetime_to_string(vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name,
                                                    &ident_stringified
                                                )
                                            ),
                                            VecElementType::Reference { reference_ident, lifetime_ident } => (
                                                {
                                                    let type_stringified = format!("{path}<&'{lifetime_ident} {reference_ident}>");
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                quote::quote!{#[serde(borrow)]}
                                            ),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_vec_display_stringified}] {only_supports_supported_container_stringified}Vec");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoVecDisplayForeignType => {
                                    if let SupportedContainer::Vec { 
                                        path: _path, 
                                        vec_element_type 
                                    } = supported_container {
                                        if let VecElementType::Path { element_path: _element_path, vec_lifetime: _vec_lifetime } = vec_element_type {}
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_vec_display_foreign_type_stringified}] only supports VecElementType::Path");
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_vec_display_foreign_type_stringified}] {only_supports_supported_container_stringified}{vec_camel_case}");
                                    }
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_token_stream;
                                                #field_ident.#vec_display_foreign_type_into_vec_string_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            std::vec::Vec<#std_string_string_token_stream>
                                        },
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoVecErrorOccurence => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::Vec { 
                                        path, 
                                        vec_element_type
                                    } = supported_container {
                                        if let VecElementType::Path { element_path, vec_lifetime } = vec_element_type  {
                                            (
                                                {
                                                    let type_stringified = format!("{path}<{element_path}{with_serialize_deserialize_camel_case}{}>", vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    &vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name,
                                                    &ident_stringified
                                                )
                                            )
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_vec_error_occurence_stringified}] only supports VecElementType::Path");
                                        }                                        
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_vec_error_occurence_stringified}] {only_supports_supported_container_stringified}{vec_camel_case}");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_to_string_without_config_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|i| i.#into_serialize_deserialize_version_token_stream())
                                                .collect()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #field_ident.iter().for_each(|i|{
                                                use crate::traits::error_logs_logic::error_occurence_unnamed::ErrorOccurenceUnnamed;
                                                i.error_occurence_unnamed();
                                            });
                                        },
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayValueDisplay => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        value_segments_stringified, 
                                        vec_value_lifetime 
                                    } = supported_container {
                                        match hashmap_key_type {
                                            HashMapKeyType::Path { key_segments_stringified, vec_lifetime } => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{}, {value_segments_stringified}{}>",
                                                        vec_lifetime_to_string(vec_lifetime),
                                                        vec_lifetime_to_string(vec_value_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                    vec_lifetime.clone(), 
                                                    vec_value_lifetime.clone(), 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                        &trait_lifetime_stringified,
                                                        &proc_macro_name,
                                                        &ident_stringified,
                                                )
                                            ),
                                            HashMapKeyType::Reference { reference_ident, lifetime_ident } => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<&'{lifetime_ident} {reference_ident}, {value_segments_stringified}{}>",
                                                        vec_lifetime_to_string(vec_value_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                quote::quote!{#[serde(borrow)]}
                                            ),
                                        }
                                        
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_value_display_stringified}] {only_supports_supported_container_stringified}{hashmap_camel_case}");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayValueDisplayForeignType => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        value_segments_stringified: _value_segments_stringified, 
                                        vec_value_lifetime: _vec_value_lifetime
                                    } = supported_container {
                                        match hashmap_key_type {
                                            HashMapKeyType::Path { key_segments_stringified, vec_lifetime } => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{},{std_string_string_stringified}>",
                                                        vec_lifetime_to_string(vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name,
                                                    &ident_stringified
                                                )
                                            ),
                                            HashMapKeyType::Reference { reference_ident, lifetime_ident } => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<&'{lifetime_ident} {reference_ident},{std_string_string_stringified}>",
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                quote::quote!{#[serde(borrow)]}
                                            ),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_value_display_foreign_type_stringified}] {only_supports_supported_container_stringified}{hashmap_camel_case}");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_token_stream;
                                                #field_ident.#hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case_token_stream()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayValueErrorOccurence => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        value_segments_stringified, 
                                        vec_value_lifetime
                                    } = supported_container {
                                        match hashmap_key_type {
                                            HashMapKeyType::Path { key_segments_stringified, vec_lifetime } => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                        vec_lifetime_to_string(vec_lifetime),
                                                        vec_lifetime_to_string(vec_value_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                    vec_lifetime.clone(), 
                                                    vec_value_lifetime.clone(), 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name,
                                                    &ident_stringified,
                                                )
                                            ),
                                            HashMapKeyType::Reference { reference_ident, lifetime_ident } => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<&'{lifetime_ident} {reference_ident}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                        vec_lifetime_to_string(vec_value_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                quote::quote!{#[serde(borrow)]}
                                            ),
                                        }
                                        
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_value_error_occurence_stringified}] {only_supports_supported_container_stringified}{hashmap_camel_case}");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_to_string_without_config_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|(k, v)| (k, { v.#into_serialize_deserialize_version_token_stream() }))
                                                .collect()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #field_ident.values().for_each(|v|{
                                                use crate::traits::error_logs_logic::error_occurence_unnamed::ErrorOccurenceUnnamed;
                                                v.error_occurence_unnamed();
                                            });
                                        },
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::HashMap { 
                                        path, 
                                        hashmap_key_type,
                                        value_segments_stringified, 
                                        vec_value_lifetime 
                                    } = supported_container {
                                        if let HashMapKeyType::Path { key_segments_stringified: _key_segments_stringified, vec_lifetime: _vec_lifetime } = hashmap_key_type {
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                        vec_lifetime_to_string(vec_value_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_value_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name,
                                                    &ident_stringified
                                                )
                                            )
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_foreign_type_value_display_stringified}] only supports HashMapKeyType::Path");
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_foreign_type_value_display_stringified}] {only_supports_supported_container_stringified}{hashmap_camel_case}");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_token_stream;
                                                #field_ident.#hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case_token_stream()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType => {
                                    let type_token_stream = if let SupportedContainer::HashMap { 
                                        path, 
                                        hashmap_key_type,
                                        value_segments_stringified: _value_segments_stringified, 
                                        vec_value_lifetime: _vec_value_lifetime 
                                    } = supported_container {
                                        if let HashMapKeyType::Path { key_segments_stringified: _key_segments_stringified, vec_lifetime: _vec_lifetime } = hashmap_key_type {
                                            let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified}] only supports HashMapKeyType::Path");
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified}] {only_supports_supported_container_stringified}{hashmap_camel_case}");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_token_stream;
                                                #field_ident.#hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case_token_stream()
                                            }
                                        },
                                        type_token_stream,
                                        proc_macro2::TokenStream::new(),
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::HashMap { 
                                        path, 
                                        hashmap_key_type,
                                        value_segments_stringified, 
                                        vec_value_lifetime 
                                    } = supported_container {
                                        if let HashMapKeyType::Path { key_segments_stringified: _key_segments_stringified, vec_lifetime: _vec_lifetime } = hashmap_key_type {
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                        vec_lifetime_to_string(vec_value_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_value_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name,
                                                    &ident_stringified
                                                )
                                            )
                                        }
                                        else {
                                            panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified}] only supports HashMapKeyType::Path");
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified}] {only_supports_supported_container_stringified}{hashmap_camel_case}");
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_to_string_without_config_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_traits_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_traits_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                #field_ident.into_iter()
                                                .map(|(k, v)| {
                                                    (
                                                        {
                                                            use #crate_traits_display_foreign_type_display_foreign_type_token_stream;
                                                            k.#display_foreign_type_lower_case_token_stream().#to_string_token_stream()
                                                        },
                                                        { v.#into_serialize_deserialize_version_token_stream() },
                                                    )
                                                })
                                                .collect()
                                            }
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #field_ident.values().for_each(|v|{
                                                use crate::traits::error_logs_logic::error_occurence_unnamed::ErrorOccurenceUnnamed;
                                                v.error_occurence_unnamed();
                                            });
                                        },
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
                            enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                                #serde_borrow_attribute_token_stream
                                #field_ident: #field_type_with_serialize_deserialize_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config_with_serialize_deserialize.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_get_code_occurence_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_into_serialize_deserialize_version.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_compile_time_check_error_occurence_members.push(quote::quote!{
                                #enum_fields_logic_for_compile_time_check_error_occurence_members_used_unused
                            });
                            format_logic_for_source_to_string_without_config.push("{}");
                            fields_logic_for_source_to_string_without_config_for_attribute.push(logic_for_source_to_string_without_config_for_attribute);
                            fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute.push(logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute);
                            fields_logic_for_into_serialize_deserialize_version_for_attribute.push(quote::quote!{
                                #field_ident: #logic_for_into_serialize_deserialize_version_for_attribute
                            });
                            fields_logic_for_compile_time_check_error_occurence_members_for_attribute.push(quote::quote!{
                                #logic_for_compile_time_check_error_occurence_members_for_attribute
                            });
                        },
                        ErrorOrCodeOccurence::CodeOccurence { 
                            field_type,
                            vec_lifetime,
                         } => {
                            let serde_borrow_attribute_token_stream = get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                vec_lifetime, 
                                &mut lifetimes_for_serialize_deserialize,
                                &trait_lifetime_stringified,
                                &proc_macro_name,
                                &ident_stringified
                            );
                            let code_occurence_type_with_serialize_deserialize_token_stream = {
                                let code_occurence_type_with_serialize_deserialize_stringified = format!("{field_type}{with_serialize_deserialize_camel_case}{}", vec_lifetime_to_string(vec_lifetime));
                                code_occurence_type_with_serialize_deserialize_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {code_occurence_type_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                            };
                            enum_fields_logic_for_source_to_string_with_config.push(quote::quote! {
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config.push(quote::quote! {
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_get_code_occurence.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                                #serde_borrow_attribute_token_stream
                                #field_ident: #code_occurence_type_with_serialize_deserialize_token_stream
                            });
                            enum_fields_logic_for_source_to_string_without_config_with_serialize_deserialize.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_get_code_occurence_with_serialize_deserialize.push(quote::quote!{
                                 #field_ident
                            });
                            enum_fields_logic_for_into_serialize_deserialize_version.push(quote::quote!{
                                #field_ident
                            });
                            enum_fields_logic_for_compile_time_check_error_occurence_members.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            fields_logic_for_into_serialize_deserialize_version_for_attribute.push(quote::quote!{
                                #field_ident: #field_ident.#into_serialize_deserialize_version_token_stream()
                            });
                            fields_logic_for_compile_time_check_error_occurence_members_for_attribute.push(proc_macro2::TokenStream::new());
                        },
                    }
                });
                let enum_fields_logic_for_source_to_string_with_config_iter = enum_fields_logic_for_source_to_string_with_config.iter();
                let enum_fields_logic_for_source_to_string_without_config_iter = enum_fields_logic_for_source_to_string_without_config.iter();
                let enum_fields_logic_for_get_code_occurence_iter = enum_fields_logic_for_get_code_occurence.iter();
                let enum_fields_logic_for_enum_with_serialize_deserialize_iter = enum_fields_logic_for_enum_with_serialize_deserialize.iter();
                let enum_fields_logic_for_source_to_string_without_config_with_serialize_deserialize_iter = enum_fields_logic_for_source_to_string_without_config_with_serialize_deserialize.iter();
                let enum_fields_logic_for_get_code_occurence_with_serialize_deserialize_iter = enum_fields_logic_for_get_code_occurence_with_serialize_deserialize.iter();
                let enum_fields_logic_for_into_serialize_deserialize_version_iter = enum_fields_logic_for_into_serialize_deserialize_version.iter();
                let enum_fields_logic_for_compile_time_check_error_occurence_members_iter = enum_fields_logic_for_compile_time_check_error_occurence_members.iter();
                let format_logic_for_source_to_string_without_config_stringified = format_logic_for_source_to_string_without_config.iter()
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
                let fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute_iter = fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute.iter();
                let fields_logic_for_into_serialize_deserialize_version_for_attribute_iter = fields_logic_for_into_serialize_deserialize_version_for_attribute.iter();
                let fields_logic_for_compile_time_check_error_occurence_members_iter = fields_logic_for_compile_time_check_error_occurence_members_for_attribute.iter();
                logic_for_source_to_string_with_config.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_with_config_iter),*
                    } => {
                        use #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream;
                        self.#source_to_string_without_config_token_stream()
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
                logic_for_enum_with_serialize_deserialize.push(quote::quote! {
                    #variant_ident {
                        #(#enum_fields_logic_for_enum_with_serialize_deserialize_iter),*
                    }
                });
                logic_for_source_to_string_without_config_with_serialize_deserialize.push(quote::quote! {
                    #ident_with_serialize_deserialize_token_stream::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_without_config_with_serialize_deserialize_iter),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_without_config_handle_token_stream
                            ,
                            #(#fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute_iter),*
                        )
                    }
                });
                logic_for_get_code_occurence_with_serialize_deserialize.push(quote::quote! {
                    #ident_with_serialize_deserialize_token_stream::#variant_ident {
                        #(#enum_fields_logic_for_get_code_occurence_with_serialize_deserialize_iter),*
                    } => {
                        code_occurence
                    }
                });
                logic_for_into_serialize_deserialize_version.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_into_serialize_deserialize_version_iter),*
                    } => {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident {
                            #(#fields_logic_for_into_serialize_deserialize_version_for_attribute_iter),*
                        }
                    }
                });
                logic_for_compile_time_check_error_occurence_members.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_compile_time_check_error_occurence_members_iter),*
                    } => {
                        #(#fields_logic_for_compile_time_check_error_occurence_members_iter)*
                    }
                });
            });
            let logic_for_source_to_string_with_config_iter = logic_for_source_to_string_with_config.iter();
            let logic_for_source_to_string_without_config_iter = logic_for_source_to_string_without_config.iter();
            let logic_for_get_code_occurence_iter = logic_for_get_code_occurence.iter();
            let logic_for_enum_with_serialize_deserialize_iter = logic_for_enum_with_serialize_deserialize.iter();
            let logic_for_source_to_string_without_config_with_serialize_deserialize_iter = logic_for_source_to_string_without_config_with_serialize_deserialize.iter();
            let logic_for_get_code_occurence_with_serialize_deserialize_iter = logic_for_get_code_occurence_with_serialize_deserialize.iter();
            let logic_for_into_serialize_deserialize_version_iter = logic_for_into_serialize_deserialize_version.iter();
            let logic_for_compile_time_check_error_occurence_members_iter = logic_for_compile_time_check_error_occurence_members.iter();
            let logic_for_source_to_string_with_config = quote::quote! {
                #(#logic_for_source_to_string_with_config_iter),*
            };
            let logic_for_source_to_string_without_config = quote::quote! {
                #(#logic_for_source_to_string_without_config_iter),*
            };
            let logic_for_get_code_occurence = quote::quote! {
                #(#logic_for_get_code_occurence_iter),*
            };
            let logic_for_enum_with_serialize_deserialize = quote::quote! {
                #(#logic_for_enum_with_serialize_deserialize_iter),*
            };
            let logic_for_source_to_string_without_config_with_serialize_deserialize = quote::quote! {
                #(#logic_for_source_to_string_without_config_with_serialize_deserialize_iter),*
            };
            let logic_for_get_code_occurence_with_serialize_deserialize = quote::quote! {
                #(#logic_for_get_code_occurence_with_serialize_deserialize_iter),*
            };
            let logic_for_into_serialize_deserialize_version = quote::quote! {
                #(#logic_for_into_serialize_deserialize_version_iter),*
            };
            let logic_for_compile_time_check_error_occurence_members = quote::quote! {
                #(#logic_for_compile_time_check_error_occurence_members_iter),*
            };
            let lifetimes_for_serialize_deserialize_token_stream = lifetimes_for_serialize_deserialize_into_token_stream(
                lifetimes_for_serialize_deserialize,
                &trait_lifetime_stringified,
                &proc_macro_name, 
                &ident_stringified,
                parse_proc_macro2_token_stream_failed_message,
            );
            quote::quote! {
                impl<
                    #trait_lifetime_token_stream,
                    #generics,
                    #config_generic_token_stream
                >
                    #crate_traits_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream<
                        #trait_lifetime_token_stream,
                        #config_generic_token_stream
                    > for #ident<#generics>
                    where #config_generic_token_stream: #crate_traits_fields_get_source_place_type_token_stream
                        + #crate_traits_fields_get_timezone_token_stream
                        + #crate_traits_get_server_address_get_server_address_token_stream,
                {
                    fn #source_to_string_with_config_token_stream(
                        &self,
                        config: &#config_generic_token_stream 
                    ) -> String {
                        match self {
                            #logic_for_source_to_string_with_config
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream,
                    #generics
                >
                    #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<
                        #trait_lifetime_token_stream
                    > for #ident<#generics>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream,
                    #generics
                > 
                    #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_token_stream<
                        #trait_lifetime_token_stream
                    >
                    for #ident<#generics>
                {
                    fn #get_code_occurence_token_stream(&self) -> &#crate_common_code_occurence_code_occurence_token_stream
                    {
                        match self {
                            #logic_for_get_code_occurence
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
                pub enum #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                    #logic_for_enum_with_serialize_deserialize
                }
                impl<
                    #trait_lifetime_token_stream,
                    #lifetimes_for_serialize_deserialize_token_stream
                > #crate_traits_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<
                    #trait_lifetime_token_stream
                > for #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #logic_for_source_to_string_without_config_with_serialize_deserialize
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream,
                    #lifetimes_for_serialize_deserialize_token_stream
                > #crate_traits_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_token_stream<
                    #trait_lifetime_token_stream
                >
                    for #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream>
                {
                    fn #get_code_occurence_with_serialize_deserialize_token_stream(
                        &self,
                    ) -> &#crate_common_code_occurence_code_occurence_with_serialize_deserialize_token_stream
                    {
                        match self {
                            #logic_for_get_code_occurence_with_serialize_deserialize
                        }
                    }
                }
                impl<#generics> #ident<#generics> {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                        match self {
                            #logic_for_into_serialize_deserialize_version
                        }
                    }
                }
                impl<#generics> std::fmt::Display for #ident<#generics> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                        write!(f, "{}", self.#to_string_without_config_token_stream())
                    }
                }
                impl<#lifetimes_for_serialize_deserialize_token_stream> std::fmt::Display for #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream;
                        write!(f, "{}", self.#to_string_without_config_with_serialize_deserialize_token_stream())
                    }
                }
                impl<#generics> crate::traits::error_logs_logic::error_occurence_named::ErrorOccurenceNamed for #ident<#generics> {
                    fn error_occurence_named(&self) -> () {
                        ()
                    }
                }
                impl<#generics> #ident<#generics> {
                    fn compile_time_check_error_occurence_members(&self) {
                        match self {
                               #logic_for_compile_time_check_error_occurence_members
                        }
                    }
                }
            }.into()
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
                (&variant.ident, type_handle)
            }).collect::<Vec<(&proc_macro2::Ident, &syn::Type)>>();
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(generics_len);
            let mut logic_for_to_string_with_config_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_to_string_without_config_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            let mut logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(vec_variants_and_variants_types.len());
            vec_variants_and_variants_types.iter().for_each(|(
                variant_ident, 
                field_type, 
            )|{
                let supported_container = if let syn::Type::Path(type_path) = field_type {
                    let vec_lifetime = form_last_arg_lifetime_vec(
                        type_path, 
                        &proc_macro_name, 
                        &ident_stringified,
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
                        vec_lifetime,
                    }
                }
                else {
                    panic!("{proc_macro_name} {ident_stringified} supports only syn::Type::Path")
                };
                let (
                    logic_for_to_string_with_config_for_source_to_string_with_config_inner,
                    logic_for_to_string_without_config_inner,
                    logic_for_enum_with_serialize_deserialize_inner,
                    logic_for_to_string_without_config_with_serialize_deserialize_inner,
                    logic_for_into_serialize_deserialize_version_inner,
                    logic_for_compile_time_check_error_occurence_members_inner
                ) = {
                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::Path { path, vec_lifetime } = supported_container {
                        (
                            {
                                let type_stringified = format!(
                                    "{path}{with_serialize_deserialize_camel_case}{}",
                                    vec_lifetime_to_string(&vec_lifetime)
                                );
                                type_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                            },
                            get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                &vec_lifetime, 
                                &mut lifetimes_for_serialize_deserialize,
                                &trait_lifetime_stringified,
                                &proc_macro_name,
                                &ident_stringified,
                            )
                        )
                    }
                    else {
                        panic!("{proc_macro_name} {ident_stringified} attribute #[{attribute_error_occurence_stringified}] {only_supports_supported_container_stringified}Path");
                    };
                    (
                        quote::quote!{
                            i.#to_string_with_config_for_source_to_string_with_config_token_stream(config)
                        },
                        quote::quote!{
                            i.#to_string_without_config_token_stream()
                        },
                        quote::quote!{
                            #serde_borrow_token_stream
                            #variant_ident(#type_token_stream)
                        },
                        quote::quote!{
                            i.#to_string_without_config_with_serialize_deserialize_token_stream()
                        },
                        quote::quote!{
                            #ident_with_serialize_deserialize_token_stream::#variant_ident(i.#into_serialize_deserialize_version_token_stream())
                        },
                        quote::quote!{
                            {
                                use crate::traits::error_logs_logic::error_occurence_named::ErrorOccurenceNamed;
                                i.error_occurence_named();
                            }
                        }
                    )
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
                logic_for_enum_with_serialize_deserialize.push({
                    quote::quote!{
                        #logic_for_enum_with_serialize_deserialize_inner
                    }
                });
                logic_for_to_string_without_config_with_serialize_deserialize.push(quote::quote!{
                    #ident_with_serialize_deserialize_token_stream::#variant_ident(i) => {
                        #logic_for_to_string_without_config_with_serialize_deserialize_inner
                    }
                });
                logic_for_into_serialize_deserialize_version.push(quote::quote!{
                     #ident::#variant_ident(i) => {
                        #logic_for_into_serialize_deserialize_version_inner
                     }
                });
                logic_for_compile_time_check_error_occurence_members.push(quote::quote!{
                     #ident::#variant_ident(i) => {
                        #logic_for_compile_time_check_error_occurence_members_inner
                     }
                });
            });
            let logic_for_to_string_with_config_for_source_to_string_with_config_generated = logic_for_to_string_with_config_for_source_to_string_with_config.iter();
            let logic_for_to_string_without_config_generated = logic_for_to_string_without_config.iter();
            let logic_for_enum_with_serialize_deserialize_generated = logic_for_enum_with_serialize_deserialize.iter();
            let logic_for_to_string_without_config_with_serialize_deserialize_generated = logic_for_to_string_without_config_with_serialize_deserialize.iter();
            let logic_for_into_serialize_deserialize_version_generated = logic_for_into_serialize_deserialize_version.iter();
            let logic_for_compile_time_check_error_occurence_members_generated = logic_for_compile_time_check_error_occurence_members.iter();
            let logic_for_to_string_with_config_for_source_to_string_with_config = quote::quote! {
                #(#logic_for_to_string_with_config_for_source_to_string_with_config_generated),*
            };
            let logic_for_to_string_without_config = quote::quote! {
                #(#logic_for_to_string_without_config_generated),*
            };
            let logic_for_enum_with_serialize_deserialize = quote::quote! {
                #(#logic_for_enum_with_serialize_deserialize_generated),*
            };
            let logic_for_to_string_without_config_with_serialize_deserialize = quote::quote! {
                #(#logic_for_to_string_without_config_with_serialize_deserialize_generated),*
            };
            let logic_for_into_serialize_deserialize_version = quote::quote! {
                #(#logic_for_into_serialize_deserialize_version_generated),*
            };
            let logic_for_compile_time_check_error_occurence_members = quote::quote! {
                #(#logic_for_compile_time_check_error_occurence_members_generated),*
            };
            let lifetimes_for_serialize_deserialize_token_stream = lifetimes_for_serialize_deserialize_into_token_stream(
                lifetimes_for_serialize_deserialize,
                &trait_lifetime_stringified,
                &proc_macro_name, 
                &ident_stringified,
                parse_proc_macro2_token_stream_failed_message,
            );
            quote::quote! {
                impl<
                    #trait_lifetime_token_stream,
                    #generics,
                    #config_generic_token_stream
                >
                    #crate_traits_error_logs_logic_to_string_with_config_to_string_with_config_for_source_to_string_with_config_token_stream<
                        #trait_lifetime_token_stream,
                        #config_generic_token_stream
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
                    #trait_lifetime_token_stream
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
                pub enum #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                    #logic_for_enum_with_serialize_deserialize
                }
                impl<
                    #trait_lifetime_token_stream,
                    #lifetimes_for_serialize_deserialize_token_stream
                >
                    #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream<
                        #trait_lifetime_token_stream
                    > 
                    for #ident_with_serialize_deserialize_token_stream<
                        #lifetimes_for_serialize_deserialize_token_stream
                    >
                {
                    fn #to_string_without_config_with_serialize_deserialize_token_stream(&self) -> String {
                        match self {
                            #logic_for_to_string_without_config_with_serialize_deserialize
                        }
                    }
                }
                impl<#generics> #ident<#generics> {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                        match self {
                            #logic_for_into_serialize_deserialize_version
                        }
                    }
                }
                impl<#generics> std::fmt::Display for #ident<#generics> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                        write!(f, "{}", self.#to_string_without_config_token_stream())
                    }
                }
                impl<#lifetimes_for_serialize_deserialize_token_stream> std::fmt::Display for #ident_with_serialize_deserialize_token_stream<#lifetimes_for_serialize_deserialize_token_stream> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_traits_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream;
                        write!(f, "{}", self.#to_string_without_config_with_serialize_deserialize_token_stream())
                    }
                }
                impl<#generics> crate::traits::error_logs_logic::error_occurence_unnamed::ErrorOccurenceUnnamed for #ident<#generics> {
                    fn error_occurence_unnamed(&self) -> () {
                        ()
                    }
                }
                impl<#generics> #ident<#generics> {
                    fn compile_time_check_error_occurence_members(&self) {
                        match self {
                            #logic_for_compile_time_check_error_occurence_members
                        }
                    }
                }
            }.into()
        },
    }
}


enum SuportedEnumVariant {
    Named,
    Unnamed,
}

enum SupportedContainer {
    Vec{
        path: String,
        vec_element_type: VecElementType
    },
    HashMap{
        path: String,
        hashmap_key_type: HashMapKeyType,
        value_segments_stringified: String, 
        vec_value_lifetime: Vec<Lifetime>
    },
    Path{
        path: String, 
        vec_lifetime: Vec<Lifetime>,
    },
    Reference{
        reference_ident: proc_macro2::Ident,
        lifetime_ident: proc_macro2::Ident, 
    },
}

fn get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
    vec_lifetime: &Vec<Lifetime>, 
    lifetimes_for_serialize_deserialize: &mut Vec<String>,
    trait_lifetime_stringified: &String,
    proc_macro_name: &String,
    ident_stringified: &String
) -> proc_macro2::TokenStream {
    vec_lifetime.iter().for_each(|k|{
        if let Lifetime::Specified(specified_lifetime) = k {
            if let true = specified_lifetime == &trait_lifetime_stringified.to_string() {
                panic!("{proc_macro_name} {ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
            }
            if let false = lifetimes_for_serialize_deserialize.contains(&specified_lifetime) {
                lifetimes_for_serialize_deserialize.push(specified_lifetime.clone());
            }
        }
    });
    match vec_lifetime_to_lifetime(&vec_lifetime) {
        Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
        Lifetime::NotSpecified => proc_macro2::TokenStream::new(),
    }
}

fn get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
    key_vec_lifetime: Vec<Lifetime>, 
    value_vec_lifetime: Vec<Lifetime>, 
    lifetimes_for_serialize_deserialize: &mut Vec<String>,
    trait_lifetime_stringified: &String,
    proc_macro_name: &String,
    ident_stringified: &String,
) -> proc_macro2::TokenStream {
    key_vec_lifetime.iter().for_each(|k|{
        if let Lifetime::Specified(key_lifetime_specified) = k {
            if let true = key_lifetime_specified == &trait_lifetime_stringified.to_string() {
                panic!("{proc_macro_name} {ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
            }
            if let false = lifetimes_for_serialize_deserialize.contains(&key_lifetime_specified) {
                lifetimes_for_serialize_deserialize.push(key_lifetime_specified.clone());
            }
        }
    });
    value_vec_lifetime.iter().for_each(|v|{
        if let Lifetime::Specified(value_lifetime_specified) = v {
            if let true = value_lifetime_specified == &trait_lifetime_stringified.to_string() {
                panic!("{proc_macro_name} {ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
            }
            if let false = lifetimes_for_serialize_deserialize.contains(&value_lifetime_specified) {
                lifetimes_for_serialize_deserialize.push(value_lifetime_specified.clone());
            }
        }
    });
    match (vec_lifetime_to_lifetime(&key_vec_lifetime), vec_lifetime_to_lifetime(&value_vec_lifetime)) {
        (Lifetime::Specified(_), Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
        (Lifetime::Specified(_), Lifetime::NotSpecified) => quote::quote!{#[serde(borrow)]},
        (Lifetime::NotSpecified, Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
        (Lifetime::NotSpecified, Lifetime::NotSpecified) => proc_macro2::TokenStream::new(),
    }
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
            Lifetime::Specified(l) => write!(f, "'{l}"),
            Lifetime::NotSpecified => write!(f, ""),
        }
    }
}

fn vec_lifetime_to_string(vec: &Vec<Lifetime>) -> String {
    let mut lifetimes_stringified_handle = vec.iter().fold(String::from(""), |mut acc, path_segment| {
        acc.push_str(&format!("{},", path_segment));
        acc
    });
    lifetimes_stringified_handle.pop();
    format!("<{lifetimes_stringified_handle}>")
}

fn vec_lifetime_to_lifetime(vec: &Vec<Lifetime>) -> Lifetime {
    let mut lifetime_handle = Lifetime::NotSpecified;
    for lft in vec {
        if let Lifetime::Specified(_) = lft {
            lifetime_handle = lft.clone();
            break;
        }
    }
    lifetime_handle
}

enum ErrorOrCodeOccurence {
    Error {
        attribute: NamedAttribute,
        supported_container: SupportedContainer,
    },
    CodeOccurence {
        field_type: String,
        vec_lifetime: Vec<Lifetime>
    }
}

enum VecElementType {
    Path{
        element_path: String,
        vec_lifetime: Vec<Lifetime>
    },
    Reference {
        reference_ident: proc_macro2::Ident,
        lifetime_ident: proc_macro2::Ident
    }
}

enum HashMapKeyType {
    Path{
        key_segments_stringified: String,
        vec_lifetime: Vec<Lifetime>
    },
    Reference {
        reference_ident: proc_macro2::Ident,
        lifetime_ident: proc_macro2::Ident
    }
}

#[derive(
    Debug,
    strum_macros::EnumIter,
    strum_macros::Display,
    enum_extension::EnumExtension
)]
enum NamedAttribute {
    EoDisplay,
    EoDisplayForeignType,
    EoErrorOccurence,
    EoVecDisplay,
    EoVecDisplayForeignType,
    EoVecErrorOccurence,
    EoHashMapKeyDisplayValueDisplay,
    EoHashMapKeyDisplayValueDisplayForeignType,
    EoHashMapKeyDisplayValueErrorOccurence,
    EoHashMapKeyDisplayForeignTypeValueDisplay,
    EoHashMapKeyDisplayForeignTypeValueDisplayForeignType,
    EoHashMapKeyDisplayForeignTypeValueErrorOccurence,
}

fn form_last_arg_lifetime_vec(
    type_path_handle: &syn::TypePath, 
    proc_macro_name: &String, 
    ident_stringified: &String,
) -> Vec<Lifetime> {
    if let Some(path_segment) = type_path_handle.path.segments.last() {
        match &path_segment.arguments {
            syn::PathArguments::None => Vec::new(),
            syn::PathArguments::AngleBracketed(angle_bracketed_generic_argument) => {
                angle_bracketed_generic_argument.args.iter().map(|generic_argument|{
                    match generic_argument {
                        syn::GenericArgument::Lifetime(lfmt) => Lifetime::Specified(lfmt.ident.to_string()),
                        syn::GenericArgument::Type(_) => Lifetime::NotSpecified,
                        _ => panic!("{proc_macro_name} {ident_stringified} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] supports only syn::GenericArgument::Lifetime and syn::GenericArgument::Type")
                    }
                })
                .collect()
            },
            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name} {ident_stringified} type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
        }
    }
    else {
        panic!("{proc_macro_name} {ident_stringified} type_path.path.segments.last() is None");
    }
}

fn lifetimes_for_serialize_deserialize_into_token_stream(
    lifetimes_for_serialize_deserialize: Vec<String>,
    trait_lifetime_stringified: &String,
    proc_macro_name: &String, 
    ident_stringified: &String,
    parse_proc_macro2_token_stream_failed_message: &str,
) -> proc_macro2::TokenStream {
    if let true = lifetimes_for_serialize_deserialize.contains(&trait_lifetime_stringified.to_string()) {
        panic!("{proc_macro_name} {ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
    };
    let mut lifetimes_for_serialize_deserialize_stringified = lifetimes_for_serialize_deserialize
    .iter()
    .fold(String::from(""), |mut acc, gen_param| {
        acc.push_str(&format!("'{gen_param},"));
        acc
    });
    lifetimes_for_serialize_deserialize_stringified.pop();
    lifetimes_for_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name} {ident_stringified} {lifetimes_for_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"))
}
