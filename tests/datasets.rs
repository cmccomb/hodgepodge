use hodgepodge::{Element, Planet, States, CSS, ROYGBIV};

#[test]
fn elements_use_atomic_numbers_as_discriminants() {
    assert_eq!(Element::Hydrogen as u16, 1);
    assert_eq!(Element::Carbon as u16, 6);
    assert_eq!(Element::Oganesson as u16, 118);
}

#[test]
fn planets_follow_orbital_order() {
    assert_eq!(Planet::Mercury as u8, 1);
    assert_eq!(Planet::Earth as u8, 3);
    assert_eq!(Planet::Pluto as u8, 9);
    assert!(Planet::Venus as u8 > Planet::Mercury as u8);
}

#[test]
fn roygbiv_hex_values_are_consistent() {
    assert_eq!(ROYGBIV::Red as i32, 0x00ff_0000);
    assert_eq!(ROYGBIV::Indigo as i32, 0x004b_0082);
    assert_eq!(ROYGBIV::Violet as i32, 0x0094_00d3);
}

#[test]
fn css_colors_match_documented_hex_codes() {
    assert_eq!(CSS::Tomato as i32, 0x00ff_6347);
    assert_eq!(CSS::RebeccaPurple as i32, 0x0066_3399);
    assert_eq!(CSS::DarkSlateGrey as i32, 0x0030_4f4f);
}

#[test]
fn states_enum_covers_the_entire_us() {
    assert_eq!(States::Alabama as usize, 0);
    assert_eq!(States::California as usize, 4);
    assert_eq!(States::Wyoming as usize, 49);
}
