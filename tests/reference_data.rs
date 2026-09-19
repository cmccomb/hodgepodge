//! Fixtures are sourced independently of the enum definitions; see fixtures/README.md.

use hodgepodge::{Continent, CSS, EU};

#[test]
fn every_css_name_matches_the_w3c_rgb_value() {
    let mut names = std::collections::HashSet::new();
    for line in include_str!("fixtures/css-colors.tsv").lines() {
        let (name, hex) = line.split_once('\t').expect("name and hex columns");
        assert!(names.insert(name), "duplicate reference name: {name}");
        let color: CSS = name.parse().expect("CSS keyword must be supported");
        let expected = u32::from_str_radix(hex, 16).expect("reference RGB");
        assert_eq!(color.as_str().to_ascii_lowercase(), name);
        assert_eq!(color.rgb(), expected, "wrong RGB for {name}");
        assert_eq!(format!("{color:x}"), hex, "wrong formatting for {name}");
    }
    assert_eq!(names.len(), 148);
}

#[test]
fn eu_members_match_the_reference_directory() {
    let members: std::collections::HashSet<EU> = include_str!("fixtures/eu-members.txt")
        .lines()
        .map(|name| name.parse().expect("EU member must be supported"))
        .collect();
    assert_eq!(members.len(), 27);
    assert!(members.contains(&EU::Spain));
    assert!(members.contains(&EU::Sweden));
    assert_ne!(EU::Spain, EU::Sweden);
    assert!("SpainAndSweden".parse::<EU>().is_err());
}

#[cfg(feature = "strum")]
#[test]
fn iteration_contains_exactly_the_reference_members() {
    use hodgepodge::IntoEnumIterator;
    use std::collections::HashSet;

    let css_names: HashSet<_> = include_str!("fixtures/css-colors.tsv")
        .lines()
        .map(|line| line.split_once('\t').unwrap().0.to_owned())
        .collect();
    assert_eq!(
        CSS::iter()
            .map(|color| color.as_str().to_ascii_lowercase())
            .collect::<HashSet<_>>(),
        css_names
    );
    let eu_members: HashSet<EU> = include_str!("fixtures/eu-members.txt")
        .lines()
        .map(|name| name.parse().unwrap())
        .collect();
    assert_eq!(EU::iter().collect::<HashSet<_>>(), eu_members);
}

#[test]
fn corrected_names_are_unambiguous() {
    assert_eq!("antarctica".parse::<Continent>(), Ok(Continent::Antarctica));
    assert_eq!("fuchsia".parse::<CSS>(), Ok(CSS::Fuchsia));
    assert!("antartica".parse::<Continent>().is_err());
    assert!("fuschia".parse::<CSS>().is_err());
}

#[test]
fn color_aliases_retain_identity_and_share_exact_rgb() {
    let pairs = [
        (CSS::Aqua, CSS::Cyan),
        (CSS::Fuchsia, CSS::Magenta),
        (CSS::Grey, CSS::Gray),
        (CSS::DarkGrey, CSS::DarkGray),
        (CSS::DimGrey, CSS::DimGray),
        (CSS::LightGrey, CSS::LightGray),
        (CSS::SlateGrey, CSS::SlateGray),
        (CSS::DarkSlateGrey, CSS::DarkSlateGray),
        (CSS::LightSlateGrey, CSS::LightSlateGray),
    ];
    for (alias, canonical) in pairs {
        assert_ne!(alias, canonical);
        assert_eq!(alias.rgb(), canonical.rgb());
    }
}

#[test]
fn color_hex_formatting_supports_padding_and_prefixes() {
    assert_eq!(format!("{:x}", CSS::Blue), "0000ff");
    assert_eq!(format!("{:#x}", CSS::Blue), "0x0000ff");
    assert_eq!(format!("{:>8x}", CSS::Blue), "  0000ff");
    assert_eq!(format!("{:08x}", CSS::Blue), "000000ff");
}
