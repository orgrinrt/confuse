//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use syn::Attribute;

pub trait AttributeWithPath<'p> {
    fn get_path(&'p self) -> &'p syn::Path;
}
impl<'p> AttributeWithPath<'p> for Attribute {
    fn get_path(&'p self) -> &'p syn::Path {
        self.path()
    }
}

// TODO: do we need some extra stuff beyond `syn::Attribute`?
pub trait Attributable<'a> {
    type Attribute: AttributeWithPath<'a> + 'a;
    type Iterator: Iterator<Item = &'a Self::Attribute>;
    fn parse_attributes(&self) -> Self::Iterator {
        todo!()
    }
    fn has_attribute(&'a self, attribute: &'a str) -> bool {
        self.get_attribute(attribute).is_some() // TODO: we can likely improve efficiency here
    }
    fn get_attributes(&'a self) -> Self::Iterator;
    fn get_attribute(&'a self, attribute: &'a str) -> Option<&'a Self::Attribute> {
        self.get_attributes()
            .find(|attr| attr.get_path().is_ident(attribute))
    }
    fn add_attribute(&'a mut self, attribute: &'a Self::Attribute);
}
