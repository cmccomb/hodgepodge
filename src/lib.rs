#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! # Hodgepodge
//!
//! `hodgepodge` is a grab bag of enums that double as instant datasets for
//! lessons, prototypes, and examples. The datasets cover topics like CSS and RGB
//! colors, geography (continents, states, and EU countries), timekeeping (months
//! and weekdays), common science mnemonics (the periodic table, SI units, solar
//! system), game pieces, and even whimsical trivia.
//!
//! The intended workflow is to add `use hodgepodge::*;` at the top of your file
//! so all of the enums are in scope. From there you can immediately iterate
//! through the variants, serialize them (with the `serde` feature), or format
//! them however your demo requires.
//!
//! ## Feature-gated helpers
//!
//! * `enum-iter` – derives [`strum_macros::EnumIter`] for every dataset and
//!   re-exports [`IntoEnumIterator`] so you can write
//!   `use hodgepodge::IntoEnumIterator;` without pulling `strum` directly.
//! * `enum-count` – derives [`strum_macros::EnumCount`] across the crate and
//!   re-exports [`EnumCount`] so you can introspect the total number of variants
//!   in a dataset.
//! * `serde` – derives [`serde::Serialize`] and [`serde::Deserialize`] to help
//!   you capture the datasets in fixtures.
//!
//! ## Examples
//!
//! ```rust
//! # #[cfg(feature = "enum-iter")]
//! # {
//! use hodgepodge::{Element, IntoEnumIterator};
//!
//! for element in Element::iter() {
//!     let atomic_number = element as u16;
//!     println!("{element:?} is element {atomic_number}");
//! }
//! # }
//! # #[cfg(not(feature = "enum-iter"))]
//! # {
//! # // Enable the `enum-iter` feature to run this example.
//! # }
//! ```

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
#[cfg(feature = "enum-count")]
pub use strum::EnumCount;
/// Re-export helper traits from `strum` when the relevant feature is enabled.
#[cfg(feature = "enum-iter")]
pub use strum::IntoEnumIterator;
