//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use crate::input::attribute::Attributable;
use crate::input::attribute::ParsableAttributes;
use crate::input::pattern::PathPattern;
use core::fmt::Debug;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::fmt::Formatter;
use std::slice::Iter;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Attribute;
use syn::Path;
use syn::Token;
use syn::Type;
use syn::Visibility;

#[cfg(test)]
#[allow(unused_macro_rules)]
macro_rules! __validate_parse {
    ($target:ident {
        $test_submod:ident for $val_input:ident {
            $($test_name:ident { $($val_quote:tt)* })*
        } as $val_parsed:ident {
            $($validation:tt)*
        }
    }) => {
        use super::*;
        #[cfg(test)]
        #[allow(non_snake_case)]
        mod $test_submod {
            use super::*;
            #[allow(unused_variables)]
            fn validate($val_input: TokenStream, $val_parsed: $target::<'_>) {
                $($validation)*
            }
            $(
                #[test]
                #[allow(non_snake_case)]
                #[allow(unused_variables)]
                fn $test_name() {
                    #[allow(unused_variables)]
                    let input = quote! {
                        $($val_quote)*
                    };
                    #[allow(unused_variables)]
                    let fork = input.clone();
                    #[allow(unused_variables)]
                    let parsed = syn::parse2::<$target::<'_>>(input);
                    assert!(parsed.is_ok(), "Failed to parse {}: {:?}", stringify!($target), parsed.err());
                    validate(fork, parsed.unwrap());
                }
            )*
        }
    }
}

macro_rules! syntax_nodes {
    (
        $(
            $name:ident {
                $(ref as $ref_name:ident)?
                $(defs { $($def:tt)* })?
                $(#[attrs($parse_attrs:expr)])?
                $(#[field_meta($parse_as_field:expr)])?
                $(parse($orig_input:ident, $input:ident, $out:ident) {
                    $($parse:tt)*
                })?
                $(validate {
                    $(
                        $([$($test_cat:ident).+])? $test_submod:ident for $val_input:ident {
                            $($test_name:ident { $($val_quote:tt)* })*
                        } as $val_parsed:ident {
                            $($validation:tt)*
                        }
                    )*
                })?
            }
        )*
    ) => {
        $(
            #[derive(Clone, Default)]
            #[allow(non_camel_case_types)]
            #[allow(dead_code)]
            pub(crate) struct $name<'a> {
                // stable syntax nodes
                pub(crate) vis: Option<Visibility>,
                pub(crate) mutability: Option<Token![mut]>,
                pub(crate) ident: Option<Path>,
                pub(crate) ty: Option<Type>,
                // custom definition
                $($($def)*)?
                // privates
                attributes: Vec<<Self as Attributable<'a>>::Attribute>,
            }

            impl<'a> Debug for $name<'a> {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    let tokens = self.to_token_stream();
                    write!(f, "{}", tokens)
                }
            }

            impl<'a> Parse for $name<'a> {
                #[allow(unused_variables)]
                fn parse(orig_input: ParseStream) -> syn::Result<Self> {
                    $(let $orig_input = orig_input.fork();)?
                    let mut __out__: $name::<'a> = $name::<'a>::default();

                    #[allow(unused_mut)]
                    let mut __parse_attrs__: bool = true; // by default, we parse attributes
                    $(__parse_attrs__ = $parse_attrs;)?
                    let _input = if __parse_attrs__ {
                        __out__.parse_attributes(orig_input).map_err(|e| {
                            syn::Error::new(orig_input.span(), format!("Failed to parse attributes: {}", e))
                        })?
                    } else {
                        orig_input
                    };

                    #[allow(unused_mut)]
                    let mut __parse_field_meta__: bool = true; // by default, we parse potential field meta
                    $(__parse_field_meta__ = $parse_as_field;)?
                    if __parse_field_meta__ {
                        if _input.peek(syn::token::Pub) {
                            __out__.vis = Some(_input.parse()?);
                        }

                        if _input.peek(syn::token::Mut) {
                            __out__.mutability = Some(_input.parse::<Token![mut]>()?);
                        }

                        if !_input.peek(syn::token::Colon) && !_input.peek(syn::token::Eq) {
                            __out__.ident = Some(_input.parse()?);
                        }

                        if _input.peek(syn::token::Colon) {
                            let _: syn::token::Colon = _input.parse()?;
                            __out__.ty = Some(_input.parse()?);
                        }
                    }

                    #[allow(unused_mut)]
                    #[allow(unused_variables)]
                    $(let mut $out: $name::<'a> = __out__;)?
                    #[allow(unused_mut)]
                    #[allow(unused_variables)]
                    $(let mut $input: ParseStream = _input;)?

                    $($($parse)*)?

                    #[allow(unreachable_code)]
                    $(Ok($out))?
                }
            }

            impl<'a> ToTokens for $name<'a> {
                #[allow(unused_variables)]
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    for attr in self.get_attributes() {
                        attr.to_tokens(tokens);
                    }

                    if let Some(vis) = &self.vis {
                        vis.to_tokens(tokens);
                    }

                    if let Some(mutability) = &self.mutability {
                        mutability.to_tokens(tokens);
                    }

                    if let Some(ident) = &self.ident {
                        ident.to_tokens(tokens);
                    }

                    if let Some(ty) = &self.ty {
                        tokens.extend(quote!( : ));
                        ty.to_tokens(tokens);
                    }

                }
            }

            $(
                #[derive(Clone)]
                #[allow(non_camel_case_types)]
                pub(crate) struct $ref_name<'a>(&'a $name<'a>);

                // TODO: actual convenience stuff

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

            // impl<'a> From<$name<'a>> for syn::Field {
            //     fn from(value: $name<'a>) -> Self {
            //         SYN::Field {
            //             ATTRS: value.attributes.clone(),
            //             VIS: value.vis.as_ref().unwrap_or(&Visibility::Inherited).clone(),
            //             IDENT: Some(Ident::from(value.clone())),
            //             // NOTE: `mutability` is not used apparently (it's a reserved feature for later?), but we do need to set it
            //             MUTABILITY: FieldMutability::None,
            //             COLON_TOKEN: None,
            //             TY: value.ty.expect(&format!("Expected a type for {}", stringify!($name))).clone(),
            //         }
            //     }
            // }

            impl<'a> Attributable<'a> for $name<'_> {
                type Attribute = Attribute;
                type Iterator = Iter<'a, Self::Attribute>;

                fn get_attributes(&'a self) -> Self::Iterator {
                    self.attributes.iter()
                }

                fn add_attribute(&mut self, attribute: &Self::Attribute) {
                    self.attributes.push(attribute.clone());
                }
            }
        )*
        #[cfg(test)]
        mod test {
            use super::*;

            $(
                $(
                    #[cfg(test)]
                    paste::paste! {
                        #[allow(non_snake_case)]
                        mod [<$name:lower>] {
                            $(
                                __validate_parse!($name {
                                    [< $($($test_cat:lower _)+)? $test_submod:lower >] for $val_input {
                                        $($test_name { $($val_quote)* })*
                                    } as $val_parsed {
                                        $($validation)*
                                    }
                                });
                            )*
                        }
                    }
                )*
            )?
        }
    };
}

syntax_nodes!(
    Context {
        ref as CtxRef
        defs {
            defs: Vec<Def<'a>>,
        }
        parse(_orig_input, input, out) {
            todo!();
        }
    }
    Def {
        defs {
            value: Option<syn::Expr>,
        }
        parse(_orig_input, input, out) {
            let _eq: Token![=] = input.parse()?;
            out.value = Some(input.parse()?);
        }
        validate {
            [syntax] source for input {
                rust_like_explicit_source {                 source foo: toml = "foo/bar.toml" }
                rust_like_explicit_source_inferred_type_{   source foo = "foo/bar.toml"}
                rust_like_inferred_source {                 source "foo/bar.toml"}
                sole_str_lit_is_valid_source {              "foo/bar.toml"}
                as_syntax_sole_str_lit {                    "foo/bar.conf" as toml}
                as_syntax_explicit_source {                 source foo = "foo/bar.conf" as toml}
                as_syntax_inferred_source {                 source "foo/bar.conf" as toml}
                redundant_but_valid {                       source foo: toml = "foo/bar.conf" as toml}
            } as parsed {
                // TODO: ensure that the resulting Def has all required fields with valid values
                todo!()
            }
        }
    }
    Section {
        defs {
            rules: Vec<Rule<'a>>,
            global_ctx: Option<CtxRef<'a>>,
            local_ctx: Context<'a>,
        }
        #[attrs(true)]
        #[field_meta(false)]
        parse(_orig_input, input, out) {
            todo!();
        }
    }
    Rule {
        defs {
            pat: PathPattern,
        }
        #[attrs(true)]
        parse(_orig_input, input, out) {
            todo!();
        }
        validate {
            [syntax] pats for input {
                direct_field {                 some.field.name }
                direct_root_field {            root_value }
                single_wildcard_simple {      config.* }
                single_wildcard_deeper {      defaults.__internal.foo.bar.baz.* }
            } as parsed {
                // TODO: ensure that the resulting Def has all required fields with valid values
                // FIXME: test
                todo!()
            }
        }
    }
);
