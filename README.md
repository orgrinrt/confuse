# `confuse`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/confuse.svg)](https://github.com/orgrinrt/confuse/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/confuse)](https://crates.io/crates/confuse)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/confuse.svg)](https://github.com/orgrinrt/confuse/issues)
[![Latest Version](https://img.shields.io/badge/version-0.0.4-red.svg?label=latest)](https://github.com/orgrinrt/confuse)
![Crates.io Version](https://img.shields.io/crates/v/confuse?logoSize=auto&color=%23FDC700&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fconfuse)
![Crates.io Size](https://img.shields.io/crates/size/confuse?color=%23C27AFF&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fconfuse)
![GitHub last commit](https://img.shields.io/github/last-commit/orgrinrt/confuse?color=%23009689&link=https%3A%2F%2Fgithub.com%2Forgrinrt%2Fconfuse)

> Easily bind structured data from various file formats into schemas and properly typed build-time constants with flexible patterns.

> # ⚠️WIP⚠️
> ### Until release, consider [tomlfuse]("https://crates.io/crates/tomlfuse") version `0.0.3` for toml file binding.
> *version `0.0.4` deprecates the crate, but `0.0.3` will still be available

</div>


## Features

- Compile-time binding of structured data from a file to rust constants
- Schema generation with proper typing and inferred default values
- Flexibly preserve table hierarchies as nested modules
- Glob pattern support for selecting what to bind and what not to
    - Supports negated patterns for exclusion (`!` prefix)
- Alias support for renaming paths (`alias foo = bar.baz`)
- Preserves comments from source file into rustdoc comments in the generated code
- Infers and statically types common primitives, including *arrays*
    - *tables* translate to rust modules by default for cleaner const-time codegen, but can optionally be generated as structs with minimal, but still some, dynamic dispatch

## Supported formats
| Feature | Format | Status     | Default</br>Extensions | Default</br>Feature | Notes                                                             |
|---------|--------|------------|------------------------|---------------------|-------------------------------------------------------------------|
| `toml`  | toml   | 🚧 wip     | .toml                  | ✅                   | Special keywords for cargo manifests:</br>`crate` and `workspace` |
| `json`  | json   | 📝 planned | .json                  | ✅                   |                                                                   |
| `yaml`  | yaml   | 📝 planned | .yaml                  | ✅                   |                                                                   |
| `ron`   | ron    | 📝 planned | .ron                   | ❌                   |                                                                   |

See [Custom formats](#custom-formats) for more information on how to add support for custom formats.


## Usage: `bind!` macro
### Basic example

```rust
use confuse::bind;
use std::thread;
use std::time::Duration;

bind! {
    "path/to/config.toml"
    
    [config]
    settings.*
    !settings.defaults.*
    !settings.internal.*
    alias debug_timeout = settings.debug.timeout.length
    
    [defaults]
    project.defaults.*
    settings.defaults.*
}

fn main() {
    let timeout = config::DEBUG_TIMEOUT; //      from source's `settings.debug.timeout.length`
    let err_msg = config::debug::ERR_MESSAGE; // from source's `settings.debug.timeout.err_message`
    let hello = defaults::HELLO; //              from source's `project.defaults.hello.msg`
    for p in defaults::PEOPLE { //               from source's `project.defaults.people`
        if p.is_empty() {
            eprintln!(err_msg);
            continue;
        }
        println!("{} says {}", p, hello);
        thread::sleep(Duration::from_millis(timeout));
    }
}
```

<details>
<summary>Click to expand detailed usage</summary>

### Context and setup
#### Source definition
```rust
confuse::bind! {
    // source is defined either with explicit name or implicit one:
    source foo = "bar/baz.toml"
    // the implicit name is the file name without the extension
    "fizz/buzz.json" //    <=> `source buzz = "fizz/buzz.json"`
    
    // if the entire binding invocation only uses a single source file, the naming
    // can entirely be omitted for brevity and convenience! see binding rules for examples.
    
    // by default, the format is inferred from the extension
    // so there is, in most cases, no need to specify the format
    // e.g this is valid:
    "path/to/config.json" 
    
    // however, if you want to use an unconventional or completely
    // custom extensions, you can specify the format like this:
    "path/to/project.config" as json
    
    // you can also use a custom parser like this:
    "path/to/config.toml" as MyCustomParser
    
    // for certain formats, there are also built-in keywords for certain files
    // e.g. for toml, `crate` and `workspace` are valid and resolve to the
    // current crate's Cargo.toml and the workspace's Cargo.toml respectively, e.g:
    
    crate as toml //     <-- can resolve to something like: `$CARGO_MANIFEST_DIR/Cargo.toml`
    workspace as toml // <-- can resolve to something like: `$CARGO_MANIFEST_DIR/../../Cargo.toml`
    
    // do also note that using the `as` keyword like above is unnecessary,
    // since these special keywords only exist in toml context. This is equally valid:
    crate // <-- this is valid because the toml parser is implicitly tied by convention
    
    // the semantic `as` keyword is syntactic sugar that is
    // equivalent to the explicitly named source type notation:
    
    source workspace: toml = workspace //    <=>  `workspace as toml`  <=>  `workspace`
    source foo: MyCustomYaml = "foo.yml" //  <=>  `"foo.yml" as MyCustomYaml`
}
```
#### Advanced configuration
```rust
confuse::bind!{
    todo!()
}
```
### Bindings
#### Basics: Module declarations (sections)
```rust
confuse::bind! {
    // there can be any amount of bindings in a single invocation, with
    // any amount of source files, and any amount of patterns.
    source foo = "foo.json" as json
    source bar = "bar.yml" as yaml

    // each binding starts with the module declaration and is followed
    // by a list of patterns, which are separated by a newline.

    [settings] // <-- will generate a rust module named `settings`
    config.*           // = include all config.* fields
    !config.internal.* // = ...but exclude internals!

    // there is optionally a syntax for attributes:
    [settings, {
        // an attribute is a key-value pair separated by a comma:
        source = foo,
        // the value can also be a fn call, as long as it produces a valid value:
        other_attr = some_ctor(arg1, arg2),
        // for flag-like attributes, you can omit the value:
        argless_attr,
    }]
    // there's also a short notation that's valid as long as there are no line breaks:
    [settings(source = foo, some_flag)]
    // the attributes are useful in that they can be used to 
    // specify instructions for the codegen, such as the source file.

    // attributes defined at the section level will apply implicitly to
    // all the patterns/rules in the section, but can be overridden individually:
    [settings(source = foo)]
    config.* //                      <-- this will use `foo` as the source
    #[source = bar] debug.consts.*// <-- this will instead use `bar` as source

    // individual attributes can be freely used on any pattern/rule.
    // the attribute has to precede the pattern/rule, but is valid both
    // with a newline, and without:
    #[some_flag]
    fizz.buzz.* // <-- this is as valid as `#[some_flag] fizz.buzz.*`

    // and for convenience, there is a combinator syntax for attributes
    #[source = bar, some_flag, another_flag]
    hello.*
    // this is also valid for section-level attributes, so this is a valid section declaration too:
    #[source = bar, some_flag]
    [settings]
}
```
#### Basics: Patterns
```rust
confuse::bind! {
    // all patterns have to be inside a section declarations, i.e needs a preceding
    // `[section]` declaration, and are separated by newlines.
    [some_section]
    some.rule.*
    !some.rule._private.*
    // the patterns are glob-like, and can match as many fields as you'd like.
    // the resulting codegen will default to generating constants if no explicit
    // resolution mode is set.
    
    // you can explicitly set the resolution mode to one of:
    // - `lazy`: will be resolved at runtime, once, on first access
    // - `static`: will be resolved at compile time, but inlining left to compiler
    // - `const`: will be resolved at compile time and inlined always
    static config.foo.* //  <-- all the matches will resolve at compile time, but may not inline
    lazy config.foo.buzz // <-- valid for indidivudal fields too.
    const config.bar.baz // <-- this will resolve at compile time and always inline.
    // ^ this is the default behaviour implicitly used if no resolution mode is set.
    //   this also means you can also just omit explicitly writing it for brevity and convenience.
    
    // these resolution modes can be mixed and matched with the patterns, and
    // you can also simply override individual fields with them, even if they are
    // already brought in by a pattern, which is done by default.
    // if the field is not yet matched by a pattern, it will be implicitly included
    // with this definition notation.
    
    // negation patterns are special in that they, naturally, do not allow for
    // resolution modes, since they are, well, excluded.
    lazy !config._private.* // <-- this is invalid and will not compile
    
}
```
#### Special binding: Aliases
```rust
confuse::bind! {
    [dbg]
    // you can create aliases for example to solve naming conflicts.
    // note that aliases are intended for singular fields (including tables)
    // so they should not contain glob patterns:
    alias timeout = config.params.timeout
}
```
#### Special binding: Environment variables
```rust
confuse::bind! {
    [foo]
    // the `env` keyword allows you to bind environment variables
    // that resolve, by default, lazily at runtime:
    env FOO_TARGET_DIR
    // optionally you can also set default values to use if the env var is not set:
    env FOO_WORKING_DIR = "path/to/working/dir"
    
    // all special bindings can be mixed in with other rules, so this is valid continuation:
    some.path.*
    !some.path._private.*
}
```

### Format macros
For the supported formats, you can use the respectively named macros to bind files witih less verbosity in the macro input, for example:
```rust
use confuse::toml;
toml! {
    "path/to/config.toml"
    ...
}
toml! {
    crate // <-- this is valid because the toml parser is implicit
    ...
}
toml! {
    workspace // <-- same thing here
    ...
}
// this same applies to other supported and enabled formats and their special keywords
```
#### The main difference vs. the main `bind!` macro:
1. The format is statically bound to the macro, so the parser is implicit and need not be specified
    - Custom parsers can however still be used with the usual syntax
2. This skips some infer checks and other processing due to the statically bound format, so it is a bit faster to compile

</details>

## Usage: `#[fuse]` attribute
### Basic example: Fusing to a module
```rust
use confuse::confuse;

// the very same concepts as the `bind!` macro, but the resulting code is 
// "fused" into the item the attribute is attached to. 

// the biggest difference is that this macro supports root-level patterns, 
// instead of all of the patterns needing to be inside a section declaration.

// actual section declarations within this macro will produce submodules when
// the macro is attached to a module item, but in case of structs or enums, 
// declaring sections will result in compile errors (for now).

// as such, for modules, this works pretty much exactly the same as the macro:
#[fuse(
    "path/to/config.toml"
    
    [some_submod]
    config.*
    !config.internal.*
    
    [other_submod]
    alias debug_timeout = config.debug.timeout.length
    env FOO_TARGET_DIR
    env FOO_WORKING_DIR = "path/to/working/dir"
)]
mod settings {
    // this can be empty, but can also include whatever other items and definitions you want.
}
```
### Basic example: Fusing to a struct
```rust
// for structs, we can only define root-level patterns, aliases and other definitions:
#[fuse(
    "models/base.person" as toml
    const physicals.max_height
    const physicals.max_weight
    const alias maximum_age = physicals.max_age
    lazy alias first_name = name.first
    lazy alias last_name = name.last
)]
struct Person {
    // the struct can just be empty if only used for fusing
    // however, it can be any struct, even a fully defined one with prior use:
    name: String,
    favourite_animal: Box<dyn Animal>,
}
// this would generate something similar to:
// impl Person {
//     pub const MAX_HEIGHT: u32 = 200;
//     pub const MAX_WEIGHT: u32 = 150;
//     pub const MAXIMUM_AGE: u32 = 100;
//     pub static FIRST_NAME: Lazy<String> = Lazy::new(|| {
//         confuse::__read_config_value!("name.first")
//     }
//     pub static LAST_NAME: Lazy<String> = Lazy::new(|| {
//         confuse::__read_config_value!("name.last")
//     }
// }
```

## Limitations and future work

### Value types and patterns

- Presently only supports homogenous arrays (e.g. `["a", "b", "c"]`), not heterogeneous (e.g. `[1, "a", 3.14]`)
<details>
<summary>*Click to expand notes*</summary>

    - This is planned for the future
        - Initially by converting each element to a string representation and generating an array of strings in its stead (not ideal, but leaves the door open for consumer-side implementations for this)
        - Later down the line, as an optional alternative, by translating the array to an array of option tuples by merging the unique types of all the elements in the array as options wherein each
          `Some` value represents the element, and writing some convenience traits around the concept to get the values out of the array in a type-safe but "natural" way, while remaining build-time constant and avoiding dynamic dispatch
            - A tradeoff between runtime performance and binary size and compilation time, essentially,
              *if* someone truly needs this
    - However, I'm not sure this is a common enough use-case to make a priority right now, I would be interested to hear any use cases that would require this though
</details>

- As of right now, more complex globs are not covered in tests (e.g.
  `config.*.timeout`), and may or may not work in different cases
<details>
<summary>*Click to expand notes*</summary>

    - These tests and possibly some refactoring for increased robustness are however being implemented in very near future as it is fundamental to the concept to handle these
    - The most common use case would be the patterns supported right now, so this crate releases initially with just them stabilized
</details>

- Glob syntax for collections, i.e `{a|b|c}`, or other more involved patterns is not supported yet either
<details>
<summary>*Click to expand notes*</summary>

    - This is something that would be preferable to support, but also not a priority right now, since the use case of toml file binding feels to me like something that would not often warrant the use of this kind of complexity
</details>

- Aliasing currently only supports singular values (including tables), but not batches (i.e pattern aliases)
<details>
<summary>*Click to expand notes*</summary>

    - In future there will be support for simple batch aliasing by using the source path's segment that matches a star to place into the alias pattern's same index star
        - This will however have some constraints that make it less useful than I'd ultimately want it to be, like:
            - This would only work with patterns that contain nothing but glob stars (however the amount of those could be any)
            - If there are multiple stars, then both sides of the alias assignment must match the same amount of stars, otherwise it won't work, which may or may not be obvious and would probably be confusing to the user
    - In the long run, it'd be great to find a more robust solution, but this would be entirely outside this crate's scope, so it would be an integration of another crate that does this ultimately.
        - I would be interested to hear suggestions in the meanwhile
</details> 

### Extended features

- While constant time binding is the most useful case for something like this, it is not the only one, and I would like to explore the possibility of allowing for dynamic binding as well with some static safety measures such as creating a schematic based on a toml file for type-safe binding, and allowing sane statically typed instances of the toml file to be created and mutated at runtime with minimal, preferably zero dynamic dispatch overhead
- While this crate is named `confuse`, it could just as well be abstracted away and made implementable for any file format
<details>
<summary>*Click to expand notes*</summary>

    - It will be great to be able to confuse people outside of toml alone
        - However, I hate that making this more generic kills the perfect opportunity to adapt this concept to ron... as
          `ronfuse`...
            - but I digress
</details>


## Compatibility

This crate requires rust `1.73.0` or later. With present dependencies, this is the minimum supported version the dependencies allow. Bumping msrv is considered a breaking change and will be done in a minor version.

### Versioning policy

Minor versions may have breaking changes, which can include bumping msrv.

Patch versions are backwards compatible.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/confuse/blob/master/LICENSE)
