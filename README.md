[![CI](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml/badge.svg)](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hodgepodge.svg)](https://crates.io/crates/hodgepodge)
[![docs.rs](https://docs.rs/hodgepodge/badge.svg)](https://docs.rs/hodgepodge)

# About hodgepodge
`hodgepodge` is a grab bag of ready-made enums you can drop into lessons, prototypes, demos, and coding exercises. Each enum doubles as a tiny dataset—covering CSS color keywords, RGB swatches, the periodic table, continents, SI prefixes, solar-system trivia, decks of cards, and more—so you can focus on teaching a concept instead of inventing boilerplate data.

The crate shines when you need to illustrate iteration, formatting, serialization, or pattern matching without stopping to build sample inputs.

## Install
Add `hodgepodge` to your `Cargo.toml` using the latest published version:

```toml
[dependencies]
hodgepodge = "0.2"
```

Enable optional helpers (such as iteration or serialization) by listing the relevant Cargo features:

```toml
[dependencies]
hodgepodge = { version = "0.2", features = ["strum", "serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage
### Iterate through datasets
Enable the `strum` feature to derive `EnumIter`/`EnumCount` for each dataset and re-export [`IntoEnumIterator`](https://docs.rs/strum/latest/strum/iter/trait.IntoEnumIterator.html). That makes it easy to loop through everything, such as the periodic table:

```rust
use hodgepodge::{Element, IntoEnumIterator};

fn main() {
    for element in Element::iter() {
        let atomic_number = element as u16;
        println!("{element:?} is element {atomic_number}");
    }
}
```

### Format CSS colors
Enums such as `CSS` implement `LowerHex`, so you can turn a variant into its hexadecimal color code without extra helpers:

```rust
use hodgepodge::CSS;

fn main() {
    let swatch = CSS::Tomato;
    println!("{swatch:?} renders as #{swatch:06x}");
}
```

### Serialize and deserialize with `serde`
All enums derive `serde::Serialize` and `serde::Deserialize` when the `serde` feature is active, so shipping fixtures for tutorials is a one-liner:

```rust
use hodgepodge::Day;

fn main() -> Result<(), serde_json::Error> {
    let json = serde_json::to_string(&Day::Saturday)?;
    let day: Day = serde_json::from_str(&json)?;
    assert_eq!(day, Day::Saturday);
    Ok(())
}
```

## Features
Feature | Default | Description
--- | --- | ---
`strum` | Disabled | Derives [`EnumIter`](https://docs.rs/strum/latest/strum/trait.EnumIter.html) and [`EnumCount`](https://docs.rs/strum/latest/strum/enum_count/trait.EnumCount.html) for every dataset and re-exports the helper traits so you can iterate without depending on `strum` directly.
`enum-iter`, `enum-count` | Disabled | Legacy compatibility feature names that simply forward to `strum`.
`serde` | Disabled | Adds `serde::Serialize` and `serde::Deserialize` to every enum so they can be written to JSON, TOML, etc.

Enable any combination of these features with `cargo` flags:

```shell
cargo add hodgepodge --features "strum serde"
# or
cargo test --features "strum"
```

## Use cases
* **Education:** quickly demonstrate iteration, pattern matching, or formatting with realistic data.
* **Prototyping:** plug in enums representing colors, locations, or science mnemonics without maintaining bespoke fixtures.
* **Testing:** serialize enums with `serde` to build deterministic fixtures for integration tests.

## Development
1. `cargo fmt --all --check`
2. `cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic`
3. `cargo test --all-targets`
4. `cargo test --all-targets --features "serde strum"`
