//! Game-centric datasets such as playing card suits and ranks.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

dataset_enum! {
    /// Suits of a standard deck of cards
    pub enum Suit {
        Hearts => "Hearts",
        Clubs => "Clubs",
        Spades => "Spades",
        Diamonds => "Diamonds",
    }
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

dataset_enum! {
    /// Ranks of a standard deck of cards
    pub enum Rank {
        Ace = 1 => "Ace",
        Two = 2 => "Two",
        Three = 3 => "Three",
        Four = 4 => "Four",
        Five = 5 => "Five",
        Six = 6 => "Six",
        Seven = 7 => "Seven",
        Eight = 8 => "Eight",
        Nine = 9 => "Nine",
        Ten = 10 => "Ten",
        Jack = 11 => "Jack",
        Queen = 12 => "Queen",
        King = 13 => "King",
    }
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
    fn ordinal_reflects_rank_positions() {
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

numeric_enum! {
    /// Faces of a standard six-sided die ordered by pip count.
    #[repr(u8)]
    pub enum DiceFace {
        One = 1 => "One",
        Two = 2 => "Two",
        Three = 3 => "Three",
        Four = 4 => "Four",
        Five = 5 => "Five",
        Six = 6 => "Six",
    }
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

dataset_enum! {
    /// The six chess piece types in a fixed teaching order.
    ///
    /// Ordinals identify positions in this list, not material point values.
    #[repr(u8)]
    pub enum ChessPiece {
        Pawn = 1 => "Pawn",
        Knight = 2 => "Knight",
        Bishop = 3 => "Bishop",
        Rook = 4 => "Rook",
        Queen = 5 => "Queen",
        King = 6 => "King",
    }
}

impl ChessPiece {
    /// Returns the list position (Pawn = 1, King = 6), not a material point value.
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

/// One card in a standard 52-card deck, without jokers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Card {
    /// The card's suit.
    pub suit: Suit,
    /// The card's rank; scoring is left to the game.
    pub rank: Rank,
}

/// Returns 52 unique cards, ordered Hearts, Clubs, Spades, Diamonds, then Ace–King.
///
/// ```
/// use hodgepodge::{standard_deck, Card, Rank, Suit};
/// let deck = standard_deck();
/// assert_eq!(deck.len(), 52);
/// assert_eq!(deck[0], Card { suit: Suit::Hearts, rank: Rank::Ace });
/// ```
#[must_use]
pub fn standard_deck() -> [Card; 52] {
    const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Clubs, Suit::Spades, Suit::Diamonds];
    const RANKS: [Rank; 13] = [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];
    std::array::from_fn(|i| Card {
        suit: SUITS[i / 13],
        rank: RANKS[i % 13],
    })
}

/// Shuffles a complete deck with the caller's RNG. Consume it to deal without replacement.
#[cfg(feature = "rand")]
pub fn shuffled_deck<R: rand::Rng + ?Sized>(rng: &mut R) -> [Card; 52] {
    use rand::seq::SliceRandom;
    let mut deck = standard_deck();
    deck.shuffle(rng);
    deck
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Card> for rand::distr::StandardUniform {
    /// Samples one card uniformly, with replacement between calls.
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            suit: rng.random(),
            rank: rng.random(),
        }
    }
}

dataset_enum! {
    /// The two outcomes of an ideal coin toss.
    pub enum CoinSide { Heads => "Heads", Tails }
}

impl CoinSide {
    /// Returns the other side of the coin.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Heads => Self::Tails,
            Self::Tails => Self::Heads,
        }
    }
}

dataset_enum! {
    /// Moves in the standard three-move game.
    pub enum RockPaperScissors { Rock => "Rock", Paper => "Paper", Scissors }
}

dataset_enum! {
    /// A round's result from the current player's perspective.
    pub enum RoundOutcome { Win => "Win", Loss => "Loss", Draw }
}

impl RockPaperScissors {
    /// Returns this move's result against the opponent's move.
    #[must_use]
    pub const fn outcome_against(self, opponent: Self) -> RoundOutcome {
        match (self, opponent) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => RoundOutcome::Draw,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => RoundOutcome::Win,
            _ => RoundOutcome::Loss,
        }
    }
}

impl ChessPiece {
    /// Returns conventional 1/3/3/5/9 material points; the king has no finite value.
    #[must_use]
    pub const fn material_value(self) -> Option<u8> {
        match self {
            Self::Pawn => Some(1),
            Self::Knight | Self::Bishop => Some(3),
            Self::Rook => Some(5),
            Self::Queen => Some(9),
            Self::King => None,
        }
    }
}

impl Rank {
    /// Whether this is Jack, Queen, or King (Ace is not a face card).
    #[must_use]
    pub const fn is_face_card(self) -> bool {
        matches!(self, Self::Jack | Self::Queen | Self::King)
    }
}
