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
    use crate::Suit;

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
    use crate::Rank;

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

/// Faces of a standard six-sided die ordered by pip count.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DiceFace {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl DiceFace {
    /// Returns the pip value of the die face (1–6).
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod test_dice_face {
    use crate::DiceFace;

    #[test]
    fn pip_values_match_die_faces() {
        assert_eq!(DiceFace::One.ordinal(), 1);
        assert_eq!(DiceFace::Three.ordinal(), 3);
        assert_eq!(DiceFace::Six.ordinal(), 6);
    }

    #[cfg(feature = "strum")]
    #[test]
    fn dice_face_count_matches_standard_die() {
        use strum::EnumCount;

        assert_eq!(<DiceFace as EnumCount>::COUNT, 6);
    }
}

/// Pieces in a standard game of chess, ordered by increasing relative value.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChessPiece {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl ChessPiece {
    /// Returns an ordinal representing the piece's relative value (Pawn = 1, King = 6).
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod test_chess_piece {
    use crate::ChessPiece;

    #[test]
    fn ordinals_reflect_piece_hierarchy() {
        assert!(ChessPiece::Queen.ordinal() > ChessPiece::Rook.ordinal());
        assert!(ChessPiece::King.ordinal() > ChessPiece::Queen.ordinal());
        assert_eq!(ChessPiece::Pawn.ordinal(), 1);
    }

    #[cfg(feature = "strum")]
    #[test]
    fn chess_piece_count_matches_standard_set() {
        use strum::EnumCount;

        assert_eq!(<ChessPiece as EnumCount>::COUNT, 6);
    }
}
