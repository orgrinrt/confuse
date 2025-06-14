//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use crate::input::segment::Segment;
use syn::punctuated::Punctuated;
use syn::Token;

pub(crate) type PathPattern = Pattern<Token![.]>;

#[derive(Clone, Default)]
pub struct Pattern<P: Clone = Token![.]> {
    pub(crate) segments: Vec<Segment>,
    pub(crate) punct: P,
}

impl<P: Clone> From<Punctuated<Segment, P>> for Pattern<P> {
    fn from(punctuated: Punctuated<Segment, P>) -> Self {
        let mut segments = Vec::new();
        let punct = punctuated
            .pairs()
            .next()
            .map(|pair| (pair.punct()).cloned())
            .expect("Expected a punct to exist in Punctuated");

        for segment in punctuated.iter().cloned() {
            segments.push(segment);
        }

        Pattern {
            segments,
            punct: punct.expect("Expected `punct` to contain something when constructing and returning the `Pattern`").clone(),
        }
    }
}

impl<P: Clone> From<Pattern<P>> for Punctuated<Segment, P> {
    fn from(value: Pattern<P>) -> Self {
        let mut punctuated = Punctuated::new();
        for segment in value.segments {
            punctuated.push_value(segment);
            punctuated.push_punct(value.punct.clone());
        }
        punctuated
    }
}
