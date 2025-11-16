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
//! through the variants (with the `strum` feature), serialize them (with the
//! `serde` feature), or format them however your demo requires.
//!
//! ## Feature-gated helpers
//!
//! * `strum` – derives [`strum_macros::EnumIter`] and
//!   [`strum_macros::EnumCount`] for every dataset, re-exporting
//!   [`IntoEnumIterator`] and [`EnumCount`] so you can iterate over the variants
//!   or count them without depending on `strum` directly.
//! * `enum-iter` / `enum-count` – legacy compatibility feature names that now
//!   simply forward to `strum`.
//! * `serde` – derives [`serde::Serialize`] and [`serde::Deserialize`] to help
//!   you capture the datasets in fixtures.
//!
//! ## Examples
//!
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
//! ```rust
//! # #[cfg(feature = "serde")]
//! # {
//! use hodgepodge::Day;
//!
//! let json = serde_json::to_string(&Day::Friday).expect("serialize Day");
//! assert_eq!(json, "\"Friday\"");
//! # }
//! # #[cfg(not(feature = "serde"))]
//! # {
//! # // Enable the `serde` feature to run this example.
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
#[cfg(feature = "strum")]
pub use strum::{EnumCount, IntoEnumIterator};
