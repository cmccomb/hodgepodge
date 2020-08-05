//! A module for miscellaneous enums

use strum_macros::EnumIter;

/// Programming languages better than Rust
#[derive(Debug, EnumIter)]
pub enum BetterThanRust {
    None
}

/// An ordinal list for 1-100
#[derive(Debug, EnumIter)]
pub enum Ordinal {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
}

/// Standard medals
#[derive(Debug, EnumIter)]
pub enum Medal {
    Gold,
    Silver,
    Bronze,
}