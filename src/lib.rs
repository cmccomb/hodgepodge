#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This is a crates that just contains a bunch of enums

// Import science
pub mod science;
pub use science::*;

// Import colors
pub mod colors;
pub use colors::*;

// Import misc
pub mod misc;
pub use misc::*;

// Import geography
pub mod geography;
pub use geography::*;

// Import time
pub mod time;
pub use time::*;

// Import sports
pub mod games;
pub use games::*;

extern crate strum;
pub use strum::*;
#[macro_use]
extern crate strum_macros;
pub use strum_macros::*;
