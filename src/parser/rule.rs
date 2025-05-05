//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use syn::{Attribute, Token};
use crate::parser::pattern::Pattern;
mod kw {
    #[cfg(feature = "alias")]
    syn::custom_keyword!(alias);
    #[cfg(feature = "lazy")]
    syn::custom_keyword!(lazy);
}

pub type PathPattern = Pattern<Token![.]>;

#[derive(Clone, Default)]
pub enum ResolveMode {
    #[cfg(feature = "lazy")]
    Lazy, // once_cell::sync::Lazy
    #[default]
    Const, // inlines always
    Static, // inlining left to compiler
    #[cfg(feature = "dyn")]
    Dyn, // maps to a member for struct items and variant for enum items. for rest, it's a pair of setter and getter funcs
}

#[derive(Clone, Default)]
pub struct Rule {
    pub attributes: Vec<Attribute>,
    pub resolve_mode: ResolveMode,
    pub matcher: PathPattern,
    #[cfg(feature = "alias")]
    pub alias: Option<PathPattern>,
}
