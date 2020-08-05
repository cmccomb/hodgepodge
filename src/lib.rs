#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! This is a crates that just contains a bunch of enums

// Import science
pub mod science;
use crate::science::*;

// Import colors
pub mod colors;
use crate::colors::*;

// Import misc
pub mod misc;
use crate::misc::*;

// Import geography
pub mod geography;
use crate::geography::*;

// Import time
pub mod time;
use crate::time::*;

extern crate strum;
