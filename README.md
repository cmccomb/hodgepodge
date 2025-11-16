[![Build Status](https://travis-ci.org/cmccomb/hodgepodge.svg?branch=master)](https://travis-ci.org/cmccomb/hodgepodge)
[![Crates.io](https://img.shields.io/crates/v/hodgepodge.svg)](https://crates.io/crates/hodgepodge)
[![docs.rs](https://docs.rs/hodgepodge/badge.svg)](https://docs.rs/hodgepodge)
# About
The name says it all – this crate is a hodgepodge of lightweight enums you can use as ready-made datasets when prototyping, teaching, or experimenting with Rust.

# Examples
Usage is pretty simple. Import, and use to your heart's desire.
```rust
use hodgepodge::*;

fn main() {
    println!("{:?}, {:?}, and {:?} are RGB colors", RGB::Blue, RGB::Red, RGB::Green);
}
```
To use the iterator helpers shown below, enable the optional `strum` feature:

```toml
[dependencies]
hodgepodge = { version = "0.2", features = ["strum"] }
```

With that feature enabled, you can do things like this:
```rust
use hodgepodge::*;

fn main() {
    for member in Element::iter() {
        println!("{:?} is element {:?}", member.clone(), member as i32);
    }
}
```
And this:
```rust
use hodgepodge::*;

fn main() {
    println!("There are {:?} elements", Element::iter().count());
}
```

## Features

Hodgepodge keeps the default feature set empty so you only pay for what you use:

* `strum` – pulls in [`strum`](https://crates.io/crates/strum) and [`strum_macros`](https://crates.io/crates/strum_macros) to derive `EnumIter`/`EnumCount` for every dataset enum.
* `serde` – derives [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) so you can serialize the enums into fixtures for teaching materials or quick prototypes.
