//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

//! `package!` and `workspace!`, which read a manifest without being told where it is.
//!
//! `package!` finds the `Cargo.toml` this crate is built from and `workspace!` finds the
//! workspace one. They answer what a binary asks about itself, which is what `--version`
//! prints and what a bug report needs.
//!
//! `env!("CARGO_PKG_VERSION")` covers the crate's own version and stops there. The manifest
//! holds every version the build resolved against, and a bug report naming them is one nobody
//! has to ask a follow-up question about.

use confuse::{package, workspace};

package! {
    // What this crate calls itself, minus the metadata table, which is where a project keeps
    // things that are nobody else's business.
    [pkg]
    package.*
    !package.metadata.*

    // What it was compiled against.
    [deps]
    dependencies.*
}

workspace! {
    [ws]
    workspace.metadata.*
}

fn main() {
    println!("{} {}", pkg::NAME, pkg::VERSION);
    println!(
        "  licensed {}, built for rust {}",
        pkg::LICENSE,
        pkg::RUST_VERSION
    );
    println!("  {}", pkg::DESCRIPTION.trim());
    println!();

    // A dependency written as a table becomes a module, so its version is a constant inside
    // one. A dependency written as a bare string is the string, and would be `deps::SYN`.
    // Which one a name resolves to is decided by how the manifest spells it, so changing
    // `syn = "^2.0"` into `syn = { version = "^2.0" }` moves the constant, and the build says
    // so rather than the value quietly becoming something else.
    println!("built against");
    println!("  syn    {}", deps::syn::VERSION);
    println!("  quote  {}", deps::quote::VERSION);
    println!("  eyre   {}", deps::eyre::VERSION);
    println!();

    println!("workspace says: {} ({})", ws::PURPOSE, ws::COUNT);
}
