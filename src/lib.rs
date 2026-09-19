#![warn(clippy::all)]
#![warn(missing_docs)]

//! # Hodgepodge
//!
//! `hodgepodge` bundles ready-made enums you can drop into lessons,
//! prototypes, demos, and coding exercises. Each enum doubles as a small dataset
//! (CSS color keywords, RGB swatches, the periodic table, geography trivia, game
//! pieces, etc.) so you can focus on explaining a concept instead of inventing
//! sample data.
//!
//! Bring everything into scope with `use hodgepodge::*;`, then iterate over the
//! variants (with the `strum` feature), format their values, or serialize them
//! (with the `serde` feature) depending on what the example calls for.
//!
//! ## Names, equality, and lookup
//!
//! Every enum implements `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Display`,
//! and `FromStr`. `as_str()` and `Display` return the Rust variant name.
//! Parsing ignores ASCII case but requires the complete name without whitespace.
//! These operations require no optional features or runtime dependencies.
//!
//! ```
//! use hodgepodge::{Month, CSS};
//!
//! let month: Month = "september".parse()?;
//! assert_eq!(month, Month::September);
//! assert_eq!(month.to_string(), "September");
//! assert_eq!(CSS::Aqua.rgb(), CSS::Cyan.rgb());
//! assert_ne!(CSS::Aqua, CSS::Cyan); // Different names, identical colors.
//! # Ok::<(), hodgepodge::ParseEnumError>(())
//! ```
//!
//! ## Dataset scope
//!
//! CSS colors and EU membership have reference-backed tests. Other datasets
//! include explicit teaching conventions and legacy snapshots; consult each
//! enum's documentation before treating it as a current scientific registry.
//!
//! ## Feature-gated helpers
//!
//! * `strum` – derives [`EnumIter`](https://docs.rs/strum/latest/strum/derive.EnumIter.html) and
//!   [`EnumCount`](https://docs.rs/strum/latest/strum/derive.EnumCount.html) for every dataset, re-exporting
//!   `IntoEnumIterator` and `EnumCount` so you can iterate over or count the
//!   variants without depending on `strum` directly.
//! * `enum-iter` / `enum-count` – legacy compatibility feature names that now
//!   simply forward to `strum`.
//! * `serde` – derives `serde::Serialize` and `serde::Deserialize` so the
//!   enums can be persisted in fixtures for tutorials or quick prototypes.
//!
//! ## Examples
//!
//! ### Iterate through datasets
//! ```rust
//! # #[cfg(feature = "strum")]
//! # {
//! use hodgepodge::{Element, IntoEnumIterator};
//!
//! for element in Element::iter() {
//!     let atomic_number = element as u16;
//!     println!("{element:?} is element {atomic_number}");
//! }
//! # }
//! # #[cfg(not(feature = "strum"))]
//! # {
//! # // Enable the `strum` feature to run this example.
//! # }
//! ```
//!
//! ### Format CSS color hex values
//! ```rust
//! use hodgepodge::CSS;
//!
//! let swatch = CSS::Tomato;
//! println!("{swatch:?} renders as #{swatch:06x}");
//! ```
//!
//! ### Serialize and deserialize enums
//! ```rust
//! # #[cfg(feature = "serde")]
//! # {
//! use hodgepodge::Day;
//!
//! let json = serde_json::to_string(&Day::Friday).expect("serialize Day");
//! let day: Day = serde_json::from_str(&json).expect("deserialize Day");
//! assert_eq!(day, Day::Friday);
//! # }
//! # #[cfg(not(feature = "serde"))]
//! # {
//! # // Enable the `serde` feature to run this example.
//! # }
//! ```
//!
//! ### Work with seasons and fiscal quarters
//! ```rust
//! use hodgepodge::{Quarter, Season};
//!
//! let midyear = Season::Summer;
//! let fiscal = Quarter::Q4;
//!
//! assert_eq!(midyear as u8, 3);
//! assert_eq!(fiscal as u8, 4);
//! ```

#[macro_use]
mod macros;
mod parse;
pub use parse::ParseEnumError;

// Compile the migration and README snippets along with the API examples.
#[cfg(doctest)]
#[doc = include_str!("../MIGRATION.md")]
mod migration {}

#[cfg(all(doctest, feature = "serde", feature = "strum"))]
#[doc = include_str!("../README.md")]
mod readme {}

/// Color palettes ranging from ROYGBIV to CSS keywords.
pub mod colors;
pub use colors::*;

/// Science-themed datasets such as planets and SI prefixes.
pub mod science;
pub use science::*;

/// Geographic datasets covering continents, regions, and states.
pub mod geography;
pub use geography::*;

/// Temporal datasets including months and weekdays.
pub mod time;
pub use time::*;

/// Games and leisure datasets like playing card suits and ranks.
pub mod games;
pub use games::*;

/// Miscellaneous grab-bag datasets for playful examples.
pub mod misc;
pub use misc::*;

/// Re-export helper traits from `strum` when the relevant feature is enabled.
#[cfg(feature = "strum")]
pub use strum::{EnumCount, IntoEnumIterator};
