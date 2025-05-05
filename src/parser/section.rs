//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use crate::parser::attribute::Attributable;
use proc_macro2::Ident;
use std::slice::Iter;
use syn::{Attribute, Path};

pub struct Section<'s> {
    pub module: Path, // to support nested section defs, increase flexibility, `Path` instead of `Ident`
    pub vis: Option<syn::Visibility>, // default to pub(crate) ?
    attributes: Vec<<Self as Attributable<'s>>::Attribute>,
}

impl From<Section<'_>> for Ident {
    fn from(value: Section<'_>) -> Self {
        value
            .module
            .segments
            .last()
            .expect("Expected module path to have at least one segment to use as `Ident`")
            .ident
            .clone()
    }
}

impl<'s, 'a: 's> Attributable<'a> for Section<'s> {
    type Attribute = Attribute;
    type Iterator = Iter<'a, Self::Attribute>;

    fn get_attributes(&'a self) -> Self::Iterator {
        self.attributes.iter()
    }

    fn add_attribute(&'a mut self, attribute: &'a Self::Attribute) {
        self.attributes.push(attribute.clone());
    }
}
