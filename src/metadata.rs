//! Provenance and scope shared by the library's enum datasets.

/// How completely a dataset covers its stated scope, not the wider real world.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DatasetCoverage {
    /// Complete under an explicitly stated teaching or mathematical convention.
    CompleteConvention,
    /// Complete for the stated source table and source version/date.
    CompleteSnapshot,
    /// A selected subset; absence does not imply nonexistence.
    CuratedSelection,
    /// A compatibility list retaining aliases or historical names.
    LegacyIllustration,
}

/// A reference supporting a dataset's facts or its explicit selection convention.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DatasetSource {
    /// Source title or issuing organization.
    pub title: &'static str,
    /// Public source or repository documentation URL.
    pub url: &'static str,
    /// A source edition/version, when one is specified by the reference.
    pub version: Option<&'static str>,
    /// Date the library checked/captured this reference, when recorded.
    pub checked_on: Option<&'static str>,
}

/// Machine-readable provenance for the shipped representation of a dataset.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DatasetInfo {
    /// A library-qualified identifier independent of display labels.
    pub id: &'static str,
    /// What is included and the important coverage boundary.
    pub scope: &'static str,
    /// Completeness relative to that scope.
    pub coverage: DatasetCoverage,
    /// References for the facts or conventions; upstream content retains its own terms.
    pub sources: &'static [DatasetSource],
    /// License of this library's shipped representation, not of all referenced content.
    pub license: &'static str,
}

macro_rules! describe {
    ($type:ty, $id:literal, $scope:literal, $coverage:ident, $sources:expr, $license:literal) => {
        impl $type {
            /// The dataset's scope, sources, coverage and representation license.
            pub const INFO: DatasetInfo = DatasetInfo {
                id: $id,
                scope: $scope,
                coverage: DatasetCoverage::$coverage,
                sources: $sources,
                license: $license,
            };
        }
    };
}
const CONVENTION: DatasetSource = DatasetSource {
    title: "Hodgepodge dataset conventions",
    url: "https://github.com/cmccomb/hodgepodge/blob/v0.4.0/DATASETS.md",
    version: Some("0.4.0"),
    checked_on: None,
};
const CSS: DatasetSource = DatasetSource {
    title: "W3C CSS Color 4",
    url: "https://www.w3.org/TR/2026/CRD-css-color-4-20260913/#named-colors",
    version: Some("2026-09-13"),
    checked_on: Some("2026-09-19"),
};
const PREFIX: DatasetSource = DatasetSource {
    title: "BIPM SI prefixes",
    url: "https://www.bipm.org/en/measurement-units/si-prefixes",
    version: Some("2022 additions"),
    checked_on: Some("2026-09-20"),
};
const SI: DatasetSource = DatasetSource {
    title: "BIPM SI base units",
    url: "https://www.bipm.org/en/measurement-units/si-base-units",
    version: None,
    checked_on: Some("2026-09-20"),
};
const ELEMENT: DatasetSource = DatasetSource {
    title: "CIAAW Standard Atomic Weights",
    url: "https://ciaaw.org/atomic-weights.htm",
    version: Some("2024"),
    checked_on: Some("2026-09-19"),
};
const PLANET: DatasetSource = DatasetSource {
    title: "NASA planets",
    url: "https://science.nasa.gov/solar-system/planets/",
    version: None,
    checked_on: Some("2026-09-19"),
};
const EU: DatasetSource = DatasetSource {
    title: "European Union country directory",
    url: "https://european-union.europa.eu/principles-countries-history/eu-countries_en",
    version: None,
    checked_on: Some("2026-09-19"),
};
const US: DatasetSource = DatasetSource {
    title: "USPS Publication 28 Appendix B",
    url: "https://pe.usps.com/text/pub28/28apb.htm",
    version: None,
    checked_on: Some("2026-09-19"),
};
const CA: DatasetSource = DatasetSource { title: "Canada Post symbols and abbreviations", url: "https://www.canadapost-postescanada.ca/cpc/en/support/articles/addressing-guidelines/symbols-and-abbreviations.page", version: None, checked_on: Some("2026-09-19") };
const UN: DatasetSource = DatasetSource {
    title: "UN Statistics Division M49 country and area codes",
    url: "https://unstats.un.org/unsd/methodology/m49/overview/",
    version: None,
    checked_on: Some("2026-09-20"),
};
const TW: DatasetSource = DatasetSource {
    title: "W3C DPV Locations Extension ISO code table (Taiwan supplement)",
    url: "https://www.w3.org/community/reports/dpvcg/CG-FINAL-loc-20240801/",
    version: Some("2024-08-01"),
    checked_on: Some("2026-09-20"),
};
const CALENDAR: DatasetSource = DatasetSource {
    title: "US Naval Observatory leap years",
    url: "https://aa.usno.navy.mil/faq/leap_years",
    version: None,
    checked_on: Some("2026-09-20"),
};
const SEASONS: DatasetSource = DatasetSource {
    title: "NOAA meteorological versus astronomical seasons",
    url: "https://www.ncei.noaa.gov/news/meteorological-versus-astronomical-seasons",
    version: None,
    checked_on: Some("2026-09-20"),
};
const DNA: DatasetSource = DatasetSource {
    title: "NHGRI base pair reference",
    url: "https://www.genome.gov/genetics-glossary/Base-Pair",
    version: None,
    checked_on: Some("2026-09-19"),
};
const AMINO: DatasetSource = DatasetSource {
    title: "NCBI sequence alphabets",
    url: "https://www.ncbi.nlm.nih.gov/IEB/ToolBox/SDKDOCS/BIOSEQ.HTML",
    version: None,
    checked_on: Some("2026-09-19"),
};
const GENETIC: DatasetSource = DatasetSource {
    title: "NCBI genetic codes, standard table 1",
    url: "https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi#SG1",
    version: Some("2024-09-23"),
    checked_on: Some("2026-09-20"),
};
const ANATOMY: DatasetSource = DatasetSource { title: "OpenStax Anatomy and Physiology 2e", url: "https://openstax.org/books/anatomy-and-physiology-2e/pages/7-1-divisions-of-the-skeletal-system", version: Some("2e, 2022"), checked_on: Some("2026-09-19") };
const MUSCLE: DatasetSource = DatasetSource { title: "OpenStax Anatomy and Physiology 2e, sections 11.3-11.6", url: "https://openstax.org/books/anatomy-and-physiology-2e/pages/11-3-axial-muscles-of-the-head-neck-and-back", version: Some("2e, 2022"), checked_on: Some("2026-09-19") };
const BIOME: DatasetSource = DatasetSource { title: "Olson and Dinerstein, The Global 200, Table 1", url: "https://files.worldwildlife.org/wwfcmsprod/files/Publication/file/5xdxix5fsv_The_Global_200_Priority_Ecoregions_for_Global_Conservation.pdf", version: Some("2002"), checked_on: Some("2026-09-19") };
const ROCK: DatasetSource = DatasetSource {
    title: "National Park Service rock classifications",
    url: "https://www.nps.gov/subjects/geology/igneous.htm",
    version: None,
    checked_on: Some("2026-09-19"),
};
const EARTH: DatasetSource = DatasetSource {
    title: "NASA Earth structure",
    url: "https://science.nasa.gov/earth/facts/#h-structure",
    version: None,
    checked_on: Some("2026-09-19"),
};
const ATMOSPHERE: DatasetSource = DatasetSource {
    title: "NASA atmosphere layers",
    url: "https://science.nasa.gov/earth/earth-atmosphere/earths-atmosphere-a-multi-layered-cake/",
    version: None,
    checked_on: Some("2026-09-19"),
};
const ICS: DatasetSource = DatasetSource {
    title: "International Chronostratigraphic Chart",
    url: "https://stratigraphy.org/ICSchart/ChronostratChart2024-12.pdf",
    version: Some("2024/12"),
    checked_on: Some("2026-09-19"),
};
const DANTE: DatasetSource = DatasetSource {
    title: "University of Texas Danteworlds Inferno",
    url: "https://danteworlds.laits.utexas.edu/index2.html",
    version: None,
    checked_on: Some("2026-09-19"),
};
#[cfg(feature = "taxonomy")]
const COL: DatasetSource = DatasetSource {
    title: "Catalogue of Life Base Release",
    url: "https://doi.org/10.48580/dgz5n",
    version: Some("2026-09-11"),
    checked_on: Some("2026-09-19"),
};
#[cfg(feature = "taxonomy")]
const WIKIDATA: DatasetSource = DatasetSource {
    title: "Wikidata captured enrichment",
    url: "https://www.wikidata.org/",
    version: Some("2026-09-19 capture"),
    checked_on: Some("2026-09-19"),
};
describe!(
    crate::ROYGBIV,
    "colors.roygbiv",
    "Named teaching palette; numeric colors are documented swatches, not a physical color model.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::CMYK,
    "colors.cmyk",
    "Named teaching palette; numeric colors are documented swatches, not a physical color model.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::RGB,
    "colors.rgb",
    "Named teaching palette; numeric colors are documented swatches, not a physical color model.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(crate::CSS, "colors.css", "All 148 opaque CSS named colors; aliases retain identity; excludes transparent and currentcolor.", CompleteSnapshot, &[CSS], "MIT OR Apache-2.0");
describe!(
    crate::Unit,
    "science.unit",
    "Seven legacy quantity categories; not the seven SI base quantities or a complete unit system.",
    LegacyIllustration,
    &[CONVENTION, SI],
    "MIT OR Apache-2.0"
);
describe!(
    crate::TaxonomicRank,
    "science.taxonomic_rank",
    "Eight principal teaching ranks; source taxonomy may contain intermediate ranks.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(crate::PrefixLarge, "science.prefix_large", "Official SI prefixes above/below one or their union; no zero-exponent prefix; symbols are case-sensitive.", CompleteSnapshot, &[PREFIX], "MIT OR Apache-2.0");
describe!(crate::PrefixSmall, "science.prefix_small", "Official SI prefixes above/below one or their union; no zero-exponent prefix; symbols are case-sensitive.", CompleteSnapshot, &[PREFIX], "MIT OR Apache-2.0");
describe!(crate::SiPrefix, "science.si_prefix", "Official SI prefixes above/below one or their union; no zero-exponent prefix; symbols are case-sensitive.", CompleteSnapshot, &[PREFIX], "MIT OR Apache-2.0");
describe!(
    crate::SiBaseQuantity,
    "science.si_base_quantity",
    "The seven SI base quantities and their corresponding base units.",
    CompleteSnapshot,
    &[SI],
    "MIT OR Apache-2.0"
);
describe!(
    crate::SiBaseUnit,
    "science.si_base_unit",
    "The seven SI base quantities and their corresponding base units.",
    CompleteSnapshot,
    &[SI],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Planet,
    "science.planet",
    "The eight solar-system planets in orbital order; dwarf planets are excluded.",
    CompleteConvention,
    &[PLANET],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Element,
    "science.element",
    "Elements 1-118 with atomic numbers and symbols; no isotope or atomic-weight data.",
    CompleteSnapshot,
    &[ELEMENT],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Direction,
    "geography.direction",
    "Four cardinal directions; bearings are clockwise from north; no intercardinal points.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Continent,
    "geography.continent",
    "Seven-continent teaching convention; not a unique geopolitical classification.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Ocean,
    "geography.ocean",
    "Five-ocean teaching convention.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::EU,
    "geography.eu",
    "27 EU members as checked September 2026; established Rust spellings retained.",
    CompleteSnapshot,
    &[EU],
    "MIT OR Apache-2.0"
);
describe!(
    crate::States,
    "geography.states",
    "50 US states and postal abbreviations; excludes DC, territories and military addresses.",
    CompleteSnapshot,
    &[US],
    "MIT OR Apache-2.0"
);
describe!(
    crate::CanadianProvince,
    "geography.canadian_province",
    "Ten Canadian provinces and three territories, with postal abbreviations.",
    CompleteSnapshot,
    &[CA],
    "MIT OR Apache-2.0"
);
describe!(crate::Country, "geography.country", "Legacy illustrative names, historical spellings and aliases; not a unique sovereign-state registry.", LegacyIllustration, &[CONVENTION], "MIT OR Apache-2.0");
describe!(crate::IsoCountry, "geography.iso_country", "249 assigned ISO 3166-1 country/area entries, checked 2026-09-20; UN table plus Taiwan; excludes unassigned/user-assigned codes.", CompleteSnapshot, &[UN, TW], "MIT OR Apache-2.0");
describe!(
    crate::Day,
    "time.day",
    "Seven weekdays, numbered Monday=1 through Sunday=7.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(crate::Month, "time.month", "Twelve Gregorian months; leap-aware lengths use the proleptic calendar and astronomical year numbering.", CompleteConvention, &[CALENDAR, CONVENTION], "MIT OR Apache-2.0");
describe!(crate::Season, "time.season", "Four meteorological seasons and Northern/Southern Hemisphere conventions; local wet/dry seasons excluded.", CompleteConvention, &[SEASONS], "MIT OR Apache-2.0");
describe!(crate::Hemisphere, "time.hemisphere", "Four meteorological seasons and Northern/Southern Hemisphere conventions; local wet/dry seasons excluded.", CompleteConvention, &[SEASONS], "MIT OR Apache-2.0");
describe!(
    crate::Quarter,
    "time.quarter",
    "Four quarter numbers; month mappings require an explicit fiscal start month.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Suit,
    "games.suit",
    "Four suits and thirteen ranks of a standard 52-card French-suited deck; no jokers.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Rank,
    "games.rank",
    "Four suits and thirteen ranks of a standard 52-card French-suited deck; no jokers.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::DiceFace,
    "games.dice_face",
    "The six faces of a standard six-sided die.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::ChessPiece,
    "games.chess_piece",
    "Six orthodox chess piece types; colors and individual pieces are not separate variants.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::CoinSide,
    "games.coin_side",
    "Heads and tails under the two-outcome coin convention; excludes edge outcomes.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::RockPaperScissors,
    "games.rock_paper_scissors",
    "Three conventional moves and win/loss/draw outcomes for two-player rock-paper-scissors.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::RoundOutcome,
    "games.round_outcome",
    "Three conventional moves and win/loss/draw outcomes for two-player rock-paper-scissors.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::BetterThanRust,
    "misc.better_than_rust",
    "A humorous one-variant teaching dataset; not an empirical language ranking.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Medal,
    "misc.medal",
    "Gold, silver and bronze places in the three-medal convention.",
    CompleteConvention,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Ordinal,
    "misc.ordinal",
    "English ordinal names for integers 1 through 30.",
    CuratedSelection,
    &[CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::DnaBase,
    "biology.dna_base",
    "Four canonical DNA bases; excludes ambiguity codes and modified bases.",
    CompleteConvention,
    &[DNA],
    "MIT OR Apache-2.0"
);
describe!(
    crate::RnaBase,
    "biology.rna_base",
    "Four canonical RNA bases; excludes ambiguity codes, modified bases and wobble pairing.",
    CompleteConvention,
    &[GENETIC],
    "MIT OR Apache-2.0"
);
describe!(crate::AminoAcid, "biology.amino_acid", "Twenty standard proteinogenic amino acids; excludes selenocysteine, pyrrolysine and ambiguous residues.", CompleteConvention, &[AMINO], "MIT OR Apache-2.0");
describe!(crate::Codon, "biology.codon", "All 64 RNA triplets in 5-prime to 3-prime order; translation uses NCBI standard code table 1 and separates initiation.", CompleteConvention, &[GENETIC], "MIT OR Apache-2.0");
describe!(
    crate::BodySide,
    "anatomy.body_side",
    "Anatomical sides, two skeletal divisions and library-defined regions for the adult skeleton.",
    CompleteConvention,
    &[ANATOMY, CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::SkeletalDivision,
    "anatomy.skeletal_division",
    "Anatomical sides, two skeletal divisions and library-defined regions for the adult skeleton.",
    CompleteConvention,
    &[ANATOMY, CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::BoneRegion,
    "anatomy.bone_region",
    "Anatomical sides, two skeletal divisions and library-defined regions for the adult skeleton.",
    CompleteConvention,
    &[ANATOMY, CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(crate::Bone, "anatomy.bone", "206 individual adult bones with left/right identity; fused adult-bone convention excludes accessory sesamoids and anatomical variants.", CompleteConvention, &[ANATOMY], "MIT OR Apache-2.0");
describe!(crate::Muscle, "anatomy.muscle", "92 selected skeletal muscle types in four broad teaching regions; not individual sided instances or an exhaustive inventory.", CuratedSelection, &[MUSCLE, CONVENTION], "MIT OR Apache-2.0");
describe!(crate::MuscleRegion, "anatomy.muscle_region", "92 selected skeletal muscle types in four broad teaching regions; not individual sided instances or an exhaustive inventory.", CuratedSelection, &[MUSCLE, CONVENTION], "MIT OR Apache-2.0");
describe!(crate::Biome, "ecology.biome", "Fourteen WWF terrestrial biomes; aquatic habitats and topographic landforms are outside scope.", CompleteSnapshot, &[BIOME], "MIT OR Apache-2.0");
describe!(
    crate::RockClass,
    "geology.rock_class",
    "Igneous, sedimentary and metamorphic formation classes.",
    CompleteConvention,
    &[ROCK],
    "MIT OR Apache-2.0"
);
describe!(
    crate::Rock,
    "geology.rock",
    "21 teaching rock names and formation classes; breccia denotes sedimentary breccia here.",
    CuratedSelection,
    &[ROCK, CONVENTION],
    "MIT OR Apache-2.0"
);
describe!(
    crate::EarthLayer,
    "geology.earth_layer",
    "Four compositional/structural Earth layers, outside inward; not mechanical layers.",
    CompleteConvention,
    &[EARTH],
    "MIT OR Apache-2.0"
);
describe!(
    crate::AtmosphereLayer,
    "geology.atmosphere_layer",
    "Five principal atmospheric layers, ground outward; ionosphere overlaps them.",
    CompleteConvention,
    &[ATMOSPHERE],
    "MIT OR Apache-2.0"
);
describe!(crate::GeologicEon, "geology.geologic_eon", "ICS 2024/12 eons through epochs under the documented Cambrian/Carboniferous teaching conventions; excludes ages and numeric boundaries.", CompleteSnapshot, &[ICS, CONVENTION], "MIT OR Apache-2.0");
describe!(crate::GeologicEra, "geology.geologic_era", "ICS 2024/12 eons through epochs under the documented Cambrian/Carboniferous teaching conventions; excludes ages and numeric boundaries.", CompleteSnapshot, &[ICS, CONVENTION], "MIT OR Apache-2.0");
describe!(crate::GeologicPeriod, "geology.geologic_period", "ICS 2024/12 eons through epochs under the documented Cambrian/Carboniferous teaching conventions; excludes ages and numeric boundaries.", CompleteSnapshot, &[ICS, CONVENTION], "MIT OR Apache-2.0");
describe!(crate::GeologicEpoch, "geology.geologic_epoch", "ICS 2024/12 eons through epochs under the documented Cambrian/Carboniferous teaching conventions; excludes ages and numeric boundaries.", CompleteSnapshot, &[ICS, CONVENTION], "MIT OR Apache-2.0");
describe!(
    crate::InfernoCircle,
    "literature.inferno_circle",
    "Nine circles of Dante's Inferno; excludes vestibule and subdivisions.",
    CompleteConvention,
    &[DANTE],
    "MIT OR Apache-2.0"
);
#[cfg(feature = "taxonomy")]
describe!(crate::taxonomy::Species, "taxonomy.species", "Selected animal species and their source ancestry; explicitly extant COL records with unambiguous exact Wikidata matches and English Wikipedia sitelinks; not representative or complete Animalia coverage.", CuratedSelection, &[COL, WIKIDATA], "(MIT OR Apache-2.0) AND CC-BY-4.0");

/// Descriptors for all enum datasets enabled in this build.
/// Helper structs, errors and metadata types are not separate datasets.
pub const DATASETS: &[DatasetInfo] = &[
    crate::ROYGBIV::INFO,
    crate::CMYK::INFO,
    crate::RGB::INFO,
    crate::CSS::INFO,
    crate::Unit::INFO,
    crate::TaxonomicRank::INFO,
    crate::PrefixLarge::INFO,
    crate::PrefixSmall::INFO,
    crate::SiPrefix::INFO,
    crate::SiBaseQuantity::INFO,
    crate::SiBaseUnit::INFO,
    crate::Planet::INFO,
    crate::Element::INFO,
    crate::Direction::INFO,
    crate::Continent::INFO,
    crate::Ocean::INFO,
    crate::EU::INFO,
    crate::States::INFO,
    crate::CanadianProvince::INFO,
    crate::Country::INFO,
    crate::IsoCountry::INFO,
    crate::Day::INFO,
    crate::Month::INFO,
    crate::Season::INFO,
    crate::Hemisphere::INFO,
    crate::Quarter::INFO,
    crate::Suit::INFO,
    crate::Rank::INFO,
    crate::DiceFace::INFO,
    crate::ChessPiece::INFO,
    crate::CoinSide::INFO,
    crate::RockPaperScissors::INFO,
    crate::RoundOutcome::INFO,
    crate::BetterThanRust::INFO,
    crate::Medal::INFO,
    crate::Ordinal::INFO,
    crate::DnaBase::INFO,
    crate::RnaBase::INFO,
    crate::AminoAcid::INFO,
    crate::Codon::INFO,
    crate::BodySide::INFO,
    crate::SkeletalDivision::INFO,
    crate::BoneRegion::INFO,
    crate::Bone::INFO,
    crate::Muscle::INFO,
    crate::MuscleRegion::INFO,
    crate::Biome::INFO,
    crate::RockClass::INFO,
    crate::Rock::INFO,
    crate::EarthLayer::INFO,
    crate::AtmosphereLayer::INFO,
    crate::GeologicEon::INFO,
    crate::GeologicEra::INFO,
    crate::GeologicPeriod::INFO,
    crate::GeologicEpoch::INFO,
    crate::InfernoCircle::INFO,
    #[cfg(feature = "taxonomy")]
    crate::taxonomy::Species::INFO,
];
