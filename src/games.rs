//! A module for games

// Enables use as an iterable and computation of length
use strum_macros::{EnumIter, EnumCount};

/// Suits of a standard deck of cards
#[derive(Debug, EnumIter, EnumCount)]
pub enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds
}

/// Ranks of a standard deck of cards
#[derive(Debug, EnumIter, EnumCount)]
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
    King = 13
}

