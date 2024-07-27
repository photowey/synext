/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(dead_code)]

// syntax/derive/parser

// ----------------------------------------------------------------

extern crate proc_macro;

use std::fmt::Display;

use proc_macro2::Span;
use syn::__private::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse, Data, DataStruct, DeriveInput, Field, Fields, GenericArgument, Path, PathArguments, Type,
};

// ----------------------------------------------------------------

pub const BUILTIN_TYPE_OPTION: &str = "Option";
pub const BUILTIN_TYPE_VEC: &str = "Vec";

// ----------------------------------------------------------------

/// Try parse [`proc_macro::TokenStream`] to [`syn::DeriveInput`].
pub fn try_derive_input(input: proc_macro::TokenStream) -> DeriveInput {
    parse(input).unwrap()
}

// ----------------------------------------------------------------

/// Try parse [`syn::DeriveInput`] named fields [`Punctuated<Field, Comma>`].
pub fn try_parse_named_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    let struct_name = &input.ident;

    // @formatter:off
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!(
                "synext: Does not contain named fields! target:`{}`",
                struct_name
            ),
        },
        _ => panic!(
            "synext: Only structs are supported! target:`{}`",
            struct_name
        ),
    }
    // @formatter:on
}

// ----------------------------------------------------------------

/// Try parse [`syn::DeriveInput`] unnamed fields [`Punctuated<Field, Comma>`].
pub fn try_parse_unnamed_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    let struct_name = &input.ident;

    // @formatter:off
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Unnamed(fields) => &fields.unnamed,
            _ => panic!(
                "synext: Does not contain unnamed fields! target:`{}`",
                struct_name
            ),
        },
        // @formatter:on
        _ => panic!(
            "synext: Only structs are supported! target:`{}`",
            struct_name
        ),
    }
}

// ----------------------------------------------------------------

/// Try parse [`syn::DeriveInput`] matches fields [`Punctuated<Field, Comma>`].
pub fn try_match_fields(input: &DeriveInput) -> &Punctuated<Field, Comma> {
    let struct_name = &input.ident;

    // @formatter:off
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(fields),
            ..
        }) => &fields.unnamed,
        _ => panic!(
            "synext: Does not contain any fields! target:`{}`",
            struct_name
        ),
    }
    // @formatter:on
}

// ----------------------------------------------------------------

/// Try unwrap `syn::Type` [`core::option::Option<T>`] inner types.
pub fn try_unwrap_option(ty: &Type) -> &Type {
    try_unwrap_types(BUILTIN_TYPE_OPTION, 1, ty).unwrap()[0]
}

/// Try unwrap `syn::Type` [`Vec`] inner types.
pub fn try_unwrap_vec(ty: &Type) -> &Type {
    try_unwrap_types(BUILTIN_TYPE_VEC, 1, ty).unwrap()[0]
}

pub fn try_unwrap_types<'a>(
    ident: &str,
    target_types: usize,
    ty: &'a Type,
) -> Option<Vec<&'a Type>> {
    // @formatter:off
    if let Type::Path(syn::TypePath { ref path, .. }) = ty {
        // @formatter:on
        if try_predicate_is_ident(&ident, &path) && try_predicate_path_segments_is_not_empty(path) {
            let inner_type = try_extract_inner_types(ty);
            let mut len = 0;
            if let Some(ref inner) = inner_type {
                len = inner.len()
            }

            if len == target_types {
                return inner_type;
            } else {
                panic!("synext: Type `{}` has more inner Types then expected! (expected: {} | got: {})", ident, target_types, len);
            }
        }

        if try_predicate_is_not_ident(&ident, &path) {
            let res_ident = path.get_ident();
            if let Some(res_ident) = res_ident {
                panic!("synext: Expected Type `{:?}`, got `{:?}`", ident, res_ident);
            } else {
                panic!("synext: Expected Type `{:?}`, but has no type!", ident);
            }
        }
    }
    None
}

/// Extracts the type of types
///
/// - Option<T> -> T
/// - Vec<T> -> T
/// - Result<T, E> -> T, E
/// - String -> None
/// - ...
pub fn try_extract_inner_types(ty: &Type) -> Option<Vec<&Type>> {
    // @formatter:off
    if let Type::Path(syn::TypePath { ref path, .. }) = ty {
        // @formatter:on
        if try_predicate_path_segments_is_not_empty(path) {
            if let PathArguments::AngleBracketed(ref bracketed_generics) =
                path.segments.last().unwrap().arguments
            {
                let mut ty_vec = Vec::new();

                for generic in bracketed_generics.args.iter() {
                    if let GenericArgument::Type(ref ty) = generic {
                        ty_vec.push(ty);
                    }
                }

                if !ty_vec.is_empty() {
                    return Some(ty_vec);
                }
            }
        }
    }
    None
}

// ----------------------------------------------------------------

pub fn make_new_compile_error<T: Display>(span: Span, message: T) -> proc_macro::TokenStream {
    syn::Error::new(span, message).to_compile_error().into()
}

pub fn make_new_spanned_compile_error<T: ToTokens, U: Display>(
    tokens: T,
    message: U,
) -> proc_macro::TokenStream {
    syn::Error::new_spanned(tokens, message)
        .to_compile_error()
        .into()
}

// ---------------------------------------------------------------- boolean.function

pub fn try_predicate_is_option(ty: &Type) -> bool {
    try_predicate_is_type(BUILTIN_TYPE_OPTION, 1, ty)
}

pub fn try_predicate_is_vec(ty: &Type) -> bool {
    try_predicate_is_type(BUILTIN_TYPE_VEC, 1, ty)
}

pub fn try_predicate_is_type(ident: &str, target_types: usize, ty: &Type) -> bool {
    // @formatter:off
    if let Type::Path(syn::TypePath { ref path, .. }) = ty {
        // @formatter:on
        if try_predicate_is_ident(&ident, &path) && path.segments.len() == target_types {
            return true;
        }
    }
    false
}

pub fn try_predicate_is_not_ident(ident: &str, path: &Path) -> bool {
    !try_predicate_is_ident(ident, path)
}

pub fn try_predicate_is_ident(ident: &str, path: &Path) -> bool {
    if !path.segments.is_empty() && path.segments.last().unwrap().ident == ident {
        true
    } else {
        false
    }
}

pub fn try_predicate_path_segments_is_not_empty(path: &Path) -> bool {
    !try_predicate_path_segments_is_empty(path)
}

pub fn try_predicate_path_segments_is_empty(path: &Path) -> bool {
    path.segments.is_empty()
}
