//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use crate::parser::attribute::Attributable;
use crate::parser::pattern::Pattern;
use std::slice::Iter;
use syn::{Attribute, Token, Visibility};

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
pub struct Rule<'r> {
    pub vis: Option<Visibility>,
    pub resolve_mode: ResolveMode,
    pub matcher: PathPattern,
    #[cfg(feature = "alias")]
    pub alias: Option<PathPattern>,
    attributes: Vec<<Self as Attributable<'r>>::Attribute>,
}

impl<'r, 'a: 'r> Attributable<'a> for Rule<'r> {
    type Attribute = Attribute;
    type Iterator = Iter<'a, Self::Attribute>;

    fn get_attributes(&'a self) -> Self::Iterator {
        self.attributes.iter()
    }

    fn add_attribute(&'a mut self, attribute: &Self::Attribute) {
        self.attributes.push(attribute.clone());
    }
}
