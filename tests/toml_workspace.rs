//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

//! `workspace!`, which is public and documented and had no test.
//!
//! The file was here before this, holding a licence header and nothing else, which reads as
//! coverage from the outside and is not. It also sat in `tests/toml/`, a subdirectory cargo
//! does not scan for test targets, so nothing in it would have run even had it held a test.
//!
//! The macro reads the workspace manifest rather than the package one, so the two constants
//! below come from `[workspace.metadata]` in this crate's own `Cargo.toml`, which is both the
//! package manifest and the workspace manifest here.

use confuse::workspace;

workspace! {
    [meta]
    workspace.metadata.*
}

#[test]
fn the_workspace_manifest_is_what_gets_read() {
    assert_eq!(meta::PURPOSE, "exercising the workspace! macro");
    assert_eq!(meta::COUNT, 3);
}

#[test]
fn the_values_match_the_manifest_on_disk() {
    // The assertions above are literals, so a binding that silently stopped reading the file
    // would be caught only by them disagreeing with the manifest. Reading it back here is
    // what ties them to it: edit the metadata and this test says so rather than the constants
    // quietly holding what somebody typed into a test a year ago.
    let manifest: toml::Table =
        toml::from_str(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
            .expect("the manifest reads"))
        .expect("the manifest parses");

    let metadata = manifest["workspace"]["metadata"]
        .as_table()
        .expect("[workspace.metadata] is a table");

    assert_eq!(meta::PURPOSE, metadata["purpose"].as_str().expect("a string"));
    assert_eq!(meta::COUNT, metadata["count"].as_integer().expect("an integer"));
}
