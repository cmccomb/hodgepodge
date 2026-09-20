use hodgepodge::*;
use std::collections::HashSet;

#[test]
fn a_standard_deck_has_each_suit_rank_pair_once() {
    let deck = standard_deck();
    assert_eq!(deck.iter().collect::<HashSet<_>>().len(), 52);
    for &suit in Suit::ALL {
        for &rank in Rank::ALL {
            assert!(deck.contains(&Card { suit, rank }));
        }
    }
    assert_eq!(
        deck[0],
        Card {
            suit: Suit::Hearts,
            rank: Rank::Ace
        }
    );
    assert_eq!(
        deck[51],
        Card {
            suit: Suit::Diamonds,
            rank: Rank::King
        }
    );
}

#[test]
fn rock_paper_scissors_covers_all_nine_matchups() {
    use RockPaperScissors::{Paper, Rock, Scissors};
    use RoundOutcome::{Draw, Loss, Win};
    let moves = [Rock, Paper, Scissors];
    let expected = [[Draw, Loss, Win], [Win, Draw, Loss], [Loss, Win, Draw]];
    for (i, first) in moves.iter().enumerate() {
        for (j, second) in moves.iter().enumerate() {
            assert_eq!(first.outcome_against(*second), expected[i][j]);
        }
    }
    assert_eq!(CoinSide::Heads.opposite(), CoinSide::Tails);
    assert_eq!(CoinSide::Tails.opposite(), CoinSide::Heads);
}

#[test]
fn every_u8_is_checked_for_day_month_and_die_conversions() {
    for value in 0..=u8::MAX {
        let day = Day::try_from(value);
        let month = Month::try_from(value);
        let die = DiceFace::try_from(value);
        assert_eq!(day.is_ok(), (1..=7).contains(&value));
        assert_eq!(month.is_ok(), (1..=12).contains(&value));
        assert_eq!(die.is_ok(), (1..=6).contains(&value));
        if let Ok(day) = day {
            assert_eq!(day.number(), value);
        }
        if let Ok(month) = month {
            assert_eq!(month.number(), value);
        }
        if let Ok(die) = die {
            assert_eq!(die.ordinal(), value);
        }
    }
    let error = Day::try_from(0).unwrap_err();
    assert_eq!(error.enum_name(), "Day");
    assert_eq!(error.value(), 0);
    assert_eq!(error.to_string(), "0 is not a valid Day value");
    assert!(std::error::Error::source(&error).is_none());
}

#[cfg(feature = "rand")]
#[test]
fn shuffling_preserves_a_complete_deck_and_respects_the_callers_seed() {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    let mut first = ChaCha8Rng::seed_from_u64(2026);
    let mut second = ChaCha8Rng::seed_from_u64(2026);
    let deck = shuffled_deck(&mut first);
    assert_eq!(deck, shuffled_deck(&mut second));
    assert_ne!(deck, standard_deck());
    assert_eq!(
        deck.into_iter().collect::<HashSet<_>>(),
        standard_deck().into_iter().collect()
    );
    let mut faces = HashSet::new();
    for _ in 0..4096 {
        let card: Card = first.random();
        assert!(standard_deck().contains(&card));
        faces.insert(first.random::<DiceFace>());
    }
    assert_eq!(faces.len(), DiceFace::COUNT);
}

#[cfg(feature = "serde")]
#[test]
fn cards_have_a_documented_round_trip_format() {
    let card = Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    };
    let json = serde_json::to_string(&card).unwrap();
    assert_eq!(json, r#"{"suit":"Hearts","rank":"Ace"}"#);
    assert_eq!(serde_json::from_str::<Card>(&json).unwrap(), card);
    assert!(serde_json::from_str::<Card>(r#"{"suit":"Hearts","rank":"Joker"}"#).is_err());
}
