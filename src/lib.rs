//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

// The generic parser for the full `bind!` surface: several sources, formats other than toml, and
// the binding forms beyond a file. It is not wired into the macro yet, because it does not parse
// anything yet, so its items are unused for now.
#[allow(dead_code, unused_imports)]
mod input;

/// Binds a structured data file into typed build-time constants.
///
/// The single-source toml case is `tomlfuse`'s `file!` macro, and `bind!` writes that case in
/// the same syntax, so the input is handed to it rather than binding toml a second time here.
/// Anything the wider design adds on top, meaning several sources in one invocation and formats
/// other than toml, is not implemented: such input reaches tomlfuse's parser and is rejected
/// there.
///
/// Expanding to `::tomlfuse::file!` means the calling crate needs `tomlfuse` among its
/// dependencies alongside this one.
///
/// ```ignore
/// confuse::bind! {
///     "config.toml"
///
///     [config]
///     settings.*
///     !settings.internal.*
/// }
/// ```
#[proc_macro]
pub fn bind(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let forwarded: TokenStream2 = input.into();
    quote! {
        ::tomlfuse::file! { #forwarded }
    }
    .into()
}

/// Binds a toml file, the same as `tomlfuse`'s `file!`.
///
/// Forwarded to it rather than reimplemented; see [`bind!`] for why.
#[proc_macro]
pub fn file(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let forwarded: TokenStream2 = input.into();
    quote! {
        ::tomlfuse::file! { #forwarded }
    }
    .into()
}

/// Binds the current package's manifest, the same as `tomlfuse`'s `package!`.
#[proc_macro]
pub fn package(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let forwarded: TokenStream2 = input.into();
    quote! {
        ::tomlfuse::package! { #forwarded }
    }
    .into()
}

/// Binds the workspace manifest, the same as `tomlfuse`'s `workspace!`.
#[proc_macro]
pub fn workspace(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let forwarded: TokenStream2 = input.into();
    quote! {
        ::tomlfuse::workspace! { #forwarded }
    }
    .into()
}
