//! A module for games

// Enables use as an iterable and computation of length
use strum_macros::{EnumIter, EnumCount};

/// Suits of a standard deck of cards
#[derive(Debug, EnumIter, EnumCount, Copy, Clone)]
pub enum Suit {
    Hearts,
    Clubs,
    Spades,
    Diamonds
}

#[cfg(test)]
mod test_suit {
    use crate::{*, Suit as ENUM_TO_TEST};

    const X:ENUM_TO_TEST = ENUM_TO_TEST::Clubs;
    const Y:ENUM_TO_TEST = ENUM_TO_TEST::Spades;

    #[test]
    fn accessibility() {
        println!("I like {:?}, but I also like {:?}", X, Y)
    }

    #[test]
    fn strum() {
        for x in ENUM_TO_TEST::iter() {
            println!("{:?}", x);
        }
        println!("There are {:?} variants", ENUM_TO_TEST::iter().count())
    }
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

impl Rank {
    fn value(self) -> u8 {
        let mut value = self as u8;
        if value > 10 {
            value = 10;
        }
        value
    }
}


#[cfg(test)]
mod test_rank {
    use crate::{*, Rank as ENUM_TO_TEST};

    const X:ENUM_TO_TEST = ENUM_TO_TEST::Two;
    const Y:ENUM_TO_TEST = ENUM_TO_TEST::Jack;

    #[test]
    fn accessibility() {
        println!("I like {:?}, but I also like {:?}", X, Y)
    }

    #[test]
    fn int_casting() {
        println!("{:?} are worth {:?}, but {:?} are worth {:?}", X, X.value(), Y, Y.value())
    }

    #[test]
    fn strum() {
        for x in ENUM_TO_TEST::iter() {
            println!("{:?}", x);
        }
        println!("There are {:?} variants", ENUM_TO_TEST::iter().count())
    }
}
