//------------------------------------------------------------------------------
// Copyright (c) 2025                 orgrinrt           orgrinrt@ikiuni.dev
//                                    Hiisi Digital Oy   contact@hiisi.digital
// SPDX-License-Identifier: MPL-2.0    O. R. Toimela      N2963@student.jamk.fi
//------------------------------------------------------------------------------

//! `bind!` on a toml file, which is the case the crate currently binds.
//!
//! A section opens with its name in brackets and takes the patterns whose matches land in it.
//! `*` matches one segment, `**` any number, `{a,b}` any one alternative, `!` removes a match
//! rather than hiding it, and `alias` gives one path a name of its own.
//!
//! Every value is a constant resolved at compile time, so nothing here reads a file when it
//! runs, and a pattern that stops matching is a compile error rather than a missing value.
//!
//! # This needs `tomlfuse` too
//!
//! A binding expands to `::tomlfuse::file!`, and that path resolves in *this* crate rather
//! than in `confuse`, so `tomlfuse` has to be among the dependencies alongside it. A
//! proc-macro crate exports macros and nothing else, so `confuse` cannot pass its own on.

use confuse::bind;

bind! {
    "examples/service.toml"

    // What the service calls itself.
    [meta]
    service.*

    // Where it listens, minus the operator's own bookkeeping.
    [transport]
    transport.*
    !transport.internal.*

    // Two keys out of one section, with a name that says what the number is.
    [backoff]
    retries.attempts
    alias initial_delay_ms = retries.backoff_ms
}

fn main() {
    println!("{} {}", meta::NAME, meta::VERSION);
    println!("listening on {}:{}", transport::HOST, transport::PORT);
    println!(
        "retrying {} times, first after {}ms",
        backoff::ATTEMPTS,
        backoff::INITIAL_DELAY_MS
    );

    // transport::internal::REGION does not exist. An exclusion removes the name.
}
