[![CI](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml/badge.svg)](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hodgepodge.svg)](https://crates.io/crates/hodgepodge)
[![docs.rs](https://docs.rs/hodgepodge/badge.svg)](https://docs.rs/hodgepodge)
# About
The name says it all – this crate is a hodgepodge of lightweight enums you can use as ready-made datasets when prototyping, teaching, or experimenting with Rust.

# Examples
Usage is pretty simple. Import, and use to your heart's desire.
```rust
use hodgepodge::RGB;

fn main() {
    let blue = RGB::Blue;
    let red = RGB::Red;
    let green = RGB::Green;
    println!("{blue:?}, {red:?}, and {green:?} are RGB colors");
}
```
To use the iterator helpers shown below, enable the optional `enum-iter` feature:

```toml
[dependencies]
hodgepodge = { version = "0.2", features = ["enum-iter"] }
```

With that feature enabled, you can do things like this:
```rust
use hodgepodge::Element;
use hodgepodge::IntoEnumIterator;

fn main() {
    for member in Element::iter() {
        let atomic_number = member as i32;
        println!("{member:?} is element {atomic_number}");
    }
}
```
And this:
```rust
use hodgepodge::Element;
use hodgepodge::IntoEnumIterator;

fn main() {
    let element_count = Element::iter().count();
    println!("There are {element_count} elements");
}
```

## Features

Hodgepodge keeps the default feature set empty so you only pay for what you use:

* `enum-iter` – pulls in [`strum`](https://crates.io/crates/strum) and [`strum_macros`](https://crates.io/crates/strum_macros) to derive `EnumIter` and re-export [`IntoEnumIterator`](https://docs.rs/strum/latest/strum/iter/trait.IntoEnumIterator.html) for each dataset.
* `enum-count` – derives [`EnumCount`](https://docs.rs/strum/latest/strum/enum_count/trait.EnumCount.html) for every dataset and re-exports the trait so you can introspect totals without depending on `strum` directly.
* `serde` – derives [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) so you can serialize the enums into fixtures for teaching materials or quick prototypes.

## Development

This repository keeps contributor workflows aligned with CI. Please run the same commands locally before pushing:

1. `cargo fmt --all --check`
2. `cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic`
3. `cargo test --all-targets`
4. `cargo test --all-targets --features "serde enum-iter enum-count"`
