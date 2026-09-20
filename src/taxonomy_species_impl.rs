//! Shared behavior for the generated species enum.

use super::{Species, Taxon, SPECIES_NAMES, SPECIES_TAXA};
use std::cmp::Ordering;
use std::sync::OnceLock;

fn compare_names(left: &str, right: &str) -> Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

impl Species {
    /// The canonical variant name used by parsing, display, and serialization.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        SPECIES_NAMES[self as usize]
    }

    /// The accepted scientific name, used as this dataset's display label.
    #[must_use]
    pub fn label(self) -> &'static str {
        self.taxon().scientific_name()
    }

    /// The source record, with names, ancestry, identifiers, and Wikipedia link.
    #[must_use]
    pub const fn taxon(self) -> Taxon {
        SPECIES_TAXA[self as usize]
    }

    /// Convert a selected species record; classification groups return `None`.
    #[must_use]
    pub fn from_taxon(taxon: Taxon) -> Option<Self> {
        SPECIES_TAXA
            .binary_search_by_key(&taxon.0, |record| record.0)
            .ok()
            .map(|index| Self::ALL[index])
    }

    /// Look up a case-sensitive Catalogue of Life ID in the pinned snapshot.
    #[must_use]
    pub fn by_id(id: &str) -> Option<Self> {
        Taxon::by_id(id).and_then(Self::from_taxon)
    }
}

impl crate::Dataset for Species {
    const ALL: &'static [Self] = Self::ALL;
    const COUNT: usize = Self::COUNT;

    fn as_str(self) -> &'static str {
        self.as_str()
    }

    fn label(self) -> &'static str {
        self.label()
    }
}

impl From<Species> for Taxon {
    fn from(species: Species) -> Self {
        species.taxon()
    }
}

impl std::fmt::Debug for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

impl std::str::FromStr for Species {
    type Err = crate::ParseEnumError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        static ORDERED: OnceLock<Vec<Species>> = OnceLock::new();
        let ordered = ORDERED.get_or_init(|| {
            let mut values = Self::ALL.to_vec();
            values.sort_unstable_by(|left, right| compare_names(left.as_str(), right.as_str()));
            values
        });
        ordered
            .binary_search_by(|species| compare_names(species.as_str(), input))
            .map(|index| ordered[index])
            .map_err(|_| crate::ParseEnumError::new("Species"))
    }
}

#[cfg(feature = "strum")]
impl strum::IntoEnumIterator for Species {
    type Iterator = std::iter::Copied<std::slice::Iter<'static, Self>>;

    fn iter() -> Self::Iterator {
        Self::ALL.iter().copied()
    }
}

#[cfg(feature = "strum")]
impl strum::EnumCount for Species {
    const COUNT: usize = Self::COUNT;
}

#[cfg(feature = "rand")]
impl rand::distr::Distribution<Species> for rand::distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Species {
        Species::ALL[rng.random_range(0..Species::COUNT)]
    }
}

#[cfg(feature = "serde")]
mod serialization {
    use super::{Species, SPECIES_NAMES};
    use serde::de::{EnumAccess, Error, Unexpected, VariantAccess, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;

    impl Serialize for Species {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_unit_variant("Species", *self as u32, self.as_str())
        }
    }

    struct Identifier(Species);

    struct IdentifierVisitor;

    impl Visitor<'_> for IdentifierVisitor {
        type Value = Identifier;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a canonical Species variant name or valid variant index")
        }

        fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
            value
                .parse::<Species>()
                .ok()
                .filter(|species| species.as_str() == value)
                .map(Identifier)
                .ok_or_else(|| E::invalid_value(Unexpected::Str(value), &self))
        }

        fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<Self::Value, E> {
            match std::str::from_utf8(value) {
                Ok(text) => self.visit_str(text),
                Err(_) => Err(E::invalid_value(Unexpected::Bytes(value), &self)),
            }
        }

        fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
            usize::try_from(value)
                .ok()
                .and_then(|index| Species::ALL.get(index))
                .copied()
                .map(Identifier)
                .ok_or_else(|| E::invalid_value(Unexpected::Unsigned(value), &self))
        }
    }

    impl<'de> Deserialize<'de> for Identifier {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_identifier(IdentifierVisitor)
        }
    }

    struct SpeciesVisitor;

    impl<'de> Visitor<'de> for SpeciesVisitor {
        type Value = Species;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a Species enum value")
        }

        fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
            let (Identifier(species), variant) = data.variant::<Identifier>()?;
            variant.unit_variant()?;
            Ok(species)
        }
    }

    impl<'de> Deserialize<'de> for Species {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_enum("Species", SPECIES_NAMES, SpeciesVisitor)
        }
    }
}
