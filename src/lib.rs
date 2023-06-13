#![deny(
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

//todo change how hashmap shows in console
//todo maybe structs that are enums or containing enums - maybe convert them not into String, but some custom type that copies all logic of the type?
//todo maybe add multiple lifetimes supports with attribute parameters like this 
// #[derive(Serialize)]
// struct Foo {
//     #[doc = include_str!("x.md")]
//     x: u32
// }
//todo - maybe remove possibility to use references for display, display_foreign_type, error occurence for WithSerializeDeserialize
#[proc_macro_derive(
    ErrorOccurence, 
    attributes(
        eo_display,
        eo_display_with_serialize_deserialize, 
        eo_display_foreign_type,
        eo_display_foreign_type_with_serialize_deserialize,
        eo_error_occurence,
        eo_vec_display,
        eo_vec_display_with_serialize_deserialize,
        eo_vec_display_foreign_type,
        eo_vec_display_foreign_type_with_serialize_deserialize,
        eo_vec_error_occurence,
        eo_hashmap_key_display_with_serialize_deserialize_value_display,
        eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize,
        eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type,
        eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize,
        eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence,
        eo_hashmap_key_display_foreign_type_value_display,
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize,
        eo_hashmap_key_display_foreign_type_value_display_foreign_type,
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize,
        eo_hashmap_key_display_foreign_type_value_error_occurence,
    )
)]
pub fn error_occurence(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    use convert_case::Casing;
    let error_camel_case = "Error";
    let occurence_camel_case = "Occurence";
    let proc_macro_name = format!("{error_camel_case}{occurence_camel_case}");
    let ast: syn::DeriveInput =
        syn::parse(input).unwrap_or_else(|_| panic!("{proc_macro_name} let ast: syn::DeriveInput = syn::parse(input) failed"));
    let parse_proc_macro2_token_stream_failed_message = ".parse::<proc_macro2::TokenStream>() failed";
    let error_occurence_lower_case = proc_macro_name.to_case(convert_case::Case::Snake).to_lowercase();
    let trait_lifetime_stringified = format!("'{error_occurence_lower_case}_proc_macro_reserved_lifetime_name");
    let ident = &ast.ident;
    let ident_stringified = ident.to_string();
    let supports_only_stringified = "supports only";
    let proc_macro_name_ident_stringified = format!("{proc_macro_name} {ident_stringified}");
    let data_enum = if let syn::Data::Enum(data_enum) = ast.data {
        data_enum
    }
    else {
        panic!("{proc_macro_name_ident_stringified} {supports_only_stringified} syn::Data::Enum");
    };
    let generics_len = ast.generics.params.len();
    //its really hard to support more than 1 lifetimes coz dont know how many generics would be in the WithSerializeDeserialize inner error_occurence variants and fields
    if generics_len != 1 {
        panic!("{proc_macro_name_ident_stringified} generics_len != 1");
    }
    let generics = {
        let mut lifetimes_stringified = ast.generics.params.iter()
        .fold(String::from(""), |mut acc, gen_param| {
            if let syn::GenericParam::Lifetime(lifetime_deref) = gen_param {
                acc.push_str(&format!("'{},", lifetime_deref.lifetime.ident));
                acc
            }
            else {
                panic!("{proc_macro_name_ident_stringified} {supports_only_stringified} syn::GenericParam::Lifetime");
            }
        });
        lifetimes_stringified.pop();
        if let true = lifetimes_stringified.contains(&trait_lifetime_stringified) {
            panic!("{proc_macro_name_ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
        }
        lifetimes_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {lifetimes_stringified} {parse_proc_macro2_token_stream_failed_message}"))
    };
    let syn_fields = "syn::Fields";
    let named_camel_case = "Named";
    let named_lower_case = named_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let unnamed_camel_case = format!("Un{named_lower_case}");
    let supported_enum_variant = {
        let mut all_equal: Option<SuportedEnumVariant> = None;
        if let true = &data_enum.variants.is_empty() {
            panic!("{proc_macro_name_ident_stringified} enum variants are empty");
        }
        let error_message = format!("{proc_macro_name_ident_stringified} {supports_only_stringified} enums where all variants are {syn_fields}::{named_camel_case} or all variants are {syn_fields}::{unnamed_camel_case}");
        data_enum.variants.iter().for_each(|variant|{
            match &variant.fields {
                syn::Fields::Named(_) => {
                    match &all_equal {
                        Some(supported_variant) => {
                            if let SuportedEnumVariant::Unnamed = supported_variant {
                                panic!("{error_message}");
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
                                panic!("{error_message}");
                            }
                        },
                        None => {
                            all_equal = Some(SuportedEnumVariant::Unnamed);
                        },
                    }
                },
                syn::Fields::Unit => panic!("{error_message}"),
            }
        });
        if let Some(supported_enum_variant) = all_equal {
            supported_enum_variant
        }
        else {
            panic!("{proc_macro_name_ident_stringified} {supports_only_stringified} with enums where all variants are named or unnamed");
        }
    };
    let trait_lifetime_token_stream = trait_lifetime_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {trait_lifetime_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let serialize_deserialize_camel_case = "SerializeDeserialize";
    let with_camel_case = "With";
    let with_serialize_deserialize_camel_case = format!("{with_camel_case}{serialize_deserialize_camel_case}");
    let ident_with_serialize_deserialize_stringified = format!("{ident}{with_serialize_deserialize_camel_case}");
    let ident_with_serialize_deserialize_token_stream = ident_with_serialize_deserialize_stringified
        .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {ident_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let config_camel_case = "Config";
    let config_generic_camel_case = format!("{config_camel_case}Generic");
    let config_generic_token_stream = config_generic_camel_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {config_generic_camel_case} {parse_proc_macro2_token_stream_failed_message}"));
    let source_camel_case = "Source";
    let string_camel_case = "String";
    let to_string_camel_case = format!("To{string_camel_case}");
    let to_string_with_config_camel_case = format!("{to_string_camel_case}{with_camel_case}{config_camel_case}");
    let source_to_string_with_config_camel_case = format!("{source_camel_case}{to_string_with_config_camel_case}");
    let unnamed_lower_case = unnamed_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let error_logs_logic_stringified = "error_logs_logic";
    let error_occurence_named_camel_case = format!("{proc_macro_name}{named_camel_case}");
    let error_occurence_named_lower_case = format!("{error_occurence_lower_case}_{named_lower_case}");
    let error_occurence_named_token_stream = error_occurence_named_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_named_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let error_occurence_unnamed_camel_case = format!("{proc_macro_name}{unnamed_camel_case}");
    let error_occurence_unnamed_lower_case = format!("{error_occurence_lower_case}_{unnamed_lower_case}");
    let error_occurence_unnamed_token_stream = error_occurence_unnamed_lower_case
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {error_occurence_unnamed_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_stringified = "crate::common";
    let crate_common_error_logs_logic_error_occurence_named_error_occurence_named_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{error_occurence_named_lower_case}::{error_occurence_named_camel_case}");
    let crate_common_error_logs_logic_error_occurence_named_error_occurence_named_token_stream = crate_common_error_logs_logic_error_occurence_named_error_occurence_named_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_error_occurence_named_error_occurence_named_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{error_occurence_unnamed_lower_case}::{error_occurence_unnamed_camel_case}");
    let crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_token_stream = crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_error_logs_logic_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::");
    let to_string_without_config_camel_case = format!("{to_string_camel_case}{with_camel_case}out{config_camel_case}");
    let to_string_without_config_lower_case = to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let crate_common_error_logs_logic_to_string_without_config_to_string_without_config_stringified = format!("{crate_common_error_logs_logic_stringified}{to_string_without_config_lower_case}::{to_string_without_config_camel_case}");
    let crate_common_error_logs_logic_to_string_without_config_to_string_without_config_token_stream = crate_common_error_logs_logic_to_string_without_config_to_string_without_config_stringified
    .parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_to_string_without_config_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_stringified = format!("{crate_common_error_logs_logic_to_string_without_config_to_string_without_config_stringified}{with_serialize_deserialize_camel_case}");
    let crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream = crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let config_fields_stringified = "config_fields";
    let crate_common_config_stringified = "crate::common::config";
    let crate_common_config_config_fields_stringified = format!("{crate_common_config_stringified}::{config_fields_stringified}::");
    let get_camel_case = "Get";
    let crate_common_config_config_fields_get_source_place_type_stringified = format!("{crate_common_config_config_fields_stringified}{get_camel_case}{source_camel_case}PlaceType");
    let crate_common_config_config_fields_get_source_place_type_token_stream = 
    crate_common_config_config_fields_get_source_place_type_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_config_config_fields_get_source_place_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_config_config_fields_get_timezone_stringified = format!("{crate_common_config_config_fields_stringified}{get_camel_case}Timezone");
    let crate_common_config_config_fields_get_timezone_token_stream = 
    crate_common_config_config_fields_get_timezone_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_config_config_fields_get_timezone_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let source_lower_case = source_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let to_string_with_config_lower_case = to_string_with_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let source_to_string_with_config_stringified = format!("{source_lower_case}_{to_string_with_config_lower_case}");
    let source_to_string_with_config_token_stream = 
    source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_without_config_token_stream = 
    to_string_without_config_lower_case.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {to_string_without_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let with_serialize_deserialize_lower_case = with_serialize_deserialize_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let to_string_without_config_with_serialize_deserialize_stringified = format!("{to_string_without_config_lower_case}_{with_serialize_deserialize_lower_case}");
    let to_string_without_config_with_serialize_deserialize_token_stream = 
    to_string_without_config_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {to_string_without_config_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let into_camel_case = "Into";
    let into_lower_case = into_camel_case.to_lowercase();
    let serialize_deserialize_lower_case = serialize_deserialize_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
    let into_serialize_deserialize_version_stringified = format!("{into_lower_case}_{serialize_deserialize_lower_case}_version");
    let into_serialize_deserialize_version_token_stream = into_serialize_deserialize_version_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {into_serialize_deserialize_version_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let supported_container_double_dot_double_dot = "SupportedContainer::";
    let supports_only_supported_container_stringified = format!("{supports_only_stringified} {supported_container_double_dot_double_dot}");
    let path_camel_case = "Path";
    let syn_type_path_stringified = format!("syn::Type::{path_camel_case}");
    let is_none_stringified = "is None";
    let suported_enum_variant_stringified = "SuportedEnumVariant";
    let syn_generic_argument_type_stringified = "syn::GenericArgument::Type";
    let compile_time_check_error_occurence_members_stringified = format!("compile_time_check_{error_occurence_lower_case}_members");
    let compile_time_check_error_occurence_members_token_stream = compile_time_check_error_occurence_members_stringified
    .parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {compile_time_check_error_occurence_members_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let crate_common_error_logs_logic_to_string_with_config_to_string_with_config_stringified = format!("{crate_common_error_logs_logic_stringified}{to_string_with_config_lower_case}::{to_string_with_config_camel_case}");
    let crate_common_error_logs_logic_to_string_with_config_to_string_with_config_token_stream = 
    crate_common_error_logs_logic_to_string_with_config_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_to_string_with_config_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
    let to_string_with_config_token_stream = 
    to_string_with_config_lower_case.parse::<proc_macro2::TokenStream>()
    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {to_string_with_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
    let key_camel_case = "Key";
    let value_camel_case = "Value";
    let key_lower_case = key_camel_case.to_lowercase();
    let value_lower_case = value_camel_case.to_lowercase();
    let reference_camel_case = "Reference";
    let hashmap_camel_case = "HashMap";
    let hashmap_lower_case = hashmap_camel_case.to_case(convert_case::Case::Flat);
    let vec_camel_case = "Vec";
    let vec_lower_case = vec_camel_case.to_lowercase(); 
    let token_stream = match supported_enum_variant {
        SuportedEnumVariant::Named => {
            let code_occurence_camel_case = format!("Code{occurence_camel_case}");
            let code_occurence_lower_case = code_occurence_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
            let foreign_type_camel_case = "ForeignType";
            let display_camel_case = "Display";
            let display_foreign_type_camel_case = format!("{display_camel_case}{foreign_type_camel_case}");
            let display_foreign_type_lower_case = display_foreign_type_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
            let display_lower_case = display_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
            let attribute_prefix_stringified = "eo_";
            let attribute_display_stringified = format!("{attribute_prefix_stringified}{display_lower_case}");
            let attribute_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{display_foreign_type_lower_case}");
            let attribute_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_error_occurence_stringified = format!("{attribute_prefix_stringified}{error_occurence_lower_case}");
            let attribute_vec_display_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_lower_case}");
            let attribute_vec_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_vec_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_foreign_type_lower_case}");
            let attribute_vec_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_vec_error_occurence_stringified = format!("{attribute_prefix_stringified}{vec_lower_case}_{error_occurence_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_with_serialize_deserialize_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}_{value_lower_case}_{error_occurence_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{display_foreign_type_lower_case}_{with_serialize_deserialize_lower_case}");
            let attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified = format!("{attribute_prefix_stringified}{hashmap_lower_case}_{key_lower_case}_{display_foreign_type_lower_case}_{value_lower_case}_{error_occurence_lower_case}");
            let variants_vec = data_enum.variants.into_iter().map(|variant| {
                let variant_fields_vec = if let syn::Fields::Named(fields_named) = variant.fields {
                    fields_named.named.into_iter().map(|field|{
                        let field_ident = field.ident.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} field.ident {is_none_stringified}"));
                        let error_or_code_occurence = match field_ident == *code_occurence_lower_case {
                            true => {
                                let (code_occurence_type_stringified, code_occurence_lifetime) = {
                                    if let syn::Type::Path(type_path) = &field.ty {
                                        (
                                            {
                                                let mut code_occurence_type_repeat_checker = false;
                                                let code_occurence_segments_stringified_handle = type_path.path.segments.iter()
                                                .fold(String::from(""), |mut acc, path_segment| {
                                                    let path_segment_ident = &path_segment.ident;
                                                    match *path_segment_ident == code_occurence_camel_case {
                                                        true => {
                                                            if code_occurence_type_repeat_checker {
                                                                panic!("{proc_macro_name_ident_stringified} code_occurence_ident detected more than one {code_occurence_camel_case} inside type path");
                                                            }
                                                            acc.push_str(&path_segment_ident.to_string());
                                                            code_occurence_type_repeat_checker = true;
                                                        },
                                                        false => acc.push_str(&format!("{path_segment_ident}::")),
                                                    }
                                                    acc
                                                });
                                                if !code_occurence_type_repeat_checker {
                                                    panic!("{proc_macro_name_ident_stringified} no {code_occurence_camel_case} named field");
                                                }
                                                code_occurence_segments_stringified_handle
                                            },
                                            form_last_arg_lifetime_vec(
                                                &type_path.path.segments,
                                                &proc_macro_name_ident_stringified,
                                                supports_only_stringified,
                                                is_none_stringified,
                                                syn_generic_argument_type_stringified
                                            ),
                                        )
                                      }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {code_occurence_lower_case} {supports_only_stringified} {syn_type_path_stringified}");
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
                                            let error_message = format!("{proc_macro_name_ident_stringified} two or more supported attributes!");
                                            if let true = attr.path.segments[0].ident == attribute_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_vec_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoVecErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_with_serialize_deserialize_value_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_foreign_type_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize);
                                                }
                                            }
                                            else if let true = attr.path.segments[0].ident == attribute_hashmap_key_display_foreign_type_value_error_occurence_stringified {
                                                if let true = option_attribute.is_some() {
                                                    panic!("{error_message}");
                                                }
                                                else {
                                                    option_attribute = Some(NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence);
                                                }
                                            }//other attributes are not for this proc_macro
                                        }//other attributes are not for this proc_macro
                                    });
                                    option_attribute.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} option attribute {is_none_stringified}"))
                                };
                                let syn_type_reference = format!("syn::Type::{reference_camel_case}");
                                let error_message = format!("{supports_only_stringified} {syn_type_path_stringified} and {syn_type_reference}");
                                let supported_container = match field.ty {
                                    syn::Type::Path(type_path) => {
                                        let path = generate_path_from_segments(&type_path.path.segments);
                                        let vec_lifetime = form_last_arg_lifetime_vec(
                                            &type_path.path.segments,
                                            &proc_macro_name_ident_stringified,
                                            supports_only_stringified,
                                            is_none_stringified,
                                            syn_generic_argument_type_stringified
                                        );
                                        let path_segment = type_path.path.segments.into_iter().last()
                                        .unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().last() {is_none_stringified}"));
                                        if path_segment.ident == vec_camel_case {
                                            let vec_element_type = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = path_segment.arguments {
                                                if let true = angle_brackets_generic_arguments.args.len() == 1 {
                                                    if let syn::GenericArgument::Type(type_handle) = 
                                                        angle_brackets_generic_arguments.args
                                                        .into_iter().next()
                                                        .unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.into_iter().nth(0) {is_none_stringified}"))
                                                    {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => VecElementType::Path{
                                                                element_path: generate_path_from_segments(&type_path.path.segments),
                                                                vec_lifetime: form_last_arg_lifetime_vec(
                                                                    &type_path.path.segments, 
                                                                    &proc_macro_name_ident_stringified,
                                                                    supports_only_stringified,
                                                                    is_none_stringified,
                                                                    syn_generic_argument_type_stringified
                                                                )
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments
                                                                        .into_iter().next()
                                                                        .unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {is_none_stringified}"))
                                                                        .ident
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {supports_only_stringified} {syn_type_path_stringified}");
                                                                };
                                                                VecElementType::Reference {
                                                                    reference_ident,
                                                                    lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {is_none_stringified}")).ident
                                                                }
                                                            },
                                                            _ => panic!("{proc_macro_name_ident_stringified} type_handle {supports_only_stringified} {syn_type_path_stringified} and {syn_type_reference}"),
                                                        }
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {supports_only_stringified} {syn_generic_argument_type_stringified}");
                                                    }
                                                }
                                                else {
                                                    panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 1");
                                                }
                                            }
                                            else {
                                                panic!("{proc_macro_name_ident_stringified} path_segment.arguments {supports_only_stringified} syn::PathArguments::AngleBracketed");
                                            };
                                            SupportedContainer::Vec{
                                                path,
                                                vec_element_type
                                            }
                                        }
                                        else if path_segment.ident == hashmap_camel_case {
                                            let (
                                                hashmap_key_type,
                                                hashmap_value_type
                                            ) = if let syn::PathArguments::AngleBracketed(angle_brackets_generic_arguments) = path_segment.arguments {
                                                if let true = angle_brackets_generic_arguments.args.len() == 2 {
                                                    let (
                                                        key_generic_argument,
                                                        value_generic_argument
                                                    ) = {
                                                        let mut key_generic_argument_option = None;
                                                        let mut value_generic_argument_option = None;
                                                        angle_brackets_generic_arguments.args
                                                        .into_iter()
                                                        .enumerate()
                                                        .for_each(|(index, generic_argument)|{
                                                            match index {
                                                                0 => {
                                                                    key_generic_argument_option = Some(generic_argument);
                                                                }
                                                                1 => {
                                                                    value_generic_argument_option = Some(generic_argument);
                                                                }
                                                                _ => panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() != 2")
                                                            }
                                                        });
                                                        (
                                                            key_generic_argument_option.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} key_generic_argument_option {is_none_stringified}")),
                                                            value_generic_argument_option.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} value_generic_argument_option {is_none_stringified}"))
                                                        )
                                                    };
                                                    let hashmap_key_type 
                                                    = if let syn::GenericArgument::Type(type_handle) = 
                                                        key_generic_argument
                                                    {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => {
                                                                HashMapKeyType::Path{
                                                                    key_segments_stringified: generate_path_from_segments(&type_path.path.segments),
                                                                    key_vec_lifetime: form_last_arg_lifetime_vec(
                                                                        &type_path.path.segments, 
                                                                        &proc_macro_name_ident_stringified,
                                                                        supports_only_stringified,
                                                                        is_none_stringified,
                                                                        syn_generic_argument_type_stringified
                                                                    )
                                                                }
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let key_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments
                                                                        .into_iter().next()
                                                                        .unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {is_none_stringified}"))
                                                                        .ident
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {supports_only_stringified} {syn_type_path_stringified}");
                                                                };
                                                                HashMapKeyType::Reference {
                                                                    key_reference_ident,
                                                                    key_lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {is_none_stringified}")).ident
                                                                }
                                                            },
                                                            _ => panic!("{proc_macro_name_ident_stringified} type_handle {supports_only_stringified} {syn_type_path_stringified} and {syn_type_reference}"),
                                                        }
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name_ident_stringified} key_generic_argument {supports_only_stringified} {syn_generic_argument_type_stringified}");
                                                    };
                                                    let hashmap_value_type = if let syn::GenericArgument::Type(type_handle) = value_generic_argument {
                                                        match type_handle {
                                                            syn::Type::Path(type_path) => {
                                                                HashMapValueType::Path{
                                                                    value_segments_stringified: generate_path_from_segments(&type_path.path.segments),
                                                                    value_vec_lifetime: form_last_arg_lifetime_vec(
                                                                        &type_path.path.segments,  
                                                                        &proc_macro_name_ident_stringified,
                                                                        supports_only_stringified,
                                                                        is_none_stringified,
                                                                        syn_generic_argument_type_stringified
                                                                    )
                                                                }
                                                            },
                                                            syn::Type::Reference(type_reference) => {
                                                                let value_reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                                                    if let true = type_path.path.segments.len() == 1 {
                                                                        type_path.path.segments
                                                                        .into_iter().next()
                                                                        .unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {is_none_stringified}"))
                                                                        .ident
                                                                    }
                                                                    else {
                                                                        panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                                                    }
                                                                }
                                                                else {
                                                                    panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {supports_only_stringified} {syn_type_path_stringified}");
                                                                };
                                                                HashMapValueType::Reference {
                                                                    value_reference_ident,
                                                                    value_lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {is_none_stringified}")).ident
                                                                }
                                                            },
                                                            _ => panic!("{proc_macro_name_ident_stringified} type_handle {supports_only_stringified} {syn_type_path_stringified} and syn::Type::Reference"),
                                                        }
                                                    }
                                                    else {
                                                        panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args[0] {supports_only_stringified} {syn_generic_argument_type_stringified}");
                                                    };
                                                    (
                                                        hashmap_key_type,
                                                        hashmap_value_type,
                                                    )
                                                }
                                                else {
                                                    panic!("{proc_macro_name_ident_stringified} angle_brackets_generic_arguments.args.len() == 2");
                                                }
                                            }
                                            else {
                                                panic!("{proc_macro_name_ident_stringified} path_segment.arguments {supports_only_stringified} syn::PathArguments::AngleBracketed");
                                            };
                                            SupportedContainer::HashMap{
                                                path,
                                                hashmap_key_type,
                                                hashmap_value_type
                                            }
                                        }
                                        else {
                                            SupportedContainer::Path{
                                                path, 
                                                vec_lifetime,
                                            }
                                        }
                                    },
                                    syn::Type::Reference(type_reference) => {
                                        let reference_ident = if let syn::Type::Path(type_path) = *type_reference.elem {
                                            if let true = type_path.path.segments.len() == 1 {
                                                type_path.path.segments
                                                .into_iter().next()
                                                .unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} type_path.path.segments.into_iter().nth(0) {is_none_stringified}"))
                                                .ident
                                            }
                                            else {
                                                panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_path.path.segments.len() != 1");
                                            }
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {syn_type_reference} type_reference.elem {supports_only_stringified} {syn_type_path_stringified}");
                                        };
                                        SupportedContainer::Reference{
                                            reference_ident,
                                            lifetime_ident: type_reference.lifetime.unwrap_or_else(|| panic!("{proc_macro_name_ident_stringified} {syn_type_reference} lifetime {is_none_stringified}")).ident,
                                        }
                                    },
                                    _ => panic!("{proc_macro_name_ident_stringified} {code_occurence_lower_case} {error_message}"),
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
                    panic!("{proc_macro_name_ident_stringified} expected fields would be named");
                };
                (
                    variant.ident, 
                    variant_fields_vec,
                )
            })
            .collect::<Vec<(
                proc_macro2::Ident, 
                 Vec<(
                    proc_macro2::Ident,
                    ErrorOrCodeOccurence
                )>
            )>>();
            let source_to_string_without_config_camel_case = format!("{source_camel_case}{to_string_without_config_camel_case}");
            let source_to_string_without_config_lower_case = source_to_string_without_config_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
            let source_to_string_without_config_token_stream = 
            source_to_string_without_config_lower_case.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {source_to_string_without_config_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
            let crate_common_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified = format!("{crate_common_error_logs_logic_stringified}{source_to_string_without_config_lower_case}::{source_to_string_without_config_camel_case}");
            let crate_common_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream = 
            crate_common_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_source_to_string_without_config_source_to_string_without_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            let crate_common_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified = format!("{crate_common_error_logs_logic_stringified}{source_to_string_with_config_stringified}::{source_to_string_with_config_camel_case}");
            let crate_common_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream = 
            crate_common_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_source_to_string_with_config_source_to_string_with_config_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(generics_len);
            let mut logic_for_source_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_source_to_string_without_config_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_get_code_occurence_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(variants_vec.len());
            let mut should_generate_impl_compile_time_check_error_occurence_members = false;
            variants_vec.into_iter().for_each(|(
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
                let mut format_logic_for_source_to_string_with_or_without_config: Vec<&str> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_with_config_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_without_config_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_into_serialize_deserialize_version_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                let mut fields_logic_for_compile_time_check_error_occurence_members_for_attribute: Vec<proc_macro2::TokenStream> = Vec::with_capacity(fields_vec.len());
                fields_vec.into_iter().enumerate().for_each(|(index, (field_ident, error_or_code_occurence))|{
                    let unused_argument_handle_stringified = format!("_unused_argument_{index}");
                    let unused_argument_handle_token_stream = unused_argument_handle_stringified
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {unused_argument_handle_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                    let to_string_lower_case = to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                    let hashmap_display_to_string_without_config_to_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{to_string_without_config_lower_case}_{to_string_lower_case}");
                    match error_or_code_occurence {
                        ErrorOrCodeOccurence::Error { 
                            attribute, 
                            supported_container,
                        } => {
                            let field_name_with_field_value_token_stream = {
                                let field_name_with_field_value_stringified = format!("\"{field_ident}: {{}}\"");
                                field_name_with_field_value_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {field_name_with_field_value_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                            };
                            let display_foreign_type_lower_case_token_stream = 
                            display_foreign_type_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {display_foreign_type_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let to_string_token_stream = to_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let lines_space_backslash_camel_case = "LinesSpaceBackslash";
                            let lines_space_backslash_lower_case = lines_space_backslash_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                            let lines_space_backslash_lower_case_token_stream = 
                            lines_space_backslash_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {lines_space_backslash_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_stringified = format!("{crate_common_error_logs_logic_stringified}{lines_space_backslash_lower_case}::{lines_space_backslash_camel_case}");
                            let crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream = 
                            crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_stringified = "crate::common"; 
                            let crate_common_display_foreign_type_display_foreign_type_stringified = format!("{crate_common_stringified}::{display_foreign_type_lower_case}::{display_foreign_type_camel_case}");
                            let crate_common_display_foreign_type_display_foreign_type_token_stream = 
                            crate_common_display_foreign_type_display_foreign_type_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_display_foreign_type_display_foreign_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let vec_display_to_string_camel_case = format!("{vec_camel_case}{display_camel_case}{to_string_camel_case}");
                            let vec_display_to_string_lower_case = vec_display_to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                            let vec_display_to_string_lower_case_token_stream = 
                            vec_display_to_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{vec_display_to_string_lower_case}::{vec_display_to_string_camel_case}");
                            let crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream = 
                            crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let hashmap_display_display_to_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{display_lower_case}_{to_string_lower_case}");
                            let hashmap_display_display_to_string_lower_case_token_stream = 
                            hashmap_display_display_to_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_display_to_string_lower_case}::{hashmap_camel_case}{display_camel_case}{display_camel_case}{to_string_camel_case}");
                            let crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream = 
                            crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let hashmap_display_display_foreign_type_to_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{display_foreign_type_lower_case}_{to_string_lower_case}");
                            let hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{to_string_without_config_lower_case}_{to_string_lower_case}_{with_serialize_deserialize_lower_case}");
                            let hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream = 
                            hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_to_string_without_config_to_string_lower_case}::{hashmap_camel_case}{display_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}{with_serialize_deserialize_camel_case}");
                            let crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream = 
                            crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let string_lower_case = string_camel_case.to_lowercase();
                            let hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{display_foreign_type_lower_case}_{into_lower_case}_{hashmap_lower_case}_{display_lower_case}_{string_lower_case}");
                            let std_stringified = "std";
                            let std_string_string_stringified = format!("{std_stringified}::{string_lower_case}::{string_camel_case}");
                            let std_string_string_token_stream = std_string_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {std_string_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let hashmap_display_display_foreign_type_to_string_lower_case_token_stream = 
                            hashmap_display_display_foreign_type_to_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_foreign_type_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_display_foreign_type_to_string_lower_case}::{hashmap_camel_case}{display_camel_case}{display_foreign_type_camel_case}{to_string_camel_case}");
                            let crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream = 
                            crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let vec_element_type_path_stringified = format!("VecElementType::{path_camel_case}");
                            let vec_display_foreign_type_to_string_camel_case = format!("{vec_camel_case}{display_foreign_type_camel_case}{to_string_camel_case}");
                            let vec_display_foreign_type_to_string_lower_case = vec_display_foreign_type_to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                            let vec_display_foreign_type_to_string_lower_case_token_stream = 
                            vec_display_foreign_type_to_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_foreign_type_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{vec_display_foreign_type_to_string_lower_case}::{vec_display_foreign_type_to_string_camel_case}");
                            let crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream = 
                            crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let hashmap_display_foreign_type_display_to_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_lower_case}_{to_string_lower_case}");
                            let hashmap_display_foreign_type_display_to_string_lower_case_token_stream = 
                            hashmap_display_foreign_type_display_to_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_foreign_type_display_to_string_lower_case}::{hashmap_camel_case}{display_foreign_type_camel_case}{display_camel_case}{to_string_camel_case}");
                            let crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream = 
                            crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let hashmap_display_foreign_type_display_foreign_type_to_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_foreign_type_lower_case}_{to_string_lower_case}");
                            let hashmap_display_foreign_type_display_foreign_type_to_string_lower_case_token_stream = 
                            hashmap_display_foreign_type_display_foreign_type_to_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_foreign_type_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_foreign_type_display_foreign_type_to_string_lower_case}::{hashmap_camel_case}{display_foreign_type_camel_case}{display_foreign_type_camel_case}{to_string_camel_case}");
                            let crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream = 
                            crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                            let as_std_collections_hashmap_key_type_stringified = format!("as {std_stringified}::collections::{hashmap_camel_case} key type");
                            let str_stringified = "str";
                            let string_string_stringified: String = format!("{string_lower_case}::{string_camel_case}");
                            let std_string_string_stringified = format!("{std_stringified}::{string_string_stringified}");
                            let must_be_used_with_stringified = "must be used with";
                            let does_not_support_stringified = "does not support";
                            let type_camel_case = "Type";
                            let hashmap_key_type_stringified = format!("{hashmap_camel_case}{key_camel_case}{type_camel_case}");
                            let hashmap_value_type_stringified = format!("{hashmap_camel_case}{value_camel_case}{type_camel_case}");
                            let hashmap_key_type_path_stringified = format!("{hashmap_key_type_stringified}::{path_camel_case}");
                            let hashmap_key_type_reference_stringified = format!("{hashmap_key_type_stringified}::{reference_camel_case}");
                            let hashmap_value_type_path_stringified = format!("{hashmap_value_type_stringified}::{path_camel_case}");
                            let hashmap_value_type_reference_stringified = format!("{hashmap_value_type_stringified}::{reference_camel_case}");
                            let inform_use_str_string_in_different_attribute = |
                                path: String,
                                wrong_attribute: &String,
                                attribute_to_use: &String
                            | {
                                let wrong_attribute_view = attribute_view(wrong_attribute);
                                let attribute_to_use_view = attribute_view(attribute_to_use);
                                //maybe additional cases exists
                                if path == str_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {str_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == std_string_string_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {std_string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == string_string_stringified {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {string_string_stringified} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                                else if path == string_camel_case {
                                    panic!("{proc_macro_name_ident_stringified} {wrong_attribute_view} {string_camel_case} {must_be_used_with_stringified} {attribute_to_use_view}");
                                }
                            };
                            let vec_display_into_vec_string_camel_case: String = format!("{vec_camel_case}{display_camel_case}{into_camel_case}{vec_camel_case}{string_camel_case}");
                            let vec_display_into_vec_string_lower_case = vec_display_into_vec_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                            let vec_display_into_vec_string_token_stream = vec_display_into_vec_string_lower_case
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_into_vec_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                            let (
                                logic_for_source_to_string_with_config_for_attribute, 
                                logic_for_source_to_string_without_config_for_attribute,
                                logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute,
                                logic_for_into_serialize_deserialize_version_for_attribute,
                                field_type_with_serialize_deserialize_token_stream,
                                _serde_borrow_attribute_token_stream,
                                enum_fields_logic_for_compile_time_check_error_occurence_members_used_unused,
                                logic_for_compile_time_check_error_occurence_members_for_attribute
                            ) = match attribute {
                                NamedAttribute::EoDisplay => {
                                    if let SupportedContainer::Path { path, vec_lifetime: _vec_lifetime } = supported_container {
                                        inform_use_str_string_in_different_attribute(
                                            path,
                                            &attribute.to_str().to_string(),
                                            &attribute_display_with_serialize_deserialize_stringified
                                        );
                                        (
                                            quote::quote! {
                                                {
                                                    use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                    format!(
                                                        #field_name_with_field_value_token_stream,
                                                        #field_ident
                                                    )
                                                    .#lines_space_backslash_lower_case_token_stream()
                                                }
                                            },
                                            quote::quote! {
                                                {
                                                    use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                    format!(
                                                        #field_name_with_field_value_token_stream,
                                                        #field_ident
                                                    )
                                                    .#lines_space_backslash_lower_case_token_stream()
                                                }
                                            },
                                            quote::quote! {
                                                { 
                                                    use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                    format!(
                                                        #field_name_with_field_value_token_stream,
                                                        #field_ident
                                                    )
                                                    .#lines_space_backslash_lower_case_token_stream()
                                                }
                                            },
                                            quote::quote! {
                                                {
                                                    #field_ident.#to_string_token_stream()
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
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {supported_container_double_dot_double_dot}{path_camel_case}", attribute.attribute_view())
                                    }
                                },
                                NamedAttribute::EoDisplayWithSerializeDeserialize => {
                                    match supported_container {
                                        SupportedContainer::Path { path, vec_lifetime } => {
                                            let (type_token_stream, serde_borrow_token_stream) = (
                                                {
                                                    let type_stringified = format!("{path}{}", vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            );
                                            (
                                                quote::quote! {
                                                    {
                                                        use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    {
                                                        use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    { 
                                                        use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
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
                                            panic_if_not_str(
                                                &reference_ident,
                                                str_stringified,
                                                &proc_macro_name_ident_stringified,
                                                supports_only_stringified,
                                                &attribute
                                            );
                                            possible_lifetime_addition(
                                                lifetime_ident.to_string(),
                                                &mut lifetimes_for_serialize_deserialize
                                            );
                                            (
                                                quote::quote! {
                                                    {
                                                        use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    {
                                                        use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    { 
                                                        use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                        format!(
                                                            #field_name_with_field_value_token_stream,
                                                            #field_ident
                                                        )
                                                        .#lines_space_backslash_lower_case_token_stream()
                                                    }
                                                },
                                                quote::quote! {
                                                    {
                                                        #field_ident.to_string()
                                                    }
                                                },
                                                quote::quote!{#std_string_string_token_stream},
                                                quote::quote!{#[serde(borrow)]},
                                                quote::quote! {
                                                    #field_ident: #unused_argument_handle_token_stream
                                                },
                                                proc_macro2::TokenStream::new(),
                                            )
                                        },
                                        _ => panic!("{proc_macro_name_ident_stringified} {} only supports {supported_container_double_dot_double_dot}{path_camel_case} and {supported_container_double_dot_double_dot}{reference_camel_case}", attribute.attribute_view()),
                                    }
                                },
                                NamedAttribute::EoDisplayForeignType => {
                                    if let SupportedContainer::Path { path: _path, vec_lifetime: _vec_lifetime } = supported_container {}
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{path_camel_case}", attribute.attribute_view());
                                    }
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_display_foreign_type_display_foreign_type_token_stream;
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#display_foreign_type_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_display_foreign_type_display_foreign_type_token_stream;
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#display_foreign_type_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            { 
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident
                                                )
                                                .#lines_space_backslash_lower_case_token_stream() 
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_display_foreign_type_display_foreign_type_token_stream;
                                                #field_ident.#display_foreign_type_lower_case_token_stream()
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
                                NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::Path { path, vec_lifetime } = supported_container {
                                        (
                                            {
                                                let type_stringified = format!("{path}{}", vec_lifetime_to_string(&vec_lifetime));
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                            }, 
                                            get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                vec_lifetime, 
                                                &mut lifetimes_for_serialize_deserialize,
                                                &trait_lifetime_stringified,
                                                &proc_macro_name_ident_stringified
                                            )
                                        )
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{path_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_display_foreign_type_display_foreign_type_token_stream;
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#display_foreign_type_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_display_foreign_type_display_foreign_type_token_stream;
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#display_foreign_type_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            { 
                                                use #crate_common_display_foreign_type_display_foreign_type_token_stream;
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#display_foreign_type_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream() 
                                            }
                                        },
                                        quote::quote! {
                                            #field_ident
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::Path { path, vec_lifetime } = supported_container {
                                        (
                                            {
                                                let type_stringified = format!("{path}{with_serialize_deserialize_camel_case}");
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                            }, 
                                            get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                vec_lifetime, 
                                                &mut lifetimes_for_serialize_deserialize,
                                                &trait_lifetime_stringified,
                                                &proc_macro_name_ident_stringified
                                            )
                                        )
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{path_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    {
                                                        use #crate_common_error_logs_logic_to_string_with_config_to_string_with_config_token_stream;
                                                        #field_ident.#to_string_with_config_token_stream(config)
                                                    }
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#to_string_without_config_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream;
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
                                                use #crate_common_error_logs_logic_error_occurence_named_error_occurence_named_token_stream;
                                                #field_ident.#error_occurence_named_token_stream();
                                            }
                                        },
                                    )
                                },
                                NamedAttribute::EoVecDisplay => {
                                    let type_token_stream = if let SupportedContainer::Vec { 
                                        path, 
                                        vec_element_type 
                                    } = supported_container {
                                        if let VecElementType::Path { element_path, vec_lifetime: _vec_lifetime } = vec_element_type {
                                            inform_use_str_string_in_different_attribute(
                                                element_path,
                                                &attribute.to_str().to_string(),
                                                &attribute_vec_display_with_serialize_deserialize_stringified
                                            );
                                            let type_stringified = format!("{path}<{std_string_string_stringified}>");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {vec_element_type_path_stringified}", attribute.attribute_view());
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{vec_camel_case}", attribute.attribute_view());
                                    };
                                    let crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{vec_display_into_vec_string_lower_case}::{vec_display_into_vec_string_camel_case}");
                                    let crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_token_stream = crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_token_stream;
                                                #field_ident.#vec_display_into_vec_string_token_stream()
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
                                NamedAttribute::EoVecDisplayWithSerializeDeserialize => {
                                    let (
                                        type_token_stream, 
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let SupportedContainer::Vec { 
                                        path, 
                                        vec_element_type 
                                    } = supported_container {
                                        match vec_element_type {
                                            VecElementType::Path { element_path, vec_lifetime } => (
                                                {
                                                    let type_stringified = format!("{path}<{element_path}{}>", vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                ),
                                                quote::quote! {
                                                    {
                                                        #field_ident
                                                    }
                                                }
                                            ),
                                            VecElementType::Reference { reference_ident, lifetime_ident } => {
                                                panic_if_not_str(
                                                    &reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                let crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_stringified = format!("{crate_common_error_logs_logic_stringified}{vec_display_into_vec_string_lower_case}::{vec_display_into_vec_string_camel_case}");
                                                let crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_token_stream = 
                                                crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                                (
                                                    {
                                                        let type_stringified = format!("{path}<{std_string_string_stringified}>");
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    },
                                                    {
                                                        possible_lifetime_addition(
                                                            lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize,
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        {
                                                            use #crate_common_error_logs_logic_vec_display_into_vec_string_vec_display_into_vec_string_token_stream;
                                                            #field_ident.#vec_display_into_vec_string_token_stream()
                                                        }
                                                    }
                                                )
                                            },
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{vec_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        into_serialize_deserialize_logic,
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
                                            panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {vec_element_type_path_stringified}", attribute.attribute_view());
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{vec_camel_case}", attribute.attribute_view());
                                    }
                                    let vec_display_foreign_type_into_vec_string_camel_case = format!("{vec_camel_case}{display_foreign_type_camel_case}{into_camel_case}{vec_camel_case}{string_camel_case}");
                                    let vec_display_foreign_type_into_vec_string_lower_case = vec_display_foreign_type_into_vec_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                                    let vec_display_foreign_type_into_vec_string_lower_case_token_stream = 
                                    vec_display_foreign_type_into_vec_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_display_foreign_type_into_vec_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    let crate_common_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified = format!("{crate_common_error_logs_logic_stringified}{vec_display_foreign_type_into_vec_string_lower_case}::{vec_display_foreign_type_into_vec_string_camel_case}");
                                    let crate_common_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_token_stream = 
                                    crate_common_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_to_string_vec_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_vec_display_foreign_type_into_vec_string_vec_display_foreign_type_into_vec_string_token_stream;
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
                                NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::Vec { 
                                        path, 
                                        vec_element_type 
                                    } = supported_container {
                                        if let VecElementType::Path { element_path, vec_lifetime } = vec_element_type {
                                            (
                                                {
                                                    let type_stringified = format!("{path}<{element_path}{}>", vec_lifetime_to_string(&vec_lifetime));
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            )
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {vec_element_type_path_stringified}", attribute.attribute_view());
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{vec_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_display_foreign_type_to_string_vec_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            #field_ident
                                        },
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoVecErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
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
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            )
                                        }
                                        else {
                                            panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {vec_element_type_path_stringified}", attribute.attribute_view());
                                        }                                        
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{vec_camel_case}", attribute.attribute_view());
                                    };
                                    let vec_to_string_without_config_to_string_camel_case = format!("{vec_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}");
                                    let vec_to_string_without_config_to_string_lower_case = vec_to_string_without_config_to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                                    let vec_to_string_without_config_to_string_lower_case_token_stream = 
                                    vec_to_string_without_config_to_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_to_string_without_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    let crate_common_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{vec_to_string_without_config_to_string_lower_case}::{vec_to_string_without_config_to_string_camel_case}");
                                    let crate_common_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_token_stream = 
                                    crate_common_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let vec_to_string_without_config_to_string_with_serialize_deserialize_camel_case = format!("{vec_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}{with_serialize_deserialize_camel_case}");
                                    let vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case = vec_to_string_without_config_to_string_with_serialize_deserialize_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                                    let vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream = 
                                    vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_to_string_without_config_to_string_with_serialize_deserialize_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    let crate_common_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_stringified = format!("{crate_common_error_logs_logic_stringified}{vec_to_string_without_config_to_string_lower_case}::{vec_to_string_without_config_to_string_with_serialize_deserialize_camel_case}");
                                    let crate_common_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_token_stream = 
                                    crate_common_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let vec_to_string_with_config_to_string_camel_case = format!("{vec_camel_case}{to_string_with_config_camel_case}{to_string_camel_case}");
                                    let vec_to_string_with_config_to_string_lower_case = vec_to_string_with_config_to_string_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
                                    let crate_common_error_logs_logic_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{vec_to_string_with_config_to_string_lower_case}::{vec_to_string_with_config_to_string_camel_case}");
                                    let crate_common_error_logs_logic_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_token_stream = crate_common_error_logs_logic_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let vec_to_string_with_config_to_string_token_stream = vec_to_string_with_config_to_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {vec_to_string_with_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_to_string_with_config_to_string_vec_to_string_with_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_to_string_with_config_to_string_token_stream(config)
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_to_string_without_config_to_string_vec_to_string_without_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#vec_to_string_without_config_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_vec_to_string_without_config_to_string_with_serialize_deserialize_vec_to_string_without_config_to_string_with_serialize_deserialize_token_stream;
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
                                                use #crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_token_stream;
                                                i.#error_occurence_unnamed_token_stream();
                                            });
                                        },
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => {
                                    let (
                                        type_token_stream, 
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        hashmap_value_type,
                                    } = supported_container {
                                        let hashmap_key_type_path_case = |
                                            key_segments_stringified: String,
                                            key_vec_lifetime: Vec<Lifetime>,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            panic_if_not_string(
                                                &key_segments_stringified,
                                                &std_string_string_stringified,
                                                &proc_macro_name_ident_stringified,
                                                supports_only_stringified,
                                                &as_std_collections_hashmap_key_type_stringified,
                                                &attribute
                                            );
                                            let hashmap_display_display_into_hashmap_display_string_camel_case = format!("{hashmap_camel_case}{display_camel_case}{display_camel_case}{into_camel_case}{hashmap_camel_case}{display_camel_case}{string_camel_case}");
                                            let hashmap_display_display_into_hashmap_display_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{display_lower_case}_{into_lower_case}_{hashmap_lower_case}_{display_lower_case}_{string_lower_case}");
                                            let crate_common_error_logs_logic_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{hashmap_display_display_into_hashmap_display_string_lower_case}::{hashmap_display_display_into_hashmap_display_string_camel_case}");
                                            let crate_common_error_logs_logic_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_token_stream = crate_common_error_logs_logic_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                            let hashmap_display_display_into_hashmap_display_string_token_stream = 
                                            hashmap_display_display_into_hashmap_display_string_lower_case
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_into_hashmap_display_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
                                                        vec_lifetime_to_string(&key_vec_lifetime),
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    key_vec_lifetime, 
                                                    lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                ),
                                                quote::quote! {
                                                    {
                                                        use #crate_common_error_logs_logic_hashmap_display_display_into_hashmap_display_string_hashmap_display_display_into_hashmap_display_string_token_stream;
                                                        #field_ident.#hashmap_display_display_into_hashmap_display_string_token_stream()
                                                    }
                                                }
                                            )
                                        };
                                        let hashmap_key_type_reference_case = |
                                            key_lifetime_ident: proc_macro2::Ident,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            (
                                                {
                                                    let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                {
                                                    possible_lifetime_addition(
                                                        key_lifetime_ident.to_string(),
                                                        lifetimes_for_serialize_deserialize
                                                    );
                                                    quote::quote!{#[serde(borrow)]}
                                                },
                                                quote::quote! {
                                                    {
                                                        use crate::common::error_logs_logic::hashmap_display_display_into_hashmap_string_string::HashMapDisplayDisplayIntoHashMapStringString;
                                                        #field_ident.hashmap_display_display_into_hashmap_string_string()
                                                    }
                                                }
                                            )
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified, 
                                                    key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_str().to_string(),
                                                    &attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_path_case(
                                                    key_segments_stringified,
                                                    key_vec_lifetime,
                                                    &mut lifetimes_for_serialize_deserialize
                                                )
                                            },
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_str().to_string(),
                                                    &attribute_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_reference_case(
                                                    key_lifetime_ident,
                                                    &mut lifetimes_for_serialize_deserialize
                                                )
                                            },
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => {
                                    let (
                                        type_token_stream, 
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic,
                                    ) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified, 
                                                    key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => {
                                                panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{}, {value_segments_stringified}{}>",
                                                            vec_lifetime_to_string(&key_vec_lifetime),
                                                            vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    }, 
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime, 
                                                        value_vec_lifetime, 
                                                        &mut lifetimes_for_serialize_deserialize,
                                                            &trait_lifetime_stringified,
                                                            &proc_macro_name_ident_stringified,
                                                    ),
                                                    quote::quote! {
                                                        {
                                                            #field_ident
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified, 
                                                    key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident, 
                                                    value_lifetime_ident 
                                                }
                                            ) => {
                                                panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{}, {std_string_string_stringified}>",
                                                            vec_lifetime_to_string(&key_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    }, 
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime, 
                                                        vec![Lifetime::Specified(value_lifetime_ident.to_string())], 
                                                        &mut lifetimes_for_serialize_deserialize,
                                                            &trait_lifetime_stringified,
                                                            &proc_macro_name_ident_stringified,
                                                    ),
                                                    quote::quote! {
                                                        {
                                                            use crate::common::error_logs_logic::hashmap_display_display_into_hashmap_display_string::HashMapDisplayDisplayIntoHashMapDisplayString;
                                                            #field_ident.hashmap_display_display_into_hashmap_display_string()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident, 
                                                    key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => {
                                                panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified}, {value_segments_stringified}{}>",
                                                            vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    },
                                                    {
                                                        possible_lifetime_addition(
                                                            key_lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        {
                                                            use crate::common::error_logs_logic::hashmap_display_display_into_hashmap_string_display::HashMapDisplayDisplayIntoHashMapStringDisplay;
                                                            #field_ident.hashmap_display_display_into_hashmap_string_display()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident, 
                                                    key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident, 
                                                    value_lifetime_ident 
                                                }
                                            ) => {
                                                panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!("{path}<{std_string_string_stringified}, {std_string_string_stringified}>");
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    },
                                                    {
                                                        get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                            vec![Lifetime::Specified(key_lifetime_ident.to_string())], 
                                                            vec![Lifetime::Specified(value_lifetime_ident.to_string())], 
                                                            &mut lifetimes_for_serialize_deserialize,
                                                                &trait_lifetime_stringified,
                                                                &proc_macro_name_ident_stringified,
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        {
                                                            use crate::common::error_logs_logic::hashmap_display_display_into_hashmap_string_string::HashMapDisplayDisplayIntoHashMapStringString;
                                                            #field_ident.hashmap_display_display_into_hashmap_string_string()
                                                        }
                                                    }
                                                )
                                            },
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType => {
                                    let (
                                        type_token_stream, 
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic,
                                    ) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        hashmap_value_type
                                    } = supported_container {
                                        let hashmap_key_type_path_case = |
                                            key_segments_stringified: String,
                                            key_vec_lifetime: Vec<Lifetime>,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            let hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case_token_stream = 
                                            hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                            let crate_common_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case}::{hashmap_camel_case}{display_camel_case}{display_foreign_type_camel_case}{into_camel_case}{hashmap_camel_case}{display_camel_case}{string_camel_case}");
                                            let crate_common_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_token_stream = 
                                            crate_common_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{key_segments_stringified}{},{std_string_string_stringified}>",
                                                        vec_lifetime_to_string(&key_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    key_vec_lifetime, 
                                                    lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                ),
                                                quote::quote! {
                                                    {
                                                        use #crate_common_error_logs_logic_hashmap_display_display_foreign_type_into_hashmap_display_string_hashmap_display_display_foreign_type_into_hashmap_display_string_token_stream;
                                                        #field_ident.#hashmap_display_display_foreign_type_into_hashmap_display_string_lower_case_token_stream()
                                                    }
                                                }
                                            )
                                        };
                                        let hashmap_key_type_reference_case = |
                                            _key_reference_ident: proc_macro2::Ident,
                                            key_lifetime_ident: proc_macro2::Ident,
                                            lifetimes_for_serialize_deserialize: &mut Vec<String>
                                        | -> (
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream,
                                            proc_macro2::TokenStream
                                        ) {
                                            (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                {
                                                    possible_lifetime_addition(
                                                        key_lifetime_ident.to_string(),
                                                        lifetimes_for_serialize_deserialize
                                                    );
                                                    quote::quote!{#[serde(borrow)]}
                                                },
                                                quote::quote! {
                                                    {
                                                        use crate::common::error_logs_logs::hashmap_display_display_foreign_type_into_hashmap_string_string::HashMapDisplayDisplayForeignTypeIntoHashMapStringString;
                                                        #field_ident.hashmap_display_display_foreign_type_into_hashmap_string_string()
                                                    }
                                                }
                                            )
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified, 
                                                    key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => hashmap_key_type_path_case(
                                                key_segments_stringified,
                                                key_vec_lifetime,
                                                &mut lifetimes_for_serialize_deserialize
                                            ),
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident, 
                                                    key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => hashmap_key_type_reference_case(
                                                key_reference_ident,
                                                key_lifetime_ident,
                                                &mut lifetimes_for_serialize_deserialize
                                            ),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize => {
                                    let (
                                        type_token_stream, 
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified, 
                                                    key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => {
                                                panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{},{value_segments_stringified}{}>",
                                                            vec_lifetime_to_string(&key_vec_lifetime),
                                                            vec_lifetime_to_string(&value_vec_lifetime),
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    },
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime, 
                                                        value_vec_lifetime, 
                                                        &mut lifetimes_for_serialize_deserialize,
                                                            &trait_lifetime_stringified,
                                                            &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        #field_ident
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident, 
                                                    key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => {
                                                panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                            vec_lifetime_to_string(&value_vec_lifetime),
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    },
                                                    {
                                                        possible_lifetime_addition(
                                                            key_lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        {
                                                            use crate::common::error_logs_logic::hashmap_display_display_foreign_type_into_hashmap_string_string::HashMapDisplayDisplayForeignTypeIntoHashMapStringString;
                                                            #field_ident.hashmap_display_display_foreign_type_into_hashmap_string_string()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident: #unused_argument_handle_token_stream
                                        },
                                        proc_macro2::TokenStream::new(),
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    let (
                                        type_token_stream, 
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let SupportedContainer::HashMap { 
                                        path,
                                        hashmap_key_type, 
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified, 
                                                    key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => {
                                                panic_if_not_string(
                                                    &key_segments_stringified,
                                                    &std_string_string_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &as_std_collections_hashmap_key_type_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{key_segments_stringified}{}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                            vec_lifetime_to_string(&key_vec_lifetime),
                                                            vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    }, 
                                                    get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
                                                        key_vec_lifetime, 
                                                        value_vec_lifetime, 
                                                        &mut lifetimes_for_serialize_deserialize,
                                                        &trait_lifetime_stringified,
                                                        &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        {
                                                            #field_ident.into_iter()
                                                            .map(|(k, v)| (k, { v.#into_serialize_deserialize_version_token_stream() }))
                                                            .collect()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident, 
                                                    key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => {
                                                panic_if_not_str(
                                                    &key_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                            vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    },
                                                    {
                                                        possible_lifetime_addition(
                                                            key_lifetime_ident.to_string(),
                                                            &mut lifetimes_for_serialize_deserialize
                                                        );
                                                        quote::quote!{#[serde(borrow)]}
                                                    },
                                                    quote::quote! {
                                                        {
                                                            #field_ident.into_iter()
                                                            .map(|(k, v)| (k.to_string(), { v.#into_serialize_deserialize_version_token_stream() }))
                                                            .collect()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    let hashmap_display_to_string_without_config_to_string_lower_case_token_stream = 
                                    hashmap_display_to_string_without_config_to_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_to_string_without_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    let crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_to_string_without_config_to_string_lower_case}::{hashmap_camel_case}{display_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}");
                                    let crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_token_stream = 
                                    crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let hashmap_display_to_string_with_config_to_string_camel_case = format!("{hashmap_camel_case}{display_camel_case}{to_string_with_config_camel_case}{to_string_camel_case}");
                                    let hashmap_display_to_string_with_config_to_string_lower_case = format!("{hashmap_lower_case}_{display_lower_case}_{to_string_with_config_lower_case}_{to_string_lower_case}");
                                    let crate_common_error_logs_logic_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{hashmap_display_to_string_with_config_to_string_lower_case}::{hashmap_display_to_string_with_config_to_string_camel_case}");
                                    let crate_common_error_logs_logic_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_token_stream = crate_common_error_logs_logic_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let hashmap_display_to_string_with_config_to_string_token_stream = hashmap_display_to_string_with_config_to_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_to_string_with_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_to_string_with_config_to_string_hashmap_display_to_string_with_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_to_string_with_config_to_string_token_stream(config)
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_hashmap_display_to_string_without_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_to_string_without_config_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        into_serialize_deserialize_logic,
                                        type_token_stream,
                                        serde_borrow_token_stream,
                                        quote::quote! {
                                            #field_ident
                                        },
                                        quote::quote! {
                                            #field_ident.values().for_each(|v|{
                                                use #crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_token_stream;
                                                v.#error_occurence_unnamed_token_stream();
                                            });
                                        },
                                    )
                                },
                                NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay => {
                                    let type_token_stream = if let SupportedContainer::HashMap { 
                                        path, 
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        let hashmap_key_type_path_case = || -> proc_macro2::TokenStream {
                                            let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
                                            type_stringified
                                            .parse::<proc_macro2::TokenStream>()
                                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                        };
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => {
                                                inform_use_str_string_in_different_attribute(
                                                    value_segments_stringified,
                                                    &attribute.to_str().to_string(),
                                                    &attribute_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_stringified
                                                );
                                                hashmap_key_type_path_case()
                                            },
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime
                                                 }, 
                                                 HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident  
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    let hashmap_display_foreign_type_display_into_hashmap_string_string_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{display_camel_case}{into_camel_case}{hashmap_camel_case}{string_camel_case}{string_camel_case}");
                                    let hashmap_display_foreign_type_display_into_hashmap_string_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_lower_case}_{into_lower_case}_{hashmap_lower_case}_{string_lower_case}_{string_lower_case}");
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{hashmap_display_foreign_type_display_into_hashmap_string_string_lower_case}::{hashmap_display_foreign_type_display_into_hashmap_string_string_camel_case}");
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream = crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream = hashmap_display_foreign_type_display_into_hashmap_string_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_into_hashmap_string_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_string_hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream;
                                                #field_ident.#hashmap_display_foreign_type_display_into_hashmap_string_string_token_stream()
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
                                NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize => {
                                    let (
                                        type_token_stream, 
                                        serde_borrow_token_stream,
                                        into_serialize_deserialize_logic
                                    ) = if let SupportedContainer::HashMap { 
                                        path, 
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => {
                                                let hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_lower_case}_{into_lower_case}_{hashmap_lower_case}_{string_lower_case}_{display_lower_case}");
                                                let hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case_token_stream = 
                                                hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                                let crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case}::{hashmap_camel_case}{display_foreign_type_camel_case}{display_camel_case}{into_camel_case}{hashmap_camel_case}{string_camel_case}{display_camel_case}");
                                                let crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_token_stream = 
                                                crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                            vec_lifetime_to_string(&value_vec_lifetime)
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    }, 
                                                    get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                        value_vec_lifetime, 
                                                        &mut lifetimes_for_serialize_deserialize,
                                                        &trait_lifetime_stringified,
                                                        &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        {   
                                                            use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_into_hashmap_string_display_hashmap_display_foreign_type_display_into_hashmap_string_display_token_stream;
                                                            #field_ident.#hashmap_display_foreign_type_display_into_hashmap_string_display_lower_case_token_stream()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident, 
                                                    value_lifetime_ident 
                                                }
                                            ) => {
                                                panic_if_not_str(
                                                    &value_reference_ident,
                                                    str_stringified,
                                                    &proc_macro_name_ident_stringified,
                                                    supports_only_stringified,
                                                    &attribute
                                                );
                                                (
                                                    {
                                                        let type_stringified = format!(
                                                            "{path}<{std_string_string_stringified},{std_string_string_stringified}>"
                                                        );
                                                        type_stringified
                                                        .parse::<proc_macro2::TokenStream>()
                                                        .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                    }, 
                                                    get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                        vec![Lifetime::Specified(value_lifetime_ident.to_string())], 
                                                        &mut lifetimes_for_serialize_deserialize,
                                                        &trait_lifetime_stringified,
                                                        &proc_macro_name_ident_stringified
                                                    ),
                                                    quote::quote! {
                                                        {
                                                            use crate::common::error_logs_logic::hashmap_display_foreign_type_display_into_hashmap_string_string::HashMapDisplayForeignTypeDisplayForeignTypeIntoHashMapStringString;
                                                            #field_ident.hashmap_display_foreign_type_display_into_hashmap_string_string()
                                                        }
                                                    }
                                                )
                                            },
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                },
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_to_string_hashmap_display_foreign_type_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        into_serialize_deserialize_logic,
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
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => {
                                                let type_stringified = format!("{path}<{std_string_string_stringified},{std_string_string_stringified}>");
                                                type_stringified
                                                .parse::<proc_macro2::TokenStream>()
                                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                            },
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime:  _key_vec_lifetime
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_foreign_type_lower_case}_{into_lower_case}_{hashmap_lower_case}_{string_lower_case}_{string_lower_case}");
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case_token_stream = 
                                    hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_lower_case}::{hashmap_camel_case}{display_foreign_type_camel_case}{display_foreign_type_camel_case}{into_camel_case}{hashmap_camel_case}{string_camel_case}{string_camel_case}");
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_token_stream = 
                                    crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_to_string_hashmap_display_display_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_string_token_stream;
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
                                NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize => {
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::HashMap { 
                                        path, 
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => (
                                                {
                                                   let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified},{value_segments_stringified}{}>",
                                                        vec_lifetime_to_string(&value_vec_lifetime),
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                },
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    value_vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            ),
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident,
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{display_foreign_type_camel_case}{into_camel_case}{hashmap_camel_case}{string_camel_case}{display_foreign_type_camel_case}");
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{display_foreign_type_lower_case}_{into_lower_case}_{hashmap_lower_case}_{string_lower_case}_{display_foreign_type_lower_case}");
                                    let hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_token_stream = hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_lower_case}::{hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_camel_case}");
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_token_stream = crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_to_string_hashmap_display_foreign_type_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_display_foreign_type_to_string_hashmap_display_display_foreign_type_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_display_foreign_type_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_hash_map_display_foreign_type_display_foreign_type_into_hash_map_string_display_foreign_type_token_stream;
                                                #field_ident.#hashmap_display_foreign_type_display_foreign_type_into_hashmap_string_display_foreign_type_token_stream()
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
                                NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => {
                                    if let false = should_generate_impl_compile_time_check_error_occurence_members {
                                        should_generate_impl_compile_time_check_error_occurence_members = true;
                                    }
                                    let (type_token_stream, serde_borrow_token_stream) = if let SupportedContainer::HashMap { 
                                        path, 
                                        hashmap_key_type,
                                        hashmap_value_type
                                    } = supported_container {
                                        match (hashmap_key_type, hashmap_value_type) {
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified, 
                                                    value_vec_lifetime 
                                                }
                                            ) => (
                                                {
                                                    let type_stringified = format!(
                                                        "{path}<{std_string_string_stringified}, {value_segments_stringified}{with_serialize_deserialize_camel_case}{}>",
                                                        vec_lifetime_to_string(&value_vec_lifetime)
                                                    );
                                                    type_stringified
                                                    .parse::<proc_macro2::TokenStream>()
                                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                                                }, 
                                                get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                                                    value_vec_lifetime, 
                                                    &mut lifetimes_for_serialize_deserialize,
                                                    &trait_lifetime_stringified,
                                                    &proc_macro_name_ident_stringified
                                                )
                                            ),
                                            (
                                                HashMapKeyType::Path { 
                                                    key_segments_stringified: _key_segments_stringified, 
                                                    key_vec_lifetime: _key_vec_lifetime 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_path_stringified} && {hashmap_value_type_reference_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Path { 
                                                    value_segments_stringified: _value_segments_stringified, 
                                                    value_vec_lifetime: _value_vec_lifetime 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_value_type_path_stringified}", attribute.attribute_view()),
                                            (
                                                HashMapKeyType::Reference { 
                                                    key_reference_ident: _key_reference_ident, 
                                                    key_lifetime_ident: _key_lifetime_ident 
                                                }, 
                                                HashMapValueType::Reference { 
                                                    value_reference_ident: _value_reference_ident, 
                                                    value_lifetime_ident: _value_lifetime_ident 
                                                }
                                            ) => panic!("{proc_macro_name_ident_stringified} {} {does_not_support_stringified} {hashmap_key_type_reference_stringified} && {hashmap_key_type_reference_stringified}", attribute.attribute_view()),
                                        }
                                    }
                                    else {
                                        panic!("{proc_macro_name_ident_stringified} {} {supports_only_supported_container_stringified}{hashmap_camel_case}", attribute.attribute_view());
                                    };
                                    let hashmap_display_foreign_type_to_string_without_config_to_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{to_string_without_config_lower_case}_{to_string_lower_case}");
                                    let hashmap_display_foreign_type_to_string_without_config_to_string_lower_case_token_stream = 
                                    hashmap_display_foreign_type_to_string_without_config_to_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_to_string_without_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified = format!("{crate_common_error_logs_logic_stringified}{hashmap_display_foreign_type_to_string_without_config_to_string_lower_case}::{hashmap_camel_case}{display_foreign_type_camel_case}{to_string_without_config_camel_case}{to_string_camel_case}");
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_token_stream = 
                                    crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let hashmap_display_foreign_type_to_string_with_config_to_string_camel_case = format!("{hashmap_camel_case}{display_foreign_type_camel_case}{to_string_with_config_camel_case}{to_string_camel_case}");
                                    let hashmap_display_foreign_type_to_string_with_config_to_string_lower_case = format!("{hashmap_lower_case}_{display_foreign_type_lower_case}_{to_string_with_config_lower_case}_{to_string_lower_case}");
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_stringified = format!("{crate_common_stringified}::{error_logs_logic_stringified}::{hashmap_display_foreign_type_to_string_with_config_to_string_lower_case}::{hashmap_display_foreign_type_to_string_with_config_to_string_camel_case}");
                                    let crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_token_stream = crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_stringified
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                                    let hashmap_display_foreign_type_to_string_with_config_to_string_token_stream = hashmap_display_foreign_type_to_string_with_config_to_string_lower_case
                                    .parse::<proc_macro2::TokenStream>()
                                    .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {hashmap_display_foreign_type_to_string_with_config_to_string_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
                                    (
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_with_config_to_string_hashmap_display_foreign_type_to_string_with_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_to_string_with_config_to_string_token_stream(config)
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_foreign_type_to_string_without_config_to_string_hashmap_display_foreign_type_to_string_without_config_to_string_token_stream;
                                                format!(
                                                    #field_name_with_field_value_token_stream,
                                                    #field_ident.#hashmap_display_foreign_type_to_string_without_config_to_string_lower_case_token_stream()
                                                )
                                                .#lines_space_backslash_lower_case_token_stream()
                                            }
                                        },
                                        quote::quote! {
                                            {
                                                use #crate_common_error_logs_logic_lines_space_backslash_lines_space_backslash_token_stream;
                                                use #crate_common_error_logs_logic_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_hashmap_display_to_string_without_config_to_string_with_serialize_deserialize_token_stream;
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
                                                            use #crate_common_display_foreign_type_display_foreign_type_token_stream;
                                                            k.#display_foreign_type_lower_case_token_stream()
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
                                                use #crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_token_stream;
                                                v.#error_occurence_unnamed_token_stream();
                                            });
                                        },
                                    )
                                },
                            };
                            enum_fields_logic_for_source_to_string_with_config.push(quote::quote! {
                                #field_ident
                            });
                            enum_fields_logic_for_source_to_string_without_config.push(quote::quote! {
                                #field_ident
                            });
                            enum_fields_logic_for_get_code_occurence.push(quote::quote!{
                                #field_ident: #unused_argument_handle_token_stream
                            });
                            enum_fields_logic_for_enum_with_serialize_deserialize.push(quote::quote!{
                                // #serde_borrow_attribute_token_stream
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
                            format_logic_for_source_to_string_with_or_without_config.push("{}");
                            fields_logic_for_source_to_string_with_config_for_attribute.push(logic_for_source_to_string_with_config_for_attribute);
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
                            vec_lifetime: _vec_lifetime,
                         } => {
                            let code_occurence_type_with_serialize_deserialize_token_stream = {
                                let code_occurence_type_with_serialize_deserialize_stringified = 
                                format!("{field_type}{with_serialize_deserialize_camel_case}");
                                code_occurence_type_with_serialize_deserialize_stringified
                                .parse::<proc_macro2::TokenStream>()
                                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {code_occurence_type_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"))
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
                let format_logic_for_source_to_string_with_or_without_config_stringified = format_logic_for_source_to_string_with_or_without_config.iter()
                .fold(String::from(""), |mut acc, path_segment| {
                    acc.push_str(path_segment);
                    acc
                });
                let start_scope_stringified = "{{";
                let end_scope_stringified = "}}";
                let format_logic_for_source_to_string_with_or_without_config_handle_stringified = format!("\"{start_scope_stringified}\n{format_logic_for_source_to_string_with_or_without_config_stringified}{end_scope_stringified}\"");
                let format_logic_for_source_to_string_with_or_without_config_handle_token_stream = format_logic_for_source_to_string_with_or_without_config_handle_stringified
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {format_logic_for_source_to_string_with_or_without_config_handle_stringified} {parse_proc_macro2_token_stream_failed_message}"));
                let fields_logic_for_source_to_string_with_config_for_attribute_iter = fields_logic_for_source_to_string_with_config_for_attribute.iter();
                let fields_logic_for_source_to_string_without_config_for_attribute_iter = fields_logic_for_source_to_string_without_config_for_attribute.iter();
                let fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute_iter = fields_logic_for_source_to_string_without_config_with_serialize_deserialize_for_attribute.iter();
                let fields_logic_for_into_serialize_deserialize_version_for_attribute_iter = fields_logic_for_into_serialize_deserialize_version_for_attribute.iter();
                let fields_logic_for_compile_time_check_error_occurence_members_iter = fields_logic_for_compile_time_check_error_occurence_members_for_attribute.iter();
                logic_for_source_to_string_with_config.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_with_config_iter),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_with_or_without_config_handle_token_stream
                            ,
                            #(#fields_logic_for_source_to_string_with_config_for_attribute_iter),*
                        )
                    }
                });
                logic_for_source_to_string_without_config.push(quote::quote! {
                    #ident::#variant_ident {
                        #(#enum_fields_logic_for_source_to_string_without_config_iter),*
                    } => {
                        format!(
                            #format_logic_for_source_to_string_with_or_without_config_handle_token_stream
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
                            #format_logic_for_source_to_string_with_or_without_config_handle_token_stream
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
            let get_code_occurence_camel_case = format!("{get_camel_case}{code_occurence_camel_case}");
            let get_code_occurence_lower_case = get_code_occurence_camel_case.to_case(convert_case::Case::Snake).to_lowercase();
            let crate_common_error_logs_logic_get_code_occurence_get_code_occurence_stringified = format!("{crate_common_error_logs_logic_stringified}{get_code_occurence_lower_case}::{get_code_occurence_camel_case}");
            let crate_common_error_logs_logic_get_code_occurence_get_code_occurence_token_stream = 
            crate_common_error_logs_logic_get_code_occurence_get_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_get_code_occurence_get_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            let crate_common_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_stringified = format!("{crate_common_error_logs_logic_get_code_occurence_get_code_occurence_stringified}{with_serialize_deserialize_camel_case}");
            let crate_common_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_token_stream = 
            crate_common_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            let crate_common_code_occurence_code_occurence_stringified = format!("crate::common::{code_occurence_lower_case}::{code_occurence_camel_case}");
            let crate_common_code_occurence_code_occurence_token_stream = 
            crate_common_code_occurence_code_occurence_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_code_occurence_code_occurence_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            let crate_common_code_occurence_code_occurence_with_serialize_deserialize_stringified = format!("{crate_common_code_occurence_code_occurence_stringified}{with_serialize_deserialize_camel_case}");
            let crate_common_code_occurence_code_occurence_with_serialize_deserialize_token_stream = 
            crate_common_code_occurence_code_occurence_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {crate_common_code_occurence_code_occurence_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            let get_code_occurence_token_stream = 
            get_code_occurence_lower_case.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {get_code_occurence_lower_case} {parse_proc_macro2_token_stream_failed_message}"));
            let get_code_occurence_with_serialize_deserialize_stringified = format!("{get_code_occurence_lower_case}_{with_serialize_deserialize_lower_case}");
            let get_code_occurence_with_serialize_deserialize_token_stream = 
            get_code_occurence_with_serialize_deserialize_stringified.parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {get_code_occurence_with_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"));
            let compile_time_check_error_occurence_members_impl_token_stream = match should_generate_impl_compile_time_check_error_occurence_members {
                true => quote::quote!{
                    impl<#generics> #ident<#generics> {
                        fn #compile_time_check_error_occurence_members_token_stream(&self) {
                            match self {
                                #(#logic_for_compile_time_check_error_occurence_members_iter),*
                            }
                        }
                    }
                },
                false => proc_macro2::TokenStream::new(),
            };
            quote::quote! {
                impl<
                    #trait_lifetime_token_stream,
                    #generics,
                    #config_generic_token_stream
                >
                    #crate_common_error_logs_logic_source_to_string_with_config_source_to_string_with_config_token_stream<
                        #trait_lifetime_token_stream,
                        #config_generic_token_stream
                    > for #ident<#generics>
                    where #config_generic_token_stream: #crate_common_config_config_fields_get_source_place_type_token_stream
                        + #crate_common_config_config_fields_get_timezone_token_stream
                {
                    fn #source_to_string_with_config_token_stream(
                        &self,
                        config: &#config_generic_token_stream 
                    ) -> String {
                        match self {
                            #(#logic_for_source_to_string_with_config_iter),*
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream,
                    #generics
                >
                    #crate_common_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<
                        #trait_lifetime_token_stream
                    > for #ident<#generics>
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #(#logic_for_source_to_string_without_config_iter),*
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream,
                    #generics
                > 
                    #crate_common_error_logs_logic_get_code_occurence_get_code_occurence_token_stream<
                        #trait_lifetime_token_stream
                    >
                    for #ident<#generics>
                {
                    fn #get_code_occurence_token_stream(&self) -> &#crate_common_code_occurence_code_occurence_token_stream
                    {
                        match self {
                            #(#logic_for_get_code_occurence_iter),*
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
                pub enum #ident_with_serialize_deserialize_token_stream {
                    #(#logic_for_enum_with_serialize_deserialize_iter),*
                }
                impl<#trait_lifetime_token_stream> #crate_common_error_logs_logic_source_to_string_without_config_source_to_string_without_config_token_stream<
                    #trait_lifetime_token_stream
                > for #ident_with_serialize_deserialize_token_stream
                {
                    fn #source_to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #(#logic_for_source_to_string_without_config_with_serialize_deserialize_iter),*
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream> #crate_common_error_logs_logic_get_code_occurence_get_code_occurence_with_serialize_deserialize_token_stream<
                    #trait_lifetime_token_stream
                >
                    for #ident_with_serialize_deserialize_token_stream
                {
                    fn #get_code_occurence_with_serialize_deserialize_token_stream(
                        &self
                    ) -> &#crate_common_code_occurence_code_occurence_with_serialize_deserialize_token_stream
                    {
                        match self {
                            #(#logic_for_get_code_occurence_with_serialize_deserialize_iter),*
                        }
                    }
                }
                impl<#generics> #ident<#generics> {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_serialize_deserialize_token_stream {
                        match self {
                            #(#logic_for_into_serialize_deserialize_version_iter),*
                        }
                    }
                }
                impl<#generics> std::fmt::Display for #ident<#generics> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                        write!(f, "{}", self.#to_string_without_config_token_stream())
                    }
                }
                impl std::fmt::Display for #ident_with_serialize_deserialize_token_stream {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream;
                        write!(f, "{}", self.#to_string_without_config_with_serialize_deserialize_token_stream())
                    }
                }
                impl<#generics> #crate_common_error_logs_logic_error_occurence_named_error_occurence_named_token_stream for #ident<#generics> {
                    fn #error_occurence_named_token_stream(&self) {}
                }
                #compile_time_check_error_occurence_members_impl_token_stream
            }
        },
        SuportedEnumVariant::Unnamed => {
            let mut lifetimes_for_serialize_deserialize = Vec::with_capacity(generics_len);
            let data_enum_variants_len = data_enum.variants.len();
            let mut logic_for_to_string_with_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_to_string_without_config: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_enum_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_to_string_without_config_with_serialize_deserialize: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_into_serialize_deserialize_version: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            let mut logic_for_compile_time_check_error_occurence_members: Vec<proc_macro2::TokenStream> = Vec::with_capacity(data_enum_variants_len);
            data_enum.variants.iter().for_each(|variant|{
                let variant_ident = &variant.ident;
                let field_type = if let syn::Fields::Unnamed(fields_unnamed) = &variant.fields {
                    let unnamed = &fields_unnamed.unnamed;
                    if let false = unnamed.len() == 1 {
                        panic!("{proc_macro_name_ident_stringified} {suported_enum_variant_stringified}::{unnamed_camel_case} variant fields unnamed len != 1");
                    }
                    &unnamed[0].ty
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} {supports_only_stringified} {syn_fields}::{unnamed_camel_case}");
                };
                let (type_token_stream, serde_borrow_token_stream) = if let syn::Type::Path(type_path) = field_type {
                    let vec_lifetime = form_last_arg_lifetime_vec(
                        &type_path.path.segments, 
                        &proc_macro_name_ident_stringified,
                        supports_only_stringified,
                        is_none_stringified,
                        syn_generic_argument_type_stringified
                    );
                    (
                        {
                            let type_stringified = format!(
                                "{}{with_serialize_deserialize_camel_case}{}",
                                generate_path_from_segments(&type_path.path.segments),
                                vec_lifetime_to_string(&vec_lifetime)
                            );
                            type_stringified
                            .parse::<proc_macro2::TokenStream>()
                            .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {type_stringified} {parse_proc_macro2_token_stream_failed_message}"))
                        },
                        get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
                            vec_lifetime, 
                            &mut lifetimes_for_serialize_deserialize,
                            &trait_lifetime_stringified,
                            &proc_macro_name_ident_stringified,
                        )
                    )
                }
                else {
                    panic!("{proc_macro_name_ident_stringified} {supports_only_stringified} {syn_type_path_stringified}")
                };
                logic_for_to_string_with_config.push(quote::quote!{
                    #ident::#variant_ident(i) => {
                        i.#to_string_with_config_token_stream(config)
                    }
                });
                logic_for_to_string_without_config.push(quote::quote!{
                    #ident::#variant_ident(i) => {
                        i.#to_string_without_config_token_stream()
                    }
                });
                logic_for_enum_with_serialize_deserialize.push({
                    quote::quote!{
                        #serde_borrow_token_stream
                        #variant_ident(#type_token_stream)
                    }
                });
                logic_for_to_string_without_config_with_serialize_deserialize.push(quote::quote!{
                    #ident_with_serialize_deserialize_token_stream::#variant_ident(i) => {
                         i.#to_string_without_config_with_serialize_deserialize_token_stream()
                    }
                });
                logic_for_into_serialize_deserialize_version.push(quote::quote!{
                     #ident::#variant_ident(i) => {
                        #ident_with_serialize_deserialize_token_stream::#variant_ident(i.#into_serialize_deserialize_version_token_stream())
                     }
                });
                logic_for_compile_time_check_error_occurence_members.push(quote::quote!{
                     #ident::#variant_ident(i) => {
                        {
                            use #crate_common_error_logs_logic_error_occurence_named_error_occurence_named_token_stream;
                            i.#error_occurence_named_token_stream();
                        }
                     }
                });
            });
            let logic_for_to_string_with_config_generated = logic_for_to_string_with_config.iter();
            let logic_for_to_string_without_config_generated = logic_for_to_string_without_config.iter();
            let logic_for_enum_with_serialize_deserialize_generated = logic_for_enum_with_serialize_deserialize.iter();
            let logic_for_to_string_without_config_with_serialize_deserialize_generated = logic_for_to_string_without_config_with_serialize_deserialize.iter();
            let logic_for_into_serialize_deserialize_version_generated = logic_for_into_serialize_deserialize_version.iter();
            let logic_for_compile_time_check_error_occurence_members_generated = logic_for_compile_time_check_error_occurence_members.iter();
            quote::quote! {
                impl<
                    #trait_lifetime_token_stream,
                    #generics,
                    #config_generic_token_stream
                >
                    #crate_common_error_logs_logic_to_string_with_config_to_string_with_config_token_stream<
                        #trait_lifetime_token_stream,
                        #config_generic_token_stream
                    > for #ident<#generics>
                where
                    #config_generic_token_stream: #crate_common_config_config_fields_get_source_place_type_token_stream
                    + #crate_common_config_config_fields_get_timezone_token_stream
                {
                    fn #to_string_with_config_token_stream(&self, config: &#config_generic_token_stream) -> String {
                        match self {
                            #(#logic_for_to_string_with_config_generated),*
                        }
                    }
                }
                impl<
                    #trait_lifetime_token_stream,
                    #generics
                > #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_token_stream<
                    #trait_lifetime_token_stream
                >
                    for #ident<#generics>
                {
                    fn #to_string_without_config_token_stream(&self) -> String {
                        match self {
                            #(#logic_for_to_string_without_config_generated),*
                        }
                    }
                }
                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)] 
                pub enum #ident_with_serialize_deserialize_token_stream {
                    #(#logic_for_enum_with_serialize_deserialize_generated),*
                }
                impl<#trait_lifetime_token_stream>
                    #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream<
                        #trait_lifetime_token_stream
                    > 
                    for #ident_with_serialize_deserialize_token_stream
                {
                    fn #to_string_without_config_with_serialize_deserialize_token_stream(&self) -> String {
                        match self {
                            #(#logic_for_to_string_without_config_with_serialize_deserialize_generated),*
                        }
                    }
                }
                impl<#generics> #ident<#generics> {
                    pub fn #into_serialize_deserialize_version_token_stream(self) -> #ident_with_serialize_deserialize_token_stream {
                        match self {
                            #(#logic_for_into_serialize_deserialize_version_generated),*
                        }
                    }
                }
                impl<#generics> std::fmt::Display for #ident<#generics> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_token_stream;
                        write!(f, "{}", self.#to_string_without_config_token_stream())
                    }
                }
                impl std::fmt::Display for #ident_with_serialize_deserialize_token_stream {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        use #crate_common_error_logs_logic_to_string_without_config_to_string_without_config_with_serialize_deserialize_token_stream;
                        write!(f, "{}", self.#to_string_without_config_with_serialize_deserialize_token_stream())
                    }
                }
                impl<#generics> #crate_common_error_logs_logic_error_occurence_unnamed_error_occurence_unnamed_token_stream for #ident<#generics> {
                    fn #error_occurence_unnamed_token_stream(&self) -> () {
                        ()
                    }
                }
                impl<#generics> #ident<#generics> {
                    fn #compile_time_check_error_occurence_members_token_stream(&self) {
                        match self {
                            #(#logic_for_compile_time_check_error_occurence_members_generated),*
                        }
                    }
                }
            }
        },
    };
    // if ident_stringified == "" {
    //     println!("{token_stream}");
    // }
    token_stream.into()
}

#[allow(clippy::enum_variant_names)]
#[derive(
    Debug,
    strum_macros::EnumIter,
    strum_macros::Display,
    enum_extension::EnumExtension
)]
enum NamedAttribute {
    EoDisplay,
    EoDisplayWithSerializeDeserialize,
    EoDisplayForeignType,
    EoDisplayForeignTypeWithSerializeDeserialize,
    EoErrorOccurence,
    EoVecDisplay,
    EoVecDisplayWithSerializeDeserialize,
    EoVecDisplayForeignType,
    EoVecDisplayForeignTypeWithSerializeDeserialize,
    EoVecErrorOccurence,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType,
    EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize,
    EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence,
    EoHashMapKeyDisplayForeignTypeValueDisplay,
    EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize,
    EoHashMapKeyDisplayForeignTypeValueDisplayForeignType,
    EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize,
    EoHashMapKeyDisplayForeignTypeValueErrorOccurence,
}

impl NamedAttribute {
    fn to_str(&self) -> &str {
        match self {
            NamedAttribute::EoDisplay => "eo_display",
            NamedAttribute::EoDisplayWithSerializeDeserialize => "eo_display_with_serialize_deserialize",
            NamedAttribute::EoDisplayForeignType => "eo_display_foreign_type",
            NamedAttribute::EoDisplayForeignTypeWithSerializeDeserialize => "eo_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoErrorOccurence => "eo_error_occurence",
            NamedAttribute::EoVecDisplay => "eo_vec_display",
            NamedAttribute::EoVecDisplayWithSerializeDeserialize => "eo_vec_display_with_serialize_deserialize",
            NamedAttribute::EoVecDisplayForeignType => "eo_vec_display_foreign_type",
            NamedAttribute::EoVecDisplayForeignTypeWithSerializeDeserialize => "eo_vec_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoVecErrorOccurence => "eo_vec_error_occurence",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplay => "eo_hashmap_key_display_with_serialize_deserialize_value_display",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayWithSerializeDeserialize => "eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignType => "eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueDisplayForeignTypeWithSerializeDeserialize => "eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayWithSerializeDeserializeValueErrorOccurence => "eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplay => "eo_hashmap_key_display_foreign_type_value_display",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayWithSerializeDeserialize => "eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignType => "eo_hashmap_key_display_foreign_type_value_display_foreign_type",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueDisplayForeignTypeWithSerializeDeserialize => "eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize",
            NamedAttribute::EoHashMapKeyDisplayForeignTypeValueErrorOccurence => "eo_hashmap_key_display_foreign_type_value_error_occurence",
        }
    }
    fn attribute_view(&self) -> String {
        attribute_view(&self.to_str().to_string())
    }
}

enum SuportedEnumVariant {
    Named,
    Unnamed,
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

#[derive(Debug)]
enum SupportedContainer {
    Vec{
        path: String,
        vec_element_type: VecElementType
    },
    HashMap{
        path: String,
        hashmap_key_type: HashMapKeyType,
        hashmap_value_type: HashMapValueType
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

#[derive(Debug)]
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

#[derive(Debug)]
enum HashMapKeyType {
    Path{
        key_segments_stringified: String,
        key_vec_lifetime: Vec<Lifetime>
    },
    Reference {
        key_reference_ident: proc_macro2::Ident,
        key_lifetime_ident: proc_macro2::Ident
    }
}

#[derive(Debug)]
enum HashMapValueType {
    Path{
        value_segments_stringified: String,
        value_vec_lifetime: Vec<Lifetime>
    },
    Reference {
        value_reference_ident: proc_macro2::Ident,
        value_lifetime_ident: proc_macro2::Ident
    }
}

#[derive(Debug, Clone)]
enum Lifetime {
    Specified(String),
    NotSpecified,
}

fn attribute_view(attribute: &String) -> String {
    format!("attribute #[{attribute}]")
}

fn panic_if_not_str(
    reference_ident: &proc_macro2::Ident,
    str_stringified: &str,
    proc_macro_name_ident_stringified: &String,
    supports_only_stringified: &str,
    attribute: &NamedAttribute
) {
    if let false = reference_ident == str_stringified {
        panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {str_stringified}, but got {reference_ident}", attribute.attribute_view());
    }
}

fn panic_if_not_string(
    segments_stringified: &String,
    std_string_string_stringified: &String,
    proc_macro_name_ident_stringified: &String,
    supports_only_stringified: &str,
    as_std_collections_hashmap_key_type_stringified: &String,
    attribute: &NamedAttribute
) {
    if let false = segments_stringified == std_string_string_stringified {
        panic!("{proc_macro_name_ident_stringified} {} {supports_only_stringified} {std_string_string_stringified} {as_std_collections_hashmap_key_type_stringified} (hashmap key must be string for deserialization)", attribute.attribute_view());
    }
}


fn generate_path_from_segments(segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>) -> String {
    let mut segments_stringified = segments.iter()
    .fold(String::from(""), |mut acc, elem| {
        acc.push_str(&format!("{}::", elem.ident));
        acc
    });
    segments_stringified.pop();
    segments_stringified.pop();
    segments_stringified
}

fn possible_lifetime_addition(
    lifetime: String, 
    lifetimes_for_serialize_deserialize: &mut Vec<String>,
) {
    if let false = lifetimes_for_serialize_deserialize.contains(&lifetime) {
        lifetimes_for_serialize_deserialize.push(lifetime);
    };
}

fn get_possible_serde_borrow_token_stream_for_one_vec_with_possible_lifetime_addition(
    vec_lifetime: Vec<Lifetime>, 
    lifetimes_for_serialize_deserialize: &mut Vec<String>,
    trait_lifetime_stringified: &String,
    proc_macro_name_ident_stringified: &String,
) -> proc_macro2::TokenStream {
    let token_stream = match vec_lifetime_to_lifetime(&vec_lifetime) {
        Lifetime::Specified(_) => quote::quote!{#[serde(borrow)]},
        Lifetime::NotSpecified => proc_macro2::TokenStream::new(),
    };
    vec_lifetime.into_iter().for_each(|k|{
        if let Lifetime::Specified(specified_lifetime) = k {
            if let true = &specified_lifetime == trait_lifetime_stringified {
                panic!("{proc_macro_name_ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
            }
            possible_lifetime_addition(
                specified_lifetime,
                lifetimes_for_serialize_deserialize
            );
        }
    });
    token_stream
}
//potential support for few lifetime annotations, but now supported only one lifetime annotation
fn get_possible_serde_borrow_token_stream_for_two_vecs_with_possible_lifetime_addition(
    key_vec_lifetime: Vec<Lifetime>, 
    value_vec_lifetime: Vec<Lifetime>, 
    lifetimes_for_serialize_deserialize: &mut Vec<String>,
    trait_lifetime_stringified: &String,
    proc_macro_name_ident_stringified: &String,
) -> proc_macro2::TokenStream {
    let key_lifetime_enum = vec_lifetime_to_lifetime(&key_vec_lifetime);
    let value_lifetime_enum = vec_lifetime_to_lifetime(&value_vec_lifetime);
    let token_stream = match (key_lifetime_enum, value_lifetime_enum) {
        (Lifetime::Specified(_), Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
        (Lifetime::Specified(_), Lifetime::NotSpecified) => quote::quote!{#[serde(borrow)]},
        (Lifetime::NotSpecified, Lifetime::Specified(_)) => quote::quote!{#[serde(borrow)]},
        (Lifetime::NotSpecified, Lifetime::NotSpecified) => proc_macro2::TokenStream::new(),
    };
    let error_message = "must not contain reserved by macro lifetime name:";
    key_vec_lifetime.into_iter().for_each(|k|{
        if let Lifetime::Specified(key_lifetime_specified) = k {
            if let true = &key_lifetime_specified == trait_lifetime_stringified {
                panic!("{proc_macro_name_ident_stringified} {error_message} {trait_lifetime_stringified}");
            }
            possible_lifetime_addition(
                key_lifetime_specified,
                lifetimes_for_serialize_deserialize
            );
        }
    });
    value_vec_lifetime.into_iter().for_each(|v|{
        if let Lifetime::Specified(value_lifetime_specified) = v {
            if let true = &value_lifetime_specified == trait_lifetime_stringified {
                panic!("{proc_macro_name_ident_stringified} {error_message} {trait_lifetime_stringified}");
            }
            possible_lifetime_addition(
                value_lifetime_specified,
                lifetimes_for_serialize_deserialize
            );
        }
    });
    token_stream
}

impl std::fmt::Display for Lifetime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Lifetime::Specified(l) => write!(f, "'{l}"),
            Lifetime::NotSpecified => write!(f, ""),
        }
    }
}

fn vec_lifetime_to_string(vec: &[Lifetime]) -> String {
    let mut lifetimes_stringified_handle = vec.iter().fold(String::from(""), |mut acc, path_segment| {
        acc.push_str(&format!("{},", path_segment));
        acc
    });
    lifetimes_stringified_handle.pop();
    format!("<{lifetimes_stringified_handle}>")
}

fn vec_lifetime_to_lifetime(vec: &Vec<Lifetime>) -> &Lifetime {
    let mut lifetime_handle = &Lifetime::NotSpecified;
    for lft in vec {
        if let Lifetime::Specified(_) = lft {
            lifetime_handle = lft;
            break;
        }
    }
    lifetime_handle
}

fn form_last_arg_lifetime_vec(
    segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>, 
    proc_macro_name_ident_stringified: &String, 
    supports_only_stringified: &str,
    is_none_stringified: &str,
    syn_generic_argument_type_stringified: &str,
) -> Vec<Lifetime> {
    if let Some(path_segment) = segments.last() {
        match &path_segment.arguments {
            syn::PathArguments::None => Vec::new(),
            syn::PathArguments::AngleBracketed(angle_bracketed_generic_argument) => {
                angle_bracketed_generic_argument.args.iter().map(|generic_argument|{
                    match generic_argument {
                        syn::GenericArgument::Lifetime(lfmt) => Lifetime::Specified(lfmt.ident.to_string()),
                        syn::GenericArgument::Type(_) => Lifetime::NotSpecified,
                        _ => panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() angle_bracketed_generic_argument.args[0] {supports_only_stringified} syn::GenericArgument::Lifetime and {syn_generic_argument_type_stringified}")
                    }
                })
                .collect()
            },
            syn::PathArguments::Parenthesized(_) => panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() is unexpected syn::PathArguments::Parenthesized"),
        }
    }
    else {
        panic!("{proc_macro_name_ident_stringified} type_path.path.segments.last() {is_none_stringified}");
    }
}

// fn lifetimes_for_serialize_deserialize_into_token_stream(
//     lifetimes_for_serialize_deserialize: Vec<String>,
//     trait_lifetime_stringified: &String,
//     proc_macro_name_ident_stringified: &String,
//     parse_proc_macro2_token_stream_failed_message: &str,
// ) -> proc_macro2::TokenStream {
//     if let true = lifetimes_for_serialize_deserialize.contains(&trait_lifetime_stringified.to_string()) {
//         panic!("{proc_macro_name_ident_stringified} must not contain reserved by macro lifetime name: {trait_lifetime_stringified}");
//     };
//     let mut lifetimes_for_serialize_deserialize_stringified = lifetimes_for_serialize_deserialize
//     .iter()
//     .fold(String::from(""), |mut acc, gen_param| {
//         acc.push_str(&format!("'{gen_param},"));
//         acc
//     });
//     lifetimes_for_serialize_deserialize_stringified.pop();
//     lifetimes_for_serialize_deserialize_stringified
//     .parse::<proc_macro2::TokenStream>()
//     .unwrap_or_else(|_| panic!("{proc_macro_name_ident_stringified} {lifetimes_for_serialize_deserialize_stringified} {parse_proc_macro2_token_stream_failed_message}"))
// }
