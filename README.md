[![CI](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml/badge.svg)](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hodgepodge.svg)](https://crates.io/crates/hodgepodge)
[![docs.rs](https://docs.rs/hodgepodge/badge.svg)](https://docs.rs/hodgepodge)

# hodgepodge

Ready-made Rust enums for lessons, prototypes, and test fixtures. Use colors,
calendar names, chemical elements, places, and game pieces to demonstrate
iteration, pattern matching, lookup, and serialization with familiar inputs.

Every dataset supports `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Display`,
`FromStr`, and an allocation-free `as_str()`. The default build has no dependencies.
Iteration and serialization are optional.

This checkout prepares **0.3.0**. See [the migration guide](MIGRATION.md) for
changes from 0.2, [the changelog](CHANGELOG.md) for release status, and
[dataset scope](DATASETS.md) for sources and teaching conventions.

## Installation

Version 0.3 requires Rust 1.71 or newer. Once 0.3 is published:

```toml
[dependencies]
hodgepodge = "0.3"
```

To use the release candidate before publication, use a local path dependency:

```toml
[dependencies]
hodgepodge = { path = "../hodgepodge" }
```

Enable `strum` for iteration and `serde` for serialization:

```toml
[dependencies]
hodgepodge = { version = "0.3", features = ["strum", "serde"] }
serde_json = "1.0"
```

## Parse names and use them as keys

`Display` and `as_str()` return the Rust variant name, such as `NorthAmerica`.
Parsing ignores ASCII case and otherwise requires that complete name. Whitespace,
spaces between words, numeric strings, and abbreviations are rejected; call
`trim()` explicitly when processing whitespace-delimited input.

```rust
use hodgepodge::{Month, ParseEnumError};
use std::collections::HashMap;

fn main() -> Result<(), ParseEnumError> {
    let month: Month = "september".parse()?;
    let visits = HashMap::from([(month, 3)]);
    assert_eq!(visits[&Month::September], 3);
    println!("{month}: {} visits", visits[&month]);
    Ok(())
}
```

## Work with exact color values

Use `rgb()` to obtain a packed `0xRRGGBB` value from any color enum. Distinct CSS
names can represent the same RGB value. Hex formatting produces six lowercase
digits by default; `#` adds the Rust `0x` prefix.

```rust
use hodgepodge::CSS;

fn main() {
    assert_ne!(CSS::Aqua, CSS::Cyan);
    assert_eq!(CSS::Aqua.rgb(), CSS::Cyan.rgb());
    assert_eq!(CSS::Aqua.rgb(), 0x00ffff);
    println!("{}: #{:x}", CSS::Aqua, CSS::Aqua);
}
```

Color enum discriminants are identifiers. Replace color-to-integer casts with
`rgb()` when upgrading from 0.2.

## Iterate through a dataset

With the `strum` feature, the helper traits are re-exported by `hodgepodge`:

```rust
use hodgepodge::{Element, EnumCount, IntoEnumIterator};

fn main() {
    assert_eq!(Element::COUNT, 118);
    for element in Element::iter() {
        println!("{element} has atomic number {}", element as u16);
    }
}
```

## Save and recover fixtures

With the `serde` feature, enums serialize to their exact variant names. Serde
deserialization is case-sensitive even though `FromStr` ignores ASCII case.

```rust
use hodgepodge::DiceFace;

fn main() -> Result<(), serde_json::Error> {
    let rolls = vec![DiceFace::One, DiceFace::Six];
    let json = serde_json::to_string(&rolls)?;
    let recovered: Vec<DiceFace> = serde_json::from_str(&json)?;
    assert_eq!(recovered, rolls);
    Ok(())
}
```

## Features

| Feature | Effect |
| --- | --- |
| Default | Common traits, names, parsing, and color conversions; no dependencies |
| `strum` | `EnumIter`, `EnumCount`, and re-exported helper traits |
| `enum-iter`, `enum-count` | Compatibility names that each enable `strum` |
| `serde` | `Serialize` and `Deserialize` on every dataset |

## Complete teaching examples

Run these from the checkout:

```sh
cargo run --example lookup
cargo run --example palette --features strum
cargo run --example fixtures --features serde
```

- `lookup` parses names and counts visits using enum map keys.
- `palette` groups the 148 CSS names by exact RGB value, exposing aliases.
- `fixtures` saves and restores deterministic dice rolls and color names as JSON.

## Development

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic
bash scripts/test-features.sh
RUSTUP_TOOLCHAIN=1.71.0 bash scripts/test-features.sh
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo package --all-features
```

The test script covers standalone and combined features, compatibility feature
names, every example target, and documentation tests. Reference fixtures are
checked in; tests do not fetch external data.

Licensed under MIT OR Apache-2.0.
