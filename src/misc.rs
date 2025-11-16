//! Miscellaneous datasets such as medals, ordinals, and programming trivia.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

// Enables the optional iterator and variant-count derives.
#[cfg(feature = "strum")]
use strum_macros::{EnumCount, EnumIter};

/// Programming languages better than Rust
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BetterThanRust {
    /// That's right, there aren't any.
    None = 0,
}

#[cfg(test)]
mod better_than_rust {
    use crate::BetterThanRust as ENUM_TO_TEST;

    #[test]
    fn accessibility() {
        let better = ENUM_TO_TEST::None;
        println!("How many programming languages are better than rust {better:?}");
    }

    #[test]
    fn int_casting() {
        let better = ENUM_TO_TEST::None as i32;
        println!("How many programming languages are better than rust? {better}");
    }
}

/// Standard medals
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Medal {
    /// A [Gold Medal](https://en.wikipedia.org/wiki/Gold_medal) is typical awarded for first place
    Gold = 1,

    /// A [Silver Medal](https://en.wikipedia.org/wiki/Silver_medal) is typical awarded for second place
    Silver = 2,

    /// A [Bronze Medal](https://en.wikipedia.org/wiki/Silver_medal) is typical awarded for third place
    Bronze = 3,
}

/// An ordinal list for 1-100
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ordinal {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Sixth = 6,
    Seventh = 7,
    Eighth = 8,
    Ninth = 9,
    Tenth = 10,
    Eleventh = 11,
    Twelfth = 12,
    Thirteenth = 13,
    Fourteenth = 14,
    Fifteenth = 15,
    Sixteenth = 16,
    Seventeenth = 17,
    Eighteenth = 18,
    Nineteenth = 19,
    Twentieth = 20,
    Twentyfirst = 21,
    Twentysecond = 22,
    Twentythird = 23,
    Twentyfourth = 24,
    Twentyfifth = 25,
    Twentysixth = 26,
    Twentyseventh = 27,
    Twentyeighth = 28,
    Twentyninth = 29,
    Thirtieth = 30,
}
