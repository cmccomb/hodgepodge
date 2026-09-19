use hodgepodge::*;
use std::{fmt::Debug, hash::Hash, str::FromStr};

macro_rules! all_datasets {
    ($check:ident) => {
        $check::<ROYGBIV>();
        $check::<CMYK>();
        $check::<RGB>();
        $check::<CSS>();
        $check::<Unit>();
        $check::<TaxonomicRank>();
        $check::<PrefixLarge>();
        $check::<PrefixSmall>();
        $check::<Planet>();
        $check::<Element>();
        $check::<Direction>();
        $check::<Continent>();
        $check::<EU>();
        $check::<States>();
        $check::<Ocean>();
        $check::<CanadianProvince>();
        $check::<Country>();
        $check::<Day>();
        $check::<Month>();
        $check::<Season>();
        $check::<Quarter>();
        $check::<Suit>();
        $check::<Rank>();
        $check::<DiceFace>();
        $check::<ChessPiece>();
        $check::<BetterThanRust>();
        $check::<Medal>();
        $check::<Ordinal>();
    };
}

#[test]
fn every_dataset_supports_the_default_api() {
    fn check<
        T: Copy + Clone + Debug + Eq + Hash + std::fmt::Display + FromStr<Err = ParseEnumError>,
    >() {
    }
    all_datasets!(check);
}

#[test]
fn names_are_available_without_optional_features() {
    const SEPTEMBER: &str = Month::September.as_str();
    assert_eq!(SEPTEMBER, "September");
    assert_eq!("sEpTeMbEr".parse::<Month>(), Ok(Month::September));
    assert_eq!(Month::September.to_string(), SEPTEMBER);
    let mut counts = std::collections::HashMap::new();
    counts.insert(Month::September, 3);
    assert_eq!(counts.get(&"september".parse::<Month>().unwrap()), Some(&3));
}

#[test]
fn invalid_names_return_a_typed_error() {
    for input in ["", " September", "September ", "Sep", "9", "Séptember"] {
        let error = input.parse::<Month>().unwrap_err();
        assert_eq!(error.enum_name(), "Month");
        assert_eq!(error.to_string(), "unknown Month variant");
        assert!(std::error::Error::source(&error).is_none());
    }
    assert!("North America".parse::<Continent>().is_err());
    assert_eq!(
        "northamerica".parse::<Continent>(),
        Ok(Continent::NorthAmerica)
    );
}

#[cfg(feature = "strum")]
#[test]
fn every_variant_round_trips_through_its_name() {
    fn check<T>()
    where
        T: Copy
            + Debug
            + Eq
            + Hash
            + std::fmt::Display
            + FromStr<Err = ParseEnumError>
            + IntoEnumIterator
            + EnumCount,
    {
        let mut seen = std::collections::HashSet::new();
        for value in T::iter() {
            let name = value.to_string();
            assert_eq!(name.parse::<T>(), Ok(value));
            assert_eq!(name.to_ascii_lowercase().parse::<T>(), Ok(value));
            assert_eq!(name.to_ascii_uppercase().parse::<T>(), Ok(value));
            assert!(seen.insert(value), "duplicate variant: {name}");
        }
        assert_eq!(seen.len(), T::COUNT);
    }
    all_datasets!(check);
}

#[cfg(all(feature = "serde", feature = "strum"))]
#[test]
fn every_variant_round_trips_through_json() {
    fn check<T>()
    where
        T: Debug
            + Eq
            + std::fmt::Display
            + IntoEnumIterator
            + serde::Serialize
            + serde::de::DeserializeOwned,
    {
        for value in T::iter() {
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json, format!("\"{value}\""));
            assert_eq!(serde_json::from_str::<T>(&json).unwrap(), value);
        }
    }
    all_datasets!(check);
}
