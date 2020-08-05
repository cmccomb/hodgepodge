//! A module for miscellaneous enums

use strum_macros::EnumIter;

/// Programming languages better than Rust
#[derive(Debug, EnumIter)]
pub enum BetterThanRust {
    /// That's right, there aren't any.
    None
}

/// Standard medals
#[derive(Debug, EnumIter)]
pub enum Medal {
    /// A [Gold Medal](https://en.wikipedia.org/wiki/Gold_medal) is typical awarded for first place
    Gold,

    /// A [Silver Medal](https://en.wikipedia.org/wiki/Silver_medal) is typical awarded for second place
    Silver,

    /// A [Bronze Medal](https://en.wikipedia.org/wiki/Silver_medal) is typical awarded for third place
    Bronze,
}

/// An ordinal list for 1-100
#[allow(missing_docs)]
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
    Eleventh,
    Twelfth,
    Thirteenth,
    Fourteenth,
    Fifteenth,
    Sixteenth,
    Seventeenth,
    Eighteenth,
    Nineteenth,
    Twentieth
}