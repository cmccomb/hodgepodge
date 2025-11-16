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
    fn standard_deck_contains_four_suits() {
        assert_eq!(Suit::Hearts as u8, 0);
        assert_eq!(Suit::Clubs as u8, 1);
        assert_eq!(Suit::Spades as u8, 2);
        assert_eq!(Suit::Diamonds as u8, 3);
    }

    #[cfg(feature = "strum")]
    #[test]
    fn suit_count_matches_standard_deck() {
        use strum::EnumCount;

        assert_eq!(<Suit as EnumCount>::COUNT, 4);
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
    fn ordinal_reflects_blackjack_values() {
        assert_eq!(Rank::Ace.ordinal(), 1);
        assert_eq!(Rank::Ten.ordinal(), 10);
        assert_eq!(Rank::King.ordinal(), 13);
    }

    #[test]
    fn face_cards_are_higher_than_number_cards() {
        assert!(Rank::Queen.ordinal() > Rank::Ten.ordinal());
        assert!(Rank::Jack.ordinal() > Rank::Nine.ordinal());
    }

    #[cfg(feature = "strum")]
    #[test]
    fn ranks_cover_standard_deck_span() {
        use strum::EnumCount;

        assert_eq!(<Rank as EnumCount>::COUNT, 13);
    }
}
