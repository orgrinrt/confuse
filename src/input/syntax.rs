//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use crate::input::attribute::Attributable;
use crate::input::pattern::PathPattern;
use core::fmt::Debug;
use proc_macro2::Ident;
use std::fmt::Formatter;
use std::slice::Iter;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Attribute;
use syn::FieldMutability;
use syn::Path;
use syn::Type;
use syn::Visibility;

macro_rules! syntax_node {
    ($($name:ident : $val_ty:ty { $(ref as $ref_name:ident)? defs {$($def:tt)*} parse($input:ident, $out:ident) {$($parse:tt)*}})*) => {
        $(
            #[derive(Clone, Default)]
            pub(crate) struct $name<'a> {
                // stable syntax nodes
                pub(crate) vis: Option<Visibility>,
                pub(crate) mutability: Option<FieldMutability>,
                pub(crate) ident: Option<Path>,
                pub(crate) val: Option<$val_ty>,
                pub(crate) ty: Option<Type>,
                // custom definition
                $($def)*
                // privates
                attributes: Vec<<Self as Attributable<'a>>::Attribute>,
            }
            impl<'a> Debug for $name<'a> {
                fn fmt(&self, fmt: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
                    todo!()
                }
            }

            impl<'a> Parse for $name<'a> {
                fn parse(_input: ParseStream) -> syn::Result<Self> {
                    let $input = _input.fork();
                    let mut $out: Self = Self::default();
                    $($parse)*
                    Ok($out)
                }
            }

            $(
                #[derive(Clone)]
                pub(crate) struct $ref_name<'a>(&'a $name<'a>);
                impl<'a> $ref_name<'a> {
                    pub fn new(value: &'a $name<'a>) -> $ref_name<'a> {
                        Self(value)
                    }
                }

                impl<'a> AsRef<$name<'a>> for $ref_name<'a> {
                    fn as_ref(&self) -> &'a $name<'a> {
                        self.0
                    }
                }
            )?

            impl<'a> From<$name<'a>> for Ident {
                fn from(value: $name<'a>) -> Self {
                    value
                        .ident
                        .expect(&format!("Expected a valid ident for {}", stringify!($name)))
                        .segments
                        .last()
                        .expect("Expected module path to have at least one segment to use as `Ident`")
                        .ident
                        .clone()
                }
            }

            impl<'a> From<$name<'a>> for syn::Field {
                fn from(value: $name<'a>) -> Self {
                    syn::Field {
                        attrs: value.attributes.clone(),
                        vis: value.vis.as_ref().unwrap_or(&Visibility::Inherited).clone(),
                        ident: Some(Ident::from(value.clone())),
                        mutability: value.mutability.as_ref().unwrap_or(&FieldMutability::None).clone(),
                        colon_token: None,
                        ty: value.ty.expect(&format!("Expected a type for {}", stringify!($name))).clone(),
                    }
                }
            }

            impl<'r, 'a: 'r> Attributable<'a> for $name<'r> {
                type Attribute = Attribute;
                type Iterator = Iter<'a, Self::Attribute>;

                fn get_attributes(&'a self) -> Self::Iterator {
                    self.attributes.iter()
                }

                fn add_attribute(&'a mut self, attribute: &Self::Attribute) {
                    self.attributes.push(attribute.clone());
                }
            }
        )*
    };
}

syntax_node!(
    Context: Vec<Def<'a>> {
        ref as CtxRef
        defs {}
        parse(input, out) {
            todo!();
        }
    }
    Def: syn::Expr {
        defs {}
        parse(input, out) {
            todo!();
        }
    }
    Section: Vec<Rule<'a>> {
        defs {
            global_ctx: Option<CtxRef<'a>>, // maybe Arc?
            local_ctx: Context<'a>,
        }
        parse(input, out) {
            todo!();
        }
    }
    Rule: PathPattern {
        defs {}
        parse(input, out) {
            todo!();
        }
    }
);
