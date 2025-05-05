//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use proc_macro::Ident;
use syn::punctuated::Punctuated;
use syn::LitStr;

#[cfg(feature = "patterns")]
#[derive(Clone)]
pub enum GlobSegment {
    Star,
    #[cfg(feature = "advanced_globs")]
    DoubleStar,
    QuestionMark,
    #[cfg(feature = "advanced_globs")]
    Collection(Punctuated<String, syn::Token![|]>),
}

#[derive(Clone)]
pub enum Segment {
    #[cfg(feature = "patterns")]
    Negation,
    Ident(Ident),
    LitStr(LitStr),
    #[cfg(feature = "patterns")]
    Glob(GlobSegment),
}
