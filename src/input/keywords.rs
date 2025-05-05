//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

use std::any::TypeId;
use syn::token;

macro_rules! keywords {(
    tokens {$($tok_kw:path: $tok_variant:ident $(, $tok_feature:literal)?;)*}
    custom {$($kw:ident: $variant:ident $(,$feature:literal)?;)*}
) => {
        pub mod kw {
            $(
                $(#[cfg(feature = $feature)])?
                syn::custom_keyword!($kw);
            )*
        }
        pub enum Keyword {
            $(
                $(#[cfg(feature = $tok_feature)])?
                $tok_variant($tok_kw),
            )*
            $(
                $(#[cfg(feature = $feature)])?
                $variant(kw::$kw),
            )*
        }

        impl Keyword {
            pub fn is<KW: 'static>(&self) -> bool {
                match self {
                    $(
                        $(#[cfg(feature = $tok_feature)])?
                        Keyword::$tok_variant(_) => TypeId::of::<KW>() == TypeId::of::<$tok_kw>(),
                    )*
                    $(
                        $(#[cfg(feature = $feature)])?
                        Keyword::$variant(_) => TypeId::of::<KW>() == TypeId::of::<kw::$kw>(),
                    )*
                }
            }
        }
        $(
            $(#[cfg(feature = $tok_feature)])?
            impl PartialEq<Keyword> for $tok_kw {
                fn eq(&self, other: &Keyword) -> bool {
                    match other {
                        Keyword::$tok_variant(_) => true, // kw == self,
                        _ => false
                    }
                }
            }
            $(#[cfg(feature = $tok_feature)])?
            impl PartialEq<$tok_kw> for Keyword {
                fn eq(&self, _other: &$tok_kw) -> bool {
                    match self {
                        Keyword::$tok_variant(_) => true,
                        _ => false
                    }
                }
            }
            $(#[cfg(feature = $tok_feature)])?
            impl From<$tok_kw> for Keyword {
                fn from(value: $tok_kw) -> Self {
                    Keyword::$tok_variant(value)
                }
            }
            $(#[cfg(feature = $tok_feature)])?
            impl From<Keyword> for $tok_kw {
                fn from(value: Keyword) -> Self {
                    match value {
                        Keyword::$tok_variant(kw) => kw,
                        _ => panic!("Invalid conversion from Keyword to kw::{}!", stringify!($tok_variant))
                    }
                }
            }
        )*
        $(
            $(#[cfg(feature = $feature)])?
            impl PartialEq<Keyword> for kw::$kw {
                fn eq(&self, other: &Keyword) -> bool {
                    match other {
                        Keyword::$variant(_) => true, // kw == self,
                        _ => false
                    }
                }
            }
            $(#[cfg(feature = $feature)])?
            impl PartialEq<kw::$kw> for Keyword {
                fn eq(&self, _other: &kw::$kw) -> bool {
                    match self {
                        Keyword::$variant(_) => true,
                        _ => false
                    }
                }
            }
            $(#[cfg(feature = $feature)])?
            impl From<kw::$kw> for Keyword {
                fn from(value: kw::$kw) -> Self {
                    Keyword::$variant(value)
                }
            }
            $(#[cfg(feature = $feature)])?
            impl From<Keyword> for kw::$kw {
                fn from(value: Keyword) -> Self {
                    match value {
                        Keyword::$variant(kw) => kw,
                        _ => panic!("Invalid conversion from Keyword to kw::{}!", stringify!($variant))
                    }
                }
            }
        )*
    };
}

// NOTE: `paste` crate would be great here, but if we don't need it anywhere else (right now we don't)
//       we should avoid the just about unnecessary bulk and compile time addition
keywords!(
    tokens {
        token::Const: Const;
        token::Static: Static;
        token::As: As;
    }
    custom {
        lazy: Lazy, "lazy";
        alias: Alias, "alias";
        source: Source;
    }
);
