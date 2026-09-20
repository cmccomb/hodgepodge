use hodgepodge::*;
use std::collections::HashSet;

#[test]
fn all_element_numbers_and_symbols_match_ciaaw() {
    let mut seen = HashSet::new();
    let mut symbols = HashSet::new();
    for row in include_str!("fixtures/elements.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let number: u8 = fields[0].parse().unwrap();
        let name = match number {
            13 => "Aluminum",
            55 => "Cesium",
            74 => "Wolfram",
            _ => fields[2],
        };
        let element: Element = name.parse().unwrap();
        assert_eq!(element.atomic_number(), number);
        assert_eq!(element.symbol(), fields[1]);
        assert!(seen.insert(element));
        assert!(symbols.insert(element.symbol()));
    }
    assert_eq!(seen.len(), 118);
    assert_eq!(seen, Element::ALL.iter().copied().collect());
}

#[test]
fn every_si_prefix_has_the_reference_symbol_and_signed_exponent() {
    let mut large = HashSet::new();
    let mut small = HashSet::new();
    for row in include_str!("fixtures/si-prefixes.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let exponent: i8 = fields[3].parse().unwrap();
        match fields[0] {
            "PrefixLarge" => {
                let value: PrefixLarge = fields[1].parse().unwrap();
                assert_eq!(value.symbol(), fields[2]);
                assert_eq!(value.exponent(), exponent);
                assert!(large.insert(value));
            }
            "PrefixSmall" => {
                let value: PrefixSmall = fields[1].parse().unwrap();
                assert_eq!(value.symbol(), fields[2]);
                assert_eq!(value.exponent(), exponent);
                assert!(small.insert(value));
            }
            _ => panic!("unknown prefix family"),
        }
    }
    assert_eq!(large.len(), 12);
    assert_eq!(small.len(), 12);
    assert_eq!(large, PrefixLarge::ALL.iter().copied().collect());
    assert_eq!(small, PrefixSmall::ALL.iter().copied().collect());
    assert_eq!(PrefixSmall::Milli as u8, 3); // Legacy discriminant preserved.
}

#[test]
fn geographic_postal_codes_match_official_reference_sets() {
    let mut states = HashSet::new();
    let mut provinces = HashSet::new();
    let mut codes = HashSet::new();
    for row in include_str!("fixtures/us-states.tsv").lines() {
        let (name, code) = row.split_once('\t').unwrap();
        let state: States = name.parse().unwrap();
        assert_eq!(state.postal_abbreviation(), code);
        assert!(states.insert(state));
        assert!(codes.insert(code));
    }
    assert_eq!(states.len(), 50);
    assert_eq!(states, States::ALL.iter().copied().collect());
    codes.clear();
    for row in include_str!("fixtures/canada.tsv").lines() {
        let (name, code) = row.split_once('\t').unwrap();
        let province: CanadianProvince = name.parse().unwrap();
        assert_eq!(province.postal_abbreviation(), code);
        assert!(provinces.insert(province));
        assert!(codes.insert(code));
    }
    assert_eq!(provinces.len(), 13);
    assert_eq!(provinces, CanadianProvince::ALL.iter().copied().collect());
    let territories: HashSet<_> = CanadianProvince::ALL
        .iter()
        .copied()
        .filter(|p| p.is_territory())
        .collect();
    assert_eq!(
        territories,
        HashSet::from([
            CanadianProvince::NorthwestTerritories,
            CanadianProvince::Nunavut,
            CanadianProvince::Yukon
        ])
    );
}

#[test]
fn presentation_labels_leave_names_and_parsing_unchanged() {
    assert_eq!(States::NewYork.label(), "New York");
    assert_eq!(Bone::LeftRib12.label(), "Left Rib 12");
    assert_eq!(AminoAcid::AsparticAcid.label(), "Aspartic Acid");
    assert_eq!(Ordinal::Twentyfirst.label(), "Twenty-first");
    assert_eq!(States::NewYork.as_str(), "NewYork");
    assert!("New York".parse::<States>().is_err());
}

#[test]
fn calendar_and_compass_metadata_follow_documented_conventions() {
    assert_eq!(
        Month::ALL
            .iter()
            .map(|m| u16::from(m.days_in_common_year()))
            .sum::<u16>(),
        365
    );
    assert_eq!(Month::February.days_in_common_year(), 28);
    assert_eq!(Day::ALL.iter().filter(|d| d.is_weekend()).count(), 2);
    assert!(Day::Saturday.is_weekend() && Day::Sunday.is_weekend());
    for value in Day::ALL {
        assert_eq!(value.abbreviation(), &value.as_str()[..3]);
    }
    for value in Month::ALL {
        assert_eq!(value.abbreviation(), &value.as_str()[..3]);
    }
    let months: Vec<_> = Season::ALL.iter().flat_map(|s| s.months()).collect();
    assert_eq!(months.len(), 12);
    assert_eq!(
        months.into_iter().collect::<HashSet<_>>(),
        Month::ALL.iter().copied().collect()
    );
    assert_eq!(
        Season::Winter.months(),
        [Month::December, Month::January, Month::February]
    );
    for value in Direction::ALL {
        assert_eq!(value.opposite().opposite(), *value);
        assert_eq!(
            (value.bearing_degrees() + 180) % 360,
            value.opposite().bearing_degrees()
        );
    }
    assert_eq!(Direction::East.bearing_degrees(), 90);
    assert_eq!(Direction::West.bearing_degrees(), 270);
}

#[test]
fn metadata_respects_existing_numeric_meanings() {
    assert_eq!(ChessPiece::King.material_value(), None);
    assert_eq!(ChessPiece::Queen.material_value(), Some(9));
    assert_eq!(ChessPiece::King.ordinal(), 6);
    assert!(!Rank::Ace.is_face_card());
    assert_eq!(Rank::ALL.iter().filter(|r| r.is_face_card()).count(), 3);
    assert_eq!(TaxonomicRank::Domain.broader(), None);
    assert_eq!(TaxonomicRank::Species.broader(), Some(TaxonomicRank::Genus));
    assert_eq!(Unit::Mass.si_unit_symbol(), "kg");
    assert_eq!(Unit::Volume.si_unit_symbol(), "m³");
    for value in Planet::ALL {
        assert_eq!(value.orbital_order(), *value as u8);
    }
    assert_eq!(Planet::COUNT, 8);
    assert!("Pluto".parse::<Planet>().is_err());
    for color in CSS::ALL {
        let [r, g, b] = color.rgb_channels();
        assert_eq!(u32::from_be_bytes([0, r, g, b]), color.rgb());
    }
}

#[cfg(feature = "serde")]
#[test]
fn removed_planet_is_rejected_in_stored_data() {
    assert!(serde_json::from_str::<Planet>(r#""Pluto""#).is_err());
}
