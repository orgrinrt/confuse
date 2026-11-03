# `confound`

[![Crates.io](https://img.shields.io/crates/v/confound)](https://crates.io/crates/confound) [![docs.rs](https://img.shields.io/docsrs/confound)](https://docs.rs/confound)

> Structured data from files, bound into typed build-time constants with patterns and hierarchies.

`confound` is a set of proc macros, `bind!`, `file!`, `package!` and `workspace!`, that read a data file while the crate compiles and turn the keys you pick into constants. Which keys are taken is decided by glob patterns written inside the macro, tables become nested modules, an `alias` renames one path, a `!` prefix excludes, and comments in the source file become the doc comments of the constants. Only toml is bound so far, and the macros forward to [`tomlfuse`](https://github.com/orgrinrt/tomlfuse), which does the reading. The wider design, several sources in one invocation and formats like json and yaml, is not implemented, and that input is rejected by `tomlfuse`'s parser.

## Usage

```bash
cargo add confound tomlfuse
```

Both go in the dependencies, because a binding expands to `::tomlfuse::file!` and that path resolves in your crate, and a proc-macro crate can't hand its own dependencies on.

```rust,ignore
confound::bind! {
    "config.toml"

    [config]
    settings.*
    !settings.internal.*
    alias debug_timeout = settings.debug.timeout.length
}

fn main() {
    let timeout = config::DEBUG_TIMEOUT; // from `settings.debug.timeout.length`
}
```

Everything is resolved at compile time, so nothing reads a file when the program runs, and a pattern that stops matching is a compile error and not a missing value. `package!` and `workspace!` read a Cargo manifest without being told where it is. `cargo run --example bind_a_file` binds `examples/service.toml` with sections, globs, an exclusion and an alias, and `manifest_constants` does the same for `package!` and `workspace!`.

What a binding expands into is `tomlfuse`'s output, so every value is a `pub const` of `&'static str`, `&'static [T]`, `i64`, `f64` or `bool` in a `pub mod`, plus an `include_bytes!` binding the source file as a build input, which works in `#![no_std]` with or without an allocator. The `no_std` and `no_alloc` features only name that, and `tests/no_std.rs` builds a real `#![no_std]` crate against it. The crate itself is a proc macro and always builds with `std`.

## Status

Pre-1.0 and work in progress. Arrays have to be homogeneous, more complex globs such as `config.*.timeout` aren't covered by tests, and aliases only take singular values, tables included, and not patterns. The minimum supported rust is 1.73, and minor versions may break while patch versions don't.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/confuse/blob/main/LICENSE)
