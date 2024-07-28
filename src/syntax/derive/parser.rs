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
use syn::spanned::Spanned;
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
#[rustfmt::skip]
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
#[rustfmt::skip]
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
#[rustfmt::skip]
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

#[rustfmt::skip]
pub fn try_unwrap_types<'a>(
    ident: &str,
    target_types: usize,
    ty: &'a Type,
) -> Option<Vec<&'a Type>> {
    // @formatter:off
    if let Type::Path(
        syn::TypePath {
            ref path,
            ..
        }) = ty {
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

/// Try to extract the inner type of [`syn::Type`]
///
/// - Option<T> -> T
/// - Vec<T> -> T
/// - Result<T, E> -> T, E
/// - String -> None
/// - ...
#[rustfmt::skip]
pub fn try_extract_inner_types(ty: &Type) -> Option<Vec<&Type>> {
    // @formatter:off
    if let Type::Path(
        syn::TypePath {
            ref path,
            ..
        }) = ty {
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

/// Try to extract the specified path attribute value from a field's attributes.
///
/// # Arguments
///
/// * `derive_attribute` - The identifier of the derive attribute that needs to be found.
/// * `path_attribute`   - The identifier of the key-value pair attribute within the derive
///                        attribute that needs to be extracted.
/// * `field`            - A reference to the `Field` struct which contains the attributes
///                        to be searched.
///
/// # Returns
///
/// * `Ok(Some(syn::Ident))` - If the specified path attribute is found, returns the identifier
///                            wrapped in `Some`.
/// * `Ok(None)`             - If the specified path attribute is not found.
/// * `Err(syn::Error)`      - If an error occurs during parsing or the expected attribute format
///                            is not met.
///
/// # Example:
///
/// ```ignore
/// extern crate proc_macro;
///
/// use proc_macro::TokenStream;
///
/// #[proc_macro_derive(Builder, attributes(builder))]
/// pub fn builder_derive(input: TokenStream) -> TokenStream {
///     TokenStream::new()
/// }
///
/// #[derive(Builder)]
/// pub struct Hello {
///     // derive_attribute = builder
///     // path_attribute = method
///     #[builder(method = "activity")]
///     activities: Vec<String>,
/// }
/// ```
///
/// @since 0.2.0
#[rustfmt::skip]
pub fn try_extract_field_attribute_path_attribute(derive_attribute: &str, path_attribute: &str, field: &Field) -> syn::Result<Option<syn::Ident>> {
    for attr in &field.attrs {
        // @formatter:off
        if let Ok(
            syn::Meta::List(
                syn::MetaList {
                    ref path,
                    ref nested,
                    ..
                })) = attr.parse_meta()
        {
            // @formatter:on
            if let Some(p) = path.segments.first() {
                if p.ident == derive_attribute {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first() {
                        if kv.path.is_ident(path_attribute) {
                            if let syn::Lit::Str(ref target_attr) = kv.lit {
                                return Ok(Some(syn::Ident::new(
                                    target_attr.value().as_str(),
                                    attr.span(),
                                )));
                            }
                        } else {
                            if let Ok(syn::Meta::List(ref list)) = attr.parse_meta() {
                                return Err(syn::Error::new_spanned(
                                    list,
                                    format!(
                                        r#"expected `{}({} = "...")`"#,
                                        derive_attribute, path_attribute
                                    ),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(None)
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

/// Try to predicate that [`syn::Type`] is neither of type [`core::option::Option<T>`] nor of type [`std::vec::Vec<T>`]
///
/// @since 0.2.0
pub fn try_predicate_is_not_option_and_vec(ty: &Type) -> bool {
    try_predicate_is_not_option(ty) && try_predicate_is_not_vec(ty)
}

/// Try to predicate that [`syn::Type`] is not [`core::option::Option<T>`] type.
///
/// @since 0.2.0
pub fn try_predicate_is_not_option(ty: &Type) -> bool {
    !try_predicate_is_option(ty)
}

/// Try to predicate that [`syn::Type`] is not [`std::vec::Vec<T>`] type.
///
/// @since 0.2.0
pub fn try_predicate_is_not_vec(ty: &Type) -> bool {
    !try_predicate_is_vec(ty)
}

/// Try to predicate that [`syn::Type`] is [`core::option::Option<T>`] type.
///
/// @since 0.2.0
pub fn try_predicate_is_option(ty: &Type) -> bool {
    try_predicate_is_type(BUILTIN_TYPE_OPTION, 1, ty)
}

/// Try to predicate that [`syn::Type`] is [`std::vec::Vec<T>`] type.
///
/// @since 0.2.0
pub fn try_predicate_is_vec(ty: &Type) -> bool {
    try_predicate_is_type(BUILTIN_TYPE_VEC, 1, ty)
}

#[rustfmt::skip]
pub fn try_predicate_is_type(ident: &str, target_types: usize, ty: &Type) -> bool {
    // @formatter:off
    if let Type::Path(
        syn::TypePath {
            ref path,
            ..
        }) = ty {
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
    try_predicate_path_segments_is_not_empty(path) && path.segments.last().unwrap().ident == ident
}

pub fn try_predicate_path_segments_is_not_empty(path: &Path) -> bool {
    !try_predicate_path_segments_is_empty(path)
}

pub fn try_predicate_path_segments_is_empty(path: &Path) -> bool {
    path.segments.is_empty()
}
