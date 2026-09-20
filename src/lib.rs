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
//! variants through `ALL` (or with the `strum` feature), format their values, or serialize them
//! (with the `serde` feature) depending on what the example calls for.
//!
//! ## Names, equality, and lookup
//!
//! Every enum dataset implements `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Display`,
//! and `FromStr`. `as_str()` and `Display` return the Rust variant name.
//! Parsing ignores ASCII case but requires the complete name without whitespace.
//! These operations require no optional features or runtime dependencies.
//! Every enum dataset also exposes `ALL`, `COUNT`, and a human-readable `label()`,
//! with generic access through [`Dataset`].
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
//! Reference fixtures cover chemistry, geography, anatomy, ecology, and geologic
//! time. Bone counts use the conventional 206-bone adult skeleton; muscle names
//! are a teaching selection. Consult each enum's documentation for its scope.
//!
//! ## Feature-gated helpers
//!
//! * `strum` – enables iteration and variant counts for every dataset, re-exporting
//!   `IntoEnumIterator` and `EnumCount` so you can use these traits without
//!   depending on `strum` directly.
//! * `enum-iter` / `enum-count` – legacy compatibility feature names that now
//!   simply forward to `strum`.
//! * `serde` – enables `serde::Serialize` and `serde::Deserialize` so the
//!   enums and cards can be persisted in fixtures for tutorials or quick prototypes.
//! * `rand` – uniform variant sampling and card shuffling with a caller-supplied
//!   rand 0.9 RNG. Samples use replacement; consume a shuffled deck to deal unique cards.
//!
//! * `taxonomy` – a shared `Species` enum with hierarchical aliases, backed by
//!   source records, ancestry, intermediate ranks, and Wikipedia links; see the
//!   `taxonomy` module when enabled.
//!   The data carries CC BY 4.0 attribution in addition to the software license.
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
//!     let atomic_number = element.atomic_number();
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
pub mod metadata;
mod value;
pub use metadata::{DatasetCoverage, DatasetInfo, DatasetSource, DATASETS};
pub use value::EnumValueError;

/// Metadata shared by every enum dataset, without optional features.
///
/// `ALL` follows declaration order. Labels are presentation text; use `as_str()`
/// for canonical names accepted by `FromStr` and used by JSON serialization.
/// Other Serde formats may encode enum indexes; they are not stable storage IDs.
pub trait Dataset: Copy + 'static {
    /// The dataset scope, source references, coverage and representation license.
    const INFO: DatasetInfo;
    /// Every variant in declaration order.
    const ALL: &'static [Self];
    /// The number of variants.
    const COUNT: usize;
    /// The canonical Rust variant name.
    fn as_str(self) -> &'static str;
    /// A human-readable label.
    fn label(self) -> &'static str;
}

// Compile the migration and README snippets along with the API examples.
#[cfg(doctest)]
#[doc = include_str!("../MIGRATION.md")]
mod migration {}

#[cfg(all(doctest, feature = "serde", feature = "strum"))]
#[doc = include_str!("../README.md")]
mod readme {}

#[cfg(all(doctest, feature = "serde", feature = "strum"))]
#[doc = include_str!("../GUIDE.md")]
mod guide {}

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

/// DNA/RNA bases, standard-code codons, and the twenty standard amino acids.
pub mod biology;
pub use biology::*;

/// Individual adult bones and selected skeletal muscle types.
pub mod anatomy;
pub use anatomy::*;

/// Terrestrial biome categories.
pub mod ecology;
pub use ecology::*;

/// Rocks, Earth layers, and the hierarchy of geologic time.
pub mod geology;
pub use geology::*;

/// Literary datasets such as Dante's circles of Hell.
pub mod literature;
pub use literature::*;

/// A sourced animal taxonomy with parent/child traversal and Wikipedia links.
#[cfg(feature = "taxonomy")]
pub mod taxonomy;

/// Re-export helper traits from `strum` when the relevant feature is enabled.
#[cfg(feature = "strum")]
pub use strum::{EnumCount, IntoEnumIterator};
