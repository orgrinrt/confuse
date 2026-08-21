//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

//! A `#![no_std]` crate with no allocator gets working constants out of a binding.
//!
//! The claim is about the expansion, not about this crate, which is a proc macro and runs
//! inside the compiler with `std` on every configuration. Building this crate proves nothing
//! about it, so a consumer is written out and built instead.
//!
//! What a binding expands into is `tomlfuse`'s expansion, so the guarantee is inherited
//! rather than made here. That is the reason to test it here anyway: an inherited guarantee
//! is one nobody re-checks, and the forwarding in `src/lib.rs` is a place a `std` path could
//! be introduced without anyone noticing which crate it came from.
//!
//! Every consumer below also names `tomlfuse` in its own dependencies, which is not
//! incidental. A binding expands to `::tomlfuse::file!`, an absolute path in the *consumer's*
//! crate graph, so it resolves only where the consumer depends on `tomlfuse` under that name.
//! A proc macro cannot re-export another proc-macro crate, so this crate cannot supply it,
//! and `a_consumer_without_tomlfuse_cannot_resolve_the_binding` is what holds the constraint.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// The toml every consumer below reads. One value of each type the expansion can produce.
const FIXTURE: &str = r#"
[surface]
text = "a string"
count = 3
ratio = 1.5
flag = true
list = ["one", "two"]
"#;

/// Reads one const of every type the expansion produces, under `#![no_std]`.
const USES_EVERY_TYPE: &str = r#"#![no_std]

confuse::bind! {
    "surface.toml"
    [surface]
    surface.*
}

pub const TEXT: &str = surface::TEXT;
pub const COUNT: i64 = surface::COUNT;
pub const RATIO: f64 = surface::RATIO;
pub const FLAG: bool = surface::FLAG;
pub const LIST: &[&str] = surface::LIST;
"#;

/// The `tomlfuse` requirement, spelled the way this crate's own dev-dependency spells it so
/// the two move together.
const TOMLFUSE_DEP: &str =
    r#"tomlfuse = { git = "https://github.com/orgrinrt/tomlfuse.git", version = "0.0.4" }"#;

/// Writes a consumer crate and builds it against this one at `features`.
///
/// `extra_deps` is pasted into the manifest's `[dependencies]`, which is how the case below
/// leaves `tomlfuse` out.
fn consumer_compiles(
    name: &str,
    features: &[&str],
    extra_deps: &str,
    body: &str,
) -> (bool, String) {
    let root =
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/target/no-std-consumers")).join(name);
    fs::create_dir_all(root.join("src")).expect("the consumer directory");
    fs::write(root.join("surface.toml"), FIXTURE).expect("the consumer fixture");

    let features_list = features
        .iter()
        .map(|f| format!("\"{f}\""))
        .collect::<Vec<_>>()
        .join(", ");
    fs::write(
        root.join("Cargo.toml"),
        format!(
            r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2021"

[dependencies]
{extra_deps}

[dependencies.confuse]
path = "{crate_dir}"
default-features = false
features = [{features_list}]

[workspace]
"#,
            name = name,
            extra_deps = extra_deps,
            crate_dir = env!("CARGO_MANIFEST_DIR"),
            features_list = features_list,
        ),
    )
    .expect("the consumer manifest");
    fs::write(root.join("src/lib.rs"), body).expect("the consumer source");

    let output = Command::new(env!("CARGO"))
        .args(["check", "--quiet"])
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()
        .expect("cargo runs");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn a_no_std_consumer_compiles_on_every_selection() {
    // `toml` is in the default set and is what a binding needs; the other flags are along for
    // the ride, so each selection here carries it.
    for features in [
        &["toml"][..],
        &["toml", "no_std"][..],
        &["toml", "no_alloc"][..],
        &["toml", "patterns", "alias", "no_std", "no_alloc"][..],
    ] {
        let name = features.join("_");
        let (ok, stderr) =
            consumer_compiles(&format!("nostd_{name}"), features, TOMLFUSE_DEP, USES_EVERY_TYPE);
        assert!(
            ok,
            "a `#![no_std]` consumer failed to build against features {features:?}:\n{stderr}"
        );
    }
}

#[test]
fn the_no_std_consumer_really_has_no_std() {
    // Identical to the cases above but for the last line, which is the only thing that can
    // account for a difference in the outcome. Without this, a consumer that lost its
    // attribute would pass every case above while proving nothing about `no_std`.
    let body = format!(
        "{USES_EVERY_TYPE}\npub fn control() -> std::string::String {{ std::string::String::new() }}\n"
    );
    let (ok, stderr) =
        consumer_compiles("nostd_control", &["toml", "no_std"], TOMLFUSE_DEP, &body);
    assert!(
        !ok,
        "the control consumer named `std` and compiled anyway, so `#![no_std]` is not in \
         effect and every other case here proves nothing"
    );
    assert!(
        stderr.contains("unresolved module or unlinked crate `std`")
            || stderr.contains("failed to resolve"),
        "the control failed for some reason other than `std` being absent, which is not the \
         thing it is here to establish:\n{stderr}"
    );
}

#[test]
fn a_consumer_without_tomlfuse_cannot_resolve_the_binding() {
    // The documented constraint, and the one a reader is most likely to hit. A binding
    // expands to `::tomlfuse::file!`, and that path is resolved in the consumer's crate, so
    // depending on this crate alone is not enough. Nothing here can fix that: a proc-macro
    // crate exports macros and nothing else, so this one cannot hand its own `tomlfuse` on.
    //
    // What this test is for is the day somebody changes the forwarding. If the expansion ever
    // stops naming `tomlfuse`, this fails, and the README stops being true in the same
    // moment.
    let (ok, stderr) =
        consumer_compiles("without_tomlfuse", &["toml"], "", USES_EVERY_TYPE);
    assert!(
        !ok,
        "a consumer that does not depend on `tomlfuse` compiled a binding anyway. Either the \
         expansion stopped naming it, in which case the README's requirement is stale, or it \
         resolved some other way, which is worth knowing about."
    );
    assert!(
        stderr.contains("tomlfuse"),
        "the consumer failed for some reason other than `tomlfuse` being absent:\n{stderr}"
    );
}
