//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

//! The toml case binds through tomlfuse, so this checks that the macros here reach it and that
//! the constants come out of the file. Everything asserted below is read from
//! `tests/toml/test.toml` at compile time.

confound::bind! {
    "tests/test.toml"

    [basics]
    section.*

    [nested_values]
    nested.inner.*

    [without_internals]
    config.*
    !config.logging.*
}

#[test]
fn binds_scalars_from_the_file() {
    assert_eq!(basics::KEY, "value");
    assert_eq!(basics::NUMBER, 42);
}

#[test]
fn binds_arrays_from_the_file() {
    assert_eq!(basics::ARRAY, ["item1", "item2", "item3"]);
}

#[test]
fn flattens_a_nested_table() {
    // A `const` block, because every binding is a constant: this is checked when the test
    // compiles rather than when it runs, which is where a compile-time binding belongs.
    const { assert!(nested_values::VALUE) };
    assert_eq!(nested_values::STRING, "nested string");
}

#[test]
fn keeps_deeper_tables_as_modules() {
    assert_eq!(without_internals::settings::TIMEOUT, 500);
    assert_eq!(without_internals::settings::RETRIES, 3);
}

#[test]
fn honours_an_excluded_branch() {
    // `!config.logging.*` was excluded, so the module is bound without it
    const { assert!(!without_internals::DEBUG) };
}
