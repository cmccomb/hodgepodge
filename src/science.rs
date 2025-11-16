//! Science datasets such as metric units, SI prefixes, planets, and elements.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

// Enables the optional iterator and variant-count derives.
#[cfg(feature = "enum-count")]
use strum_macros::EnumCount;
#[cfg(feature = "enum-iter")]
use strum_macros::EnumIter;

/// Metric units
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Unit {
    Mass,
    Volume,
    Length,
    Time,
    Angle,
    Temperature,
    Energy,
}

/// The levels of biological classification
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TaxonomicRank {
    /// [Domain](https://en.wikipedia.org/wiki/Domain_(biology)) is the top classification level
    Domain,

    /// [Kingdom](https://en.wikipedia.org/wiki/Kingdom_(biology)) is the level just below Domain
    Kingdom,

    /// [Phylum](https://en.wikipedia.org/wiki/Phylum) is the level just below Kingdom
    Phylum,

    /// [Class](https://en.wikipedia.org/wiki/Class_(biology)) is the level just below Phylum
    Class,

    /// [Order](https://en.wikipedia.org/wiki/Order_(biology)) is the level just below Class
    Order,

    /// [Family](https://en.wikipedia.org/wiki/Family_(biology)) is the level just below Order
    Family,

    /// [Genus](https://en.wikipedia.org/wiki/Genus) is the level just below Family
    Genus,

    /// [Species](https://en.wikipedia.org/wiki/Species) is the basic unit of classification
    Species,
}

/// Metric units greater the one
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrefixLarge {
    Deca = 1,
    Hecto = 2,
    Kilo = 3,
    Mega = 6,
    Giga = 9,
    Tera = 12,
    Peta = 15,
    Exa = 18,
    Zetta = 21,
    Yotta = 24,
}

/// Metric units greater the one
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PrefixSmall {
    Deci = 1,
    Centi = 2,
    Milli = 3,
    Micro = 6,
    Nano = 9,
    Pico = 12,
    Femto = 15,
    Atto = 18,
    Zepto = 21,
    Yocto = 24,
}

/// Planets of the solar system
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Planet {
    /// [Mercury](https://en.wikipedia.org/wiki/Mercury_(planet)) is the closest planet to the sun
    Mercury = 1,

    /// [Venus](https://en.wikipedia.org/wiki/Venus) is the second closest planet to the sun
    Venus = 2,

    /// [Earth](https://en.wikipedia.org/wiki/Earth) is where we live!
    Earth = 3,

    /// [Mars](https://en.wikipedia.org/wiki/Mars) is the fourth closest planet to the sun
    Mars = 4,

    /// [Jupiter](https://en.wikipedia.org/wiki/Jupiter) is the fifth closest planet to the sun
    Jupiter = 5,

    /// [Saturn](https://en.wikipedia.org/wiki/Saturn) is the sixth closest planet to the sun
    Saturn = 6,

    /// [Uranus](https://en.wikipedia.org/wiki/Uranus) is the seventh closest planet to the sun
    Uranus = 7,

    /// [Neptune](https://en.wikipedia.org/wiki/Neptune) is the eighth closest planet to the sun
    Neptune = 8,

    /// [Pluto](https://en.wikipedia.org/wiki/Pluto) will always be a planet in my heart and my code
    Pluto = 9,
}

/// List of the elements
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum Element {
    Hydrogen = 1,
    Helium = 2,
    Lithium = 3,
    Beryllium = 4,
    Boron = 5,
    Carbon = 6,
    Nitrogen = 7,
    Oxygen = 8,
    Fluorine = 9,
    Neon = 10,
    Sodium = 11,
    Magnesium = 12,
    Aluminum = 13,
    Silicon = 14,
    Phosphorus = 15,
    Sulfur = 16,
    Chlorine = 17,
    Argon = 18,
    Potassium = 19,
    Calcium = 20,
    Scandium = 21,
    Titanium = 22,
    Vanadium = 23,
    Chromium = 24,
    Manganese = 25,
    Iron = 26,
    Cobalt = 27,
    Nickel = 28,
    Copper = 29,
    Zinc = 30,
    Gallium = 31,
    Germanium = 32,
    Arsenic = 33,
    Selenium = 34,
    Bromine = 35,
    Krypton = 36,
    Rubidium = 37,
    Strontium = 38,
    Yttrium = 39,
    Zirconium = 40,
    Niobium = 41,
    Molybdenum = 42,
    Technetium = 43,
    Ruthenium = 44,
    Rhodium = 45,
    Palladium = 46,
    Silver = 47,
    Cadmium = 48,
    Indium = 49,
    Tin = 50,
    Antimony = 51,
    Tellurium = 52,
    Iodine = 53,
    Xenon = 54,
    Cesium = 55,
    Barium = 56,
    Lanthanum = 57,
    Cerium = 58,
    Praseodymium = 59,
    Neodymium = 60,
    Promethium = 61,
    Samarium = 62,
    Europium = 63,
    Gadolinium = 64,
    Terbium = 65,
    Dysprosium = 66,
    Holmium = 67,
    Erbium = 68,
    Thulium = 69,
    Ytterbium = 70,
    Lutetium = 71,
    Hafnium = 72,
    Tantalum = 73,
    Wolfram = 74,
    Rhenium = 75,
    Osmium = 76,
    Iridium = 77,
    Platinum = 78,
    Gold = 79,
    Mercury = 80,
    Thallium = 81,
    Lead = 82,
    Bismuth = 83,
    Polonium = 84,
    Astatine = 85,
    Radon = 86,
    Francium = 87,
    Radium = 88,
    Actinium = 89,
    Thorium = 90,
    Protactinium = 91,
    Uranium = 92,
    Neptunium = 93,
    Plutonium = 94,
    Americium = 95,
    Curium = 96,
    Berkelium = 97,
    Californium = 98,
    Einsteinium = 99,
    Fermium = 100,
    Mendelevium = 101,
    Nobelium = 102,
    Lawrencium = 103,
    Rutherfordium = 104,
    Dubnium = 105,
    Seaborgium = 106,
    Bohrium = 107,
    Hassium = 108,
    Meitnerium = 109,
    Darmstadtium = 110,
    Roentgenium = 111,
    Copernicium = 112,
    Nihonium = 113,
    Flerovium = 114,
    Moscovium = 115,
    Livermorium = 116,
    Tennessine = 117,
    Oganesson = 118,
}
