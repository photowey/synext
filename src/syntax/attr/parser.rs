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

// syntax/attr/parser

// ----------------------------------------------------------------

use syn::{AttributeArgs, Lit, Meta, NestedMeta};

// ----------------------------------------------------------------

/// Try to extract the specified attribute value from an attribute macro.
///
/// # Examples
///
///```ignore
/// extern crate proc_macro;
///
/// use proc_macro::TokenStream;
/// use std::sync::Arc;
///
///#[proc_macro_attribute]
/// pub fn component(args: TokenStream, item: TokenStream) -> TokenStream {
///   item
/// }
///
/// pub struct HelloService {
///   // ...
/// }
///
/// #[component(value = "helloController")]
/// pub struct HelloController {
///    hello_service: Arc<HelloService>,
/// }
///
/// ```
/// @since 0.3.0
pub fn try_extract_attribute_args(attr: &str, args: AttributeArgs) -> Option<String> {
    let mut attrbute = None;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.is_ident(attr) {
                    if let Lit::Str(n) = nv.lit {
                        attrbute = Some(n.value());
                    }
                }
            }
            _ => {}
        }
    }

    attrbute
}

/// Try to extract the first attribute value from an attribute macro.
///
/// # Examples
///
///```ignore
/// extern crate proc_macro;
///
/// use proc_macro::TokenStream;
/// use std::sync::Arc;
///
/// #[proc_macro_attribute]
/// pub fn component(args: TokenStream, item: TokenStream) -> TokenStream {
///    // ...
/// }
///
/// pub struct HelloService {
///    // ...
/// }
///
/// #[component("helloController")] // first
/// pub struct HelloController {
///     hello_service: Arc<HelloService>,
/// }
///
/// ->
/// try_extract_attribute_first_args(args);
/// ```
/// @since 0.3.0
pub fn try_extract_attribute_first_args(args: AttributeArgs) -> Option<String> {
    match args.first() {
        Some(NestedMeta::Lit(Lit::Str(v))) => Some(v.value()),
        _ => None,
    }
}
