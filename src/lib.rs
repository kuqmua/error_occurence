#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

use std::f32::consts::E;

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
    let ident_with_deserialize_token_stream = format!("{ident}WithDeserialize")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    let path_token_stream = format!("{path}")
        .parse::<proc_macro2::TokenStream>()
        .expect("path parse failed");
    // let fields =
    match ast.data {
        syn::Data::Struct(struct_item) => {
            let fields = struct_item.fields;

            let fields_named = match fields {
                syn::Fields::Named(fields_named) => fields_named,
                _ => panic!("ImplErrorOccurence only works with named fields"),
            };
            match fields_named.named.len() {
                2 => (),
                _ => {
                    panic!("ImplErrorOccurence fields_named.len() != 2")
                }
            }
            let source_type_ident = match &fields_named.named[0].ty {
                syn::Type::Path(type_path) => type_path,
                _ => panic!(
                    "ImplErrorOccurence only works on structs fields with  syn::Type::Path type"
                ),
            };
            let first_source_type_ident = source_type_ident.path.segments[0].ident.clone();
            // let first_source_type_ident_as_string = format!("{first_source_type_ident}");
            // let source_place_type_source_token_stream =
            //     format!("{path}::config_mods::source_place_type::SourcePlaceType::Source")
            //         .parse::<proc_macro2::TokenStream>()
            //         .expect("path parse failed");
            // let source_place_type_github_token_stream =
            //     format!("{path}::config_mods::source_place_type::SourcePlaceType::Github")
            //         .parse::<proc_macro2::TokenStream>()
            //         .expect("path parse failed");
            // let source_place_type_none_token_stream =
            //     format!("{path}::config_mods::source_place_type::SourcePlaceType::None")
            //         .parse::<proc_macro2::TokenStream>()
            //         .expect("path parse failed");
            // let with_tracing_token_stream = format!("{path}::traits::with_tracing::WithTracing")
            //     .parse::<proc_macro2::TokenStream>()
            //     .expect("path parse failed");
            // let where_was_token_stream = format!("{path}::common::where_was::WhereWas")
            //     .parse::<proc_macro2::TokenStream>()
            //     .expect("path parse failed");
            // let source_place_type_token_stream =
            //     format!("{path}::config_mods::source_place_type::SourcePlaceType")
            //         .parse::<proc_macro2::TokenStream>()
            //         .expect("path parse failed");
            // let tracing_token_stream = format!("{path}::config_mods::log_type::LogType::Tracing")
            //     .parse::<proc_macro2::TokenStream>()
            //     .expect("path parse failed");
            // let stack_token_stream = format!("{path}::config_mods::log_type::LogType::Stack")
            //     .parse::<proc_macro2::TokenStream>()
            //     .expect("path parse failed");
            // let none_token_stream = format!("{path}::config_mods::log_type::LogType::None")
            //     .parse::<proc_macro2::TokenStream>()
            //     .expect("path parse failed");
            // let error_color_token_stream = format!("{path}::traits::get_color::ErrorColorBold")
            //     .parse::<proc_macro2::TokenStream>()
            //     .expect("path parse failed");
            quote::quote! {}.into()
        }
        syn::Data::Enum(data_enum) => {
            println!("{:#?}", data_enum);
            let mut all_equal: Option<SuportedEnumVariant> = None;
            for variant in &data_enum.variants {
                match &variant.fields {
                    syn::Fields::Named(fields_named) => {
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
            match all_equal {
                Some(supported_enum_variant) => {
                    match supported_enum_variant {
                        SuportedEnumVariant::Named => {
                            let mut vec_needed_info: Vec<(&proc_macro2::Ident, ErrorFieldName, &syn::Type, &syn::Type)> = Vec::new();
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
                                                (error_field_name, &first_field.ty, &second_field.ty)
                                            },
                                            false => panic!("ImplErrorOccurence only works on named fields with length of 2"),
                                        }
                                    },
                                    syn::Fields::Unnamed(_) => panic!("ImplErrorOccurence unexpected named unnamed logic"),
                                    _ => panic!("ImplErrorOccurence only works with named fields"),
                                };
                                vec_needed_info.push((variant_ident, needed_info.0, needed_info.1, needed_info.2));
                            });
                            quote::quote! {
                                // #[derive(Debug, thiserror::Error, serde::Serialize)]
                                // pub enum EightOriginError<'a> {
                                //     Something {
                                //         error: String,
                                //         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
                                //     },
                                // }
                                //difference
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
                                        //error 
                                        _config: &ConfigGeneric
                                        //inner_error
                                        config: &ConfigGeneric
                                        //inner_errors
                                        config: &ConfigGeneric
                                    ) -> String {
                                        //error
                                        use #path_token_stream::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
                                        self.source_to_string_without_config()
                                        //inner_error
                                        use crate::traits::error_logs_logic::to_string_with_config::ToStringWithConfigForSourceToStringWithConfig;
                                        match self {
                                            ThreeWrapperError::Something {
                                                inner_error,
                                                code_occurence: _code_occurence,
                                            } => inner_error.to_string_with_config_for_source_to_string_with_config(config),
                                        }
                                        //inner_errors
                                        use crate::traits::error_logs_logic::few_to_string_with_config::FewToStringWithConfig;
                                        match self {
                                            SixWrapperError::Something {
                                                inner_errors,
                                                code_occurence: _code_occurence,
                                            } => inner_errors.few_to_string_with_config(config),
                                        }
                                    }
                                }
                                /////////////////////////////////

                                /////////////////////////////////
                                //difference
                                impl<'a>
                                    #path_token_stream::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
                                        'a,
                                    > for #ident<'a>
                                {
                                    fn source_to_string_without_config(&self) -> String {
                                        match self {
                                            #ident::Something {
                                                error,
                                                code_occurence: _code_occurence,
                                            } => format!("{}", error),
                                        }
                                    }
                                }
                                //difference(only in error naming)
                                impl<'a> #path_token_stream::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
                                    for #ident<'a>
                                {
                                    fn get_code_occurence(&self) -> &#path_token_stream::common::code_occurence::CodeOccurence<'a> {
                                        match self {
                                            #ident::Something {
                                                error: _error,
                                                code_occurence,
                                            } => code_occurence,
                                        }
                                    }
                                }
                                //difference
                                #[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
                                pub enum #ident_with_deserialize_token_stream<'a> {
                                    Something {
                                        error: String,
                                        #[serde(borrow)]
                                        code_occurence: #path_token_stream::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
                                    },
                                }
                                //difference
                                impl<'a> #path_token_stream::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<'a,> for #ident_with_deserialize_token_stream<'a>
                                {
                                    fn source_to_string_without_config(&self) -> String {
                                        match self {
                                            #ident_with_deserialize_token_stream::Something {
                                                error,
                                                code_occurence: _code_occurence,
                                            } => format!("{}", error),
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
                                            #ident_with_deserialize_token_stream::Something {
                                                error: _error,
                                                code_occurence,
                                            } => code_occurence,
                                        }
                                    }
                                }
                            };
                        },
                        SuportedEnumVariant::Unnamed => todo!(),
                    }
                    quote::quote! {
                        impl<'a> std::fmt::Display for #ident<'a> {
                            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                                use #path_token_stream::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                                write!(f, "{}", self.to_string_without_config())
                            }
                        }
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
