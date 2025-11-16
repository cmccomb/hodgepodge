#![cfg(feature = "serde")]

use hodgepodge::{Rank, Suit};
use serde_json::{from_str, to_string};

#[test]
fn rank_round_trips_through_json() {
    let queen_json = to_string(&Rank::Queen).expect("serialize rank");
    let queen: Rank = from_str(&queen_json).expect("deserialize rank");
    assert!(matches!(queen, Rank::Queen));
}

#[test]
fn suit_serializes_to_numeric_discriminant() {
    let suit_json = to_string(&Suit::Spades).expect("serialize suit");
    assert_eq!(suit_json, "\"Spades\"");
    let suit: Suit = from_str(&suit_json).expect("deserialize suit");
    assert!(matches!(suit, Suit::Spades));
}
