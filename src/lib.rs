// #![warn(clippy::all)]
// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]

//! This is a crates that just contains a bunch of enums

// Import and re-export science
pub mod science;
pub use science::*;

// Import and re-export colors
pub mod colors;
pub use colors::*;

// Import and re-export misc
pub mod misc;
pub use misc::*;

// Import and re-export geography
pub mod geography;
pub use geography::*;

// Import and re-export time
pub mod time;
pub use time::*;

// Import and re-export sports
pub mod games;
pub use games::*;

// Import and re-export strum
extern crate strum;
pub use strum::*;

// Import and re-export strum_macros
#[macro_use]
extern crate strum_macros;
pub use strum_macros::*;
