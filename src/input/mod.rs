//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use crate::input::syntax::{Context, CtxRef, Section};
use syn::parse::{Parse, ParseStream};

mod attribute;
mod keywords;
mod pattern;
mod rule;
mod segment;
mod syntax;

pub(crate) type Sections<'a> = Vec<Section<'a>>;

#[allow(dead_code)] // FIXME: remove after implementing
pub struct MacroInput<'a> {
    global_ctx: Context<'a>,
    sections: Sections<'a>,
}

impl<'a> MacroInput<'a> {
    #[allow(dead_code)] // FIXME: remove after implementing
    pub(crate) fn get_global_ctx(&'a self) -> CtxRef<'a> {
        CtxRef::new(&self.global_ctx)
    }
}

impl<'a> Parse for MacroInput<'a> {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        // let global_ctx: Context<'a> = input.parse()?;
        todo!()
    }
}
