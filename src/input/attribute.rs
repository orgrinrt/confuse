//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use syn::parse::ParseBuffer;
use syn::Attribute;

pub trait AttributeWithPath<'p> {
    fn get_path(&'p self) -> &'p syn::Path;
}
impl<'p> AttributeWithPath<'p> for Attribute {
    fn get_path(&'p self) -> &'p syn::Path {
        self.path()
    }
}

pub trait Attributable<'a>: ParsableAttributes<'a> {
    type Attribute: AttributeWithPath<'a> + 'a;
    type Iterator: Iterator<Item = &'a Self::Attribute>;
    fn has_attribute(&'a self, attribute: &'a str) -> bool {
        self.get_attribute(attribute).is_some() // TODO: we can likely improve efficiency here
    }
    fn get_attributes(&'a self) -> Self::Iterator;
    fn get_attribute(&'a self, attribute: &'a str) -> Option<&'a Self::Attribute> {
        self.get_attributes()
            .find(|attr: &&Self::Attribute| {
                // TODO: this is probably not actually correct?
                attr.get_path().is_ident(attribute)
            })
    }
    fn add_attribute(&mut self, attribute: &Self::Attribute);
}

pub trait ParsableAttributes<'a> {
    fn parse_attributes(&self, input: &'a ParseBuffer<'a>) -> eyre::Result<&'a ParseBuffer<'a>>;
}

impl<'a, A: Attributable<'a, Attribute=Attribute>> ParsableAttributes<'a> for A {
    fn parse_attributes(&self, input: &'a ParseBuffer<'a>) -> eyre::Result<&'a ParseBuffer<'a>> {
        if let Err(e) = input.call(Attribute::parse_outer) {
            Err(eyre::eyre!("Failed to parse attributes: {}", e))
        } else {
            Ok(input)
        }
    }
}
