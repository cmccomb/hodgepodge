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
        $check::<CoinSide>();
        $check::<RockPaperScissors>();
        $check::<RoundOutcome>();
        $check::<DnaBase>();
        $check::<AminoAcid>();
        $check::<BodySide>();
        $check::<SkeletalDivision>();
        $check::<BoneRegion>();
        $check::<Bone>();
        $check::<MuscleRegion>();
        $check::<Muscle>();
        $check::<Biome>();
        $check::<RockClass>();
        $check::<Rock>();
        $check::<EarthLayer>();
        $check::<AtmosphereLayer>();
        $check::<GeologicEon>();
        $check::<GeologicEra>();
        $check::<GeologicPeriod>();
        $check::<GeologicEpoch>();
        $check::<InfernoCircle>();
    };
}

#[test]
fn every_dataset_supports_the_default_api() {
    fn check<
        T: Dataset
            + Copy
            + Clone
            + Debug
            + Eq
            + Hash
            + std::fmt::Display
            + FromStr<Err = ParseEnumError>,
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

#[test]
fn every_variant_round_trips_through_its_name() {
    fn check<T>()
    where
        T: Dataset + Debug + Eq + Hash + std::fmt::Display + FromStr<Err = ParseEnumError>,
    {
        let mut seen = std::collections::HashSet::new();
        for &value in T::ALL {
            let name = value.to_string();
            assert_eq!(name, value.as_str());
            assert!(!value.label().is_empty());
            assert_eq!(name.parse::<T>(), Ok(value));
            assert_eq!(name.to_ascii_lowercase().parse::<T>(), Ok(value));
            assert_eq!(name.to_ascii_uppercase().parse::<T>(), Ok(value));
            assert!(seen.insert(value), "duplicate variant: {name}");
        }
        assert_eq!(seen.len(), T::COUNT);
    }
    all_datasets!(check);
}

#[cfg(feature = "serde")]
#[test]
fn every_variant_round_trips_through_json() {
    fn check<T>()
    where
        T: Dataset
            + Debug
            + Eq
            + std::fmt::Display
            + serde::Serialize
            + serde::de::DeserializeOwned,
    {
        for &value in T::ALL {
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json, format!("\"{value}\""));
            assert_eq!(serde_json::from_str::<T>(&json).unwrap(), value);
        }
    }
    all_datasets!(check);
}

#[cfg(feature = "strum")]
#[test]
fn strum_metadata_agrees_with_default_metadata() {
    fn check<T: Dataset + IntoEnumIterator + EnumCount + Eq + Debug>() {
        assert_eq!(<T as Dataset>::COUNT, <T as EnumCount>::COUNT);
        assert_eq!(T::iter().collect::<Vec<_>>(), T::ALL);
    }
    all_datasets!(check);
}

#[cfg(feature = "rand")]
#[test]
fn every_dataset_supports_caller_seeded_sampling() {
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    fn check<T: Dataset + Eq + Debug>()
    where
        rand::distr::StandardUniform: rand::distr::Distribution<T>,
    {
        let mut first = ChaCha8Rng::seed_from_u64(42);
        let mut second = ChaCha8Rng::seed_from_u64(42);
        for _ in 0..100 {
            let value: T = first.random();
            assert!(T::ALL.contains(&value));
            assert_eq!(value, second.random());
        }
    }
    all_datasets!(check);
}
