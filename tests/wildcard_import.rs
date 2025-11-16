use hodgepodge::*;

#[test]
fn wildcard_imports_bring_enums_into_scope() {
    let favorite_color = ROYGBIV::Indigo;
    assert_eq!(format!("{favorite_color:x}"), "4b0082");

    let state = States::Oregon;
    assert!(matches!(state, States::Oregon));

    let medal = Medal::Gold;
    assert_eq!(medal as u8, 1);
}
