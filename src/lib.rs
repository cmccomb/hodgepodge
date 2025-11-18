#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

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
//! ## Feature-gated helpers
//!
//! * `strum` – derives [`strum_macros::EnumIter`] and
//!   [`strum_macros::EnumCount`] for every dataset, re-exporting
//!   [`IntoEnumIterator`] and [`EnumCount`] so you can iterate over or count the
//!   variants without depending on `strum` directly.
//! * `enum-iter` / `enum-count` – legacy compatibility feature names that now
//!   simply forward to `strum`.
//! * `serde` – derives [`serde::Serialize`] and [`serde::Deserialize`] so the
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
