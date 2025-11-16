//! Game-centric datasets such as playing card suits and ranks.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

// Enables the optional iterator and variant-count derives.
#[cfg(feature = "strum")]
use strum_macros::{EnumCount, EnumIter};

/// Suits of a standard deck of cards
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

#[cfg(test)]
mod test_suit {
    #[cfg(feature = "strum")]
    use crate::IntoEnumIterator;
    use crate::Suit as ENUM_TO_TEST;

    const X: ENUM_TO_TEST = ENUM_TO_TEST::Clubs;
    const Y: ENUM_TO_TEST = ENUM_TO_TEST::Spades;

    #[test]
    fn accessibility() {
        println!("I like {X:?}, but I also like {Y:?}");
    }

    #[cfg(feature = "strum")]
    #[test]
    fn strum() {
        for x in ENUM_TO_TEST::iter() {
            println!("{x:?}");
        }
        let variant_count = ENUM_TO_TEST::iter().count();
        println!("There are {variant_count} variants");
    }
}

/// Ranks of a standard deck of cards
#[derive(Debug)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

impl Rank {
    /// Returns the ordinal position of the rank (Ace = 1, ..., King = 13).
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod test_rank {
    #[cfg(feature = "strum")]
    use crate::IntoEnumIterator;
    use crate::Rank as ENUM_TO_TEST;

    const X: ENUM_TO_TEST = ENUM_TO_TEST::Two;
    const Y: ENUM_TO_TEST = ENUM_TO_TEST::Jack;

    #[test]
    fn accessibility() {
        println!("I like {X:?}, but I also like {Y:?}");
    }

    #[test]
    fn int_casting() {
        let x_value = X.ordinal();
        let y_value = Y.ordinal();
        println!("{X:?} are worth {x_value}, but {Y:?} are worth {y_value}");
    }

    #[cfg(feature = "strum")]
    #[test]
    fn strum() {
        for x in ENUM_TO_TEST::iter() {
            println!("{x:?}");
        }
        let variant_count = ENUM_TO_TEST::iter().count();
        println!("There are {variant_count} variants");
    }
}
