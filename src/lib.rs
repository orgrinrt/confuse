//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use crate::input::MacroInput;
use syn::parse_macro_input;

mod input;

#[proc_macro]
pub fn bind<'a>(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _input = parse_macro_input!(input as MacroInput<'a>);
    // TODO: actual impl
    let output = proc_macro::TokenStream::new();
    output
}
