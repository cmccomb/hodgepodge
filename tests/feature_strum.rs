#![cfg(feature = "strum")]

use hodgepodge::{Element, Planet, CSS};
use hodgepodge::{EnumCount, IntoEnumIterator};

#[test]
fn css_keyword_count_matches_reference_list() {
    assert_eq!(<CSS as EnumCount>::COUNT, 148);
}

#[test]
fn elements_cover_full_periodic_table() {
    assert_eq!(<Element as EnumCount>::COUNT, 118);
}

#[test]
fn iterating_planets_yields_expected_order() {
    let collected: Vec<u8> = Planet::iter().map(|planet| planet as u8).collect();
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(collected, expected);
}
