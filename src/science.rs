//! Science datasets such as metric units, SI prefixes, planets, and elements.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

dataset_enum! {
    /// Physical quantity categories, not measurement units or the SI base quantities.
    pub enum Unit {
        Mass => "Mass",
        Volume => "Volume",
        Length => "Length",
        Time => "Time",
        Angle => "Angle",
        Temperature => "Temperature",
        Energy => "Energy",
    }
}

dataset_enum! {
    /// The levels of biological classification
    pub enum TaxonomicRank {
        /// [Domain](https://en.wikipedia.org/wiki/Domain_(biology)) is the top classification level
        Domain => "Domain",

        /// [Kingdom](https://en.wikipedia.org/wiki/Kingdom_(biology)) is the level just below Domain
        Kingdom => "Kingdom",

        /// [Phylum](https://en.wikipedia.org/wiki/Phylum) is the level just below Kingdom
        Phylum => "Phylum",

        /// [Class](https://en.wikipedia.org/wiki/Class_(biology)) is the level just below Phylum
        Class => "Class",

        /// [Order](https://en.wikipedia.org/wiki/Order_(biology)) is the level just below Class
        Order => "Order",

        /// [Family](https://en.wikipedia.org/wiki/Family_(biology)) is the level just below Order
        Family => "Family",

        /// [Genus](https://en.wikipedia.org/wiki/Genus) is the level just below Family
        Genus => "Genus",

        /// [Species](https://en.wikipedia.org/wiki/Species) is the basic unit of classification
        Species => "Species",
    }
}

dataset_enum! {
    /// SI prefixes above one, including the 2022 additions ronna and quetta.
    pub enum PrefixLarge {
        Deca = 1 => "Deca",
        Hecto = 2 => "Hecto",
        Kilo = 3 => "Kilo",
        Mega = 6 => "Mega",
        Giga = 9 => "Giga",
        Tera = 12 => "Tera",
        Peta = 15 => "Peta",
        Exa = 18 => "Exa",
        Zetta = 21 => "Zetta",
        Yotta = 24 => "Yotta",
        Ronna = 27 => "Ronna",
        Quetta = 30 => "Quetta",
    }
}

dataset_enum! {
    /// SI prefixes below one, including the 2022 additions ronto and quecto.
    ///
    /// Discriminants are magnitudes: `Milli as u8 == 3` denotes a factor of 10^-3.
    pub enum PrefixSmall {
        Deci = 1 => "Deci",
        Centi = 2 => "Centi",
        Milli = 3 => "Milli",
        Micro = 6 => "Micro",
        Nano = 9 => "Nano",
        Pico = 12 => "Pico",
        Femto = 15 => "Femto",
        Atto = 18 => "Atto",
        Zepto = 21 => "Zepto",
        Yocto = 24 => "Yocto",
        Ronto = 27 => "Ronto",
        Quecto = 30 => "Quecto",
    }
}

dataset_enum! {
    /// The eight solar-system planets, ordered outward from the Sun.
    ///
    /// Pluto is a dwarf planet and is excluded.
    pub enum Planet {
        /// [Mercury](https://en.wikipedia.org/wiki/Mercury_(planet)) is the closest planet to the sun
        Mercury = 1 => "Mercury",

        /// [Venus](https://en.wikipedia.org/wiki/Venus) is the second closest planet to the sun
        Venus = 2 => "Venus",

        /// [Earth](https://en.wikipedia.org/wiki/Earth) is where we live!
        Earth = 3 => "Earth",

        /// [Mars](https://en.wikipedia.org/wiki/Mars) is the fourth closest planet to the sun
        Mars = 4 => "Mars",

        /// [Jupiter](https://en.wikipedia.org/wiki/Jupiter) is the fifth closest planet to the sun
        Jupiter = 5 => "Jupiter",

        /// [Saturn](https://en.wikipedia.org/wiki/Saturn) is the sixth closest planet to the sun
        Saturn = 6 => "Saturn",

        /// [Uranus](https://en.wikipedia.org/wiki/Uranus) is the seventh closest planet to the sun
        Uranus = 7 => "Uranus",

        /// [Neptune](https://en.wikipedia.org/wiki/Neptune) is the eighth closest planet to the sun
        Neptune = 8 => "Neptune",

    }
}

dataset_enum! {
    /// List of the elements
    #[allow(missing_docs)]
    pub enum Element {
        Hydrogen = 1 => "Hydrogen",
        Helium = 2 => "Helium",
        Lithium = 3 => "Lithium",
        Beryllium = 4 => "Beryllium",
        Boron = 5 => "Boron",
        Carbon = 6 => "Carbon",
        Nitrogen = 7 => "Nitrogen",
        Oxygen = 8 => "Oxygen",
        Fluorine = 9 => "Fluorine",
        Neon = 10 => "Neon",
        Sodium = 11 => "Sodium",
        Magnesium = 12 => "Magnesium",
        Aluminum = 13 => "Aluminum",
        Silicon = 14 => "Silicon",
        Phosphorus = 15 => "Phosphorus",
        Sulfur = 16 => "Sulfur",
        Chlorine = 17 => "Chlorine",
        Argon = 18 => "Argon",
        Potassium = 19 => "Potassium",
        Calcium = 20 => "Calcium",
        Scandium = 21 => "Scandium",
        Titanium = 22 => "Titanium",
        Vanadium = 23 => "Vanadium",
        Chromium = 24 => "Chromium",
        Manganese = 25 => "Manganese",
        Iron = 26 => "Iron",
        Cobalt = 27 => "Cobalt",
        Nickel = 28 => "Nickel",
        Copper = 29 => "Copper",
        Zinc = 30 => "Zinc",
        Gallium = 31 => "Gallium",
        Germanium = 32 => "Germanium",
        Arsenic = 33 => "Arsenic",
        Selenium = 34 => "Selenium",
        Bromine = 35 => "Bromine",
        Krypton = 36 => "Krypton",
        Rubidium = 37 => "Rubidium",
        Strontium = 38 => "Strontium",
        Yttrium = 39 => "Yttrium",
        Zirconium = 40 => "Zirconium",
        Niobium = 41 => "Niobium",
        Molybdenum = 42 => "Molybdenum",
        Technetium = 43 => "Technetium",
        Ruthenium = 44 => "Ruthenium",
        Rhodium = 45 => "Rhodium",
        Palladium = 46 => "Palladium",
        Silver = 47 => "Silver",
        Cadmium = 48 => "Cadmium",
        Indium = 49 => "Indium",
        Tin = 50 => "Tin",
        Antimony = 51 => "Antimony",
        Tellurium = 52 => "Tellurium",
        Iodine = 53 => "Iodine",
        Xenon = 54 => "Xenon",
        Cesium = 55 => "Cesium",
        Barium = 56 => "Barium",
        Lanthanum = 57 => "Lanthanum",
        Cerium = 58 => "Cerium",
        Praseodymium = 59 => "Praseodymium",
        Neodymium = 60 => "Neodymium",
        Promethium = 61 => "Promethium",
        Samarium = 62 => "Samarium",
        Europium = 63 => "Europium",
        Gadolinium = 64 => "Gadolinium",
        Terbium = 65 => "Terbium",
        Dysprosium = 66 => "Dysprosium",
        Holmium = 67 => "Holmium",
        Erbium = 68 => "Erbium",
        Thulium = 69 => "Thulium",
        Ytterbium = 70 => "Ytterbium",
        Lutetium = 71 => "Lutetium",
        Hafnium = 72 => "Hafnium",
        Tantalum = 73 => "Tantalum",
        Wolfram = 74 => "Wolfram",
        Rhenium = 75 => "Rhenium",
        Osmium = 76 => "Osmium",
        Iridium = 77 => "Iridium",
        Platinum = 78 => "Platinum",
        Gold = 79 => "Gold",
        Mercury = 80 => "Mercury",
        Thallium = 81 => "Thallium",
        Lead = 82 => "Lead",
        Bismuth = 83 => "Bismuth",
        Polonium = 84 => "Polonium",
        Astatine = 85 => "Astatine",
        Radon = 86 => "Radon",
        Francium = 87 => "Francium",
        Radium = 88 => "Radium",
        Actinium = 89 => "Actinium",
        Thorium = 90 => "Thorium",
        Protactinium = 91 => "Protactinium",
        Uranium = 92 => "Uranium",
        Neptunium = 93 => "Neptunium",
        Plutonium = 94 => "Plutonium",
        Americium = 95 => "Americium",
        Curium = 96 => "Curium",
        Berkelium = 97 => "Berkelium",
        Californium = 98 => "Californium",
        Einsteinium = 99 => "Einsteinium",
        Fermium = 100 => "Fermium",
        Mendelevium = 101 => "Mendelevium",
        Nobelium = 102 => "Nobelium",
        Lawrencium = 103 => "Lawrencium",
        Rutherfordium = 104 => "Rutherfordium",
        Dubnium = 105 => "Dubnium",
        Seaborgium = 106 => "Seaborgium",
        Bohrium = 107 => "Bohrium",
        Hassium = 108 => "Hassium",
        Meitnerium = 109 => "Meitnerium",
        Darmstadtium = 110 => "Darmstadtium",
        Roentgenium = 111 => "Roentgenium",
        Copernicium = 112 => "Copernicium",
        Nihonium = 113 => "Nihonium",
        Flerovium = 114 => "Flerovium",
        Moscovium = 115 => "Moscovium",
        Livermorium = 116 => "Livermorium",
        Tennessine = 117 => "Tennessine",
        Oganesson = 118 => "Oganesson",
    }
}

impl Element {
    /// Returns the atomic number (1–118).
    #[must_use]
    pub const fn atomic_number(self) -> u8 {
        self as u8
    }

    /// Returns the case-sensitive chemical symbol, such as `Na` or `W`.
    #[must_use]
    #[allow(clippy::too_many_lines)] // One exhaustive reference table.
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Hydrogen => "H",
            Self::Helium => "He",
            Self::Lithium => "Li",
            Self::Beryllium => "Be",
            Self::Boron => "B",
            Self::Carbon => "C",
            Self::Nitrogen => "N",
            Self::Oxygen => "O",
            Self::Fluorine => "F",
            Self::Neon => "Ne",
            Self::Sodium => "Na",
            Self::Magnesium => "Mg",
            Self::Aluminum => "Al",
            Self::Silicon => "Si",
            Self::Phosphorus => "P",
            Self::Sulfur => "S",
            Self::Chlorine => "Cl",
            Self::Argon => "Ar",
            Self::Potassium => "K",
            Self::Calcium => "Ca",
            Self::Scandium => "Sc",
            Self::Titanium => "Ti",
            Self::Vanadium => "V",
            Self::Chromium => "Cr",
            Self::Manganese => "Mn",
            Self::Iron => "Fe",
            Self::Cobalt => "Co",
            Self::Nickel => "Ni",
            Self::Copper => "Cu",
            Self::Zinc => "Zn",
            Self::Gallium => "Ga",
            Self::Germanium => "Ge",
            Self::Arsenic => "As",
            Self::Selenium => "Se",
            Self::Bromine => "Br",
            Self::Krypton => "Kr",
            Self::Rubidium => "Rb",
            Self::Strontium => "Sr",
            Self::Yttrium => "Y",
            Self::Zirconium => "Zr",
            Self::Niobium => "Nb",
            Self::Molybdenum => "Mo",
            Self::Technetium => "Tc",
            Self::Ruthenium => "Ru",
            Self::Rhodium => "Rh",
            Self::Palladium => "Pd",
            Self::Silver => "Ag",
            Self::Cadmium => "Cd",
            Self::Indium => "In",
            Self::Tin => "Sn",
            Self::Antimony => "Sb",
            Self::Tellurium => "Te",
            Self::Iodine => "I",
            Self::Xenon => "Xe",
            Self::Cesium => "Cs",
            Self::Barium => "Ba",
            Self::Lanthanum => "La",
            Self::Cerium => "Ce",
            Self::Praseodymium => "Pr",
            Self::Neodymium => "Nd",
            Self::Promethium => "Pm",
            Self::Samarium => "Sm",
            Self::Europium => "Eu",
            Self::Gadolinium => "Gd",
            Self::Terbium => "Tb",
            Self::Dysprosium => "Dy",
            Self::Holmium => "Ho",
            Self::Erbium => "Er",
            Self::Thulium => "Tm",
            Self::Ytterbium => "Yb",
            Self::Lutetium => "Lu",
            Self::Hafnium => "Hf",
            Self::Tantalum => "Ta",
            Self::Wolfram => "W",
            Self::Rhenium => "Re",
            Self::Osmium => "Os",
            Self::Iridium => "Ir",
            Self::Platinum => "Pt",
            Self::Gold => "Au",
            Self::Mercury => "Hg",
            Self::Thallium => "Tl",
            Self::Lead => "Pb",
            Self::Bismuth => "Bi",
            Self::Polonium => "Po",
            Self::Astatine => "At",
            Self::Radon => "Rn",
            Self::Francium => "Fr",
            Self::Radium => "Ra",
            Self::Actinium => "Ac",
            Self::Thorium => "Th",
            Self::Protactinium => "Pa",
            Self::Uranium => "U",
            Self::Neptunium => "Np",
            Self::Plutonium => "Pu",
            Self::Americium => "Am",
            Self::Curium => "Cm",
            Self::Berkelium => "Bk",
            Self::Californium => "Cf",
            Self::Einsteinium => "Es",
            Self::Fermium => "Fm",
            Self::Mendelevium => "Md",
            Self::Nobelium => "No",
            Self::Lawrencium => "Lr",
            Self::Rutherfordium => "Rf",
            Self::Dubnium => "Db",
            Self::Seaborgium => "Sg",
            Self::Bohrium => "Bh",
            Self::Hassium => "Hs",
            Self::Meitnerium => "Mt",
            Self::Darmstadtium => "Ds",
            Self::Roentgenium => "Rg",
            Self::Copernicium => "Cn",
            Self::Nihonium => "Nh",
            Self::Flerovium => "Fl",
            Self::Moscovium => "Mc",
            Self::Livermorium => "Lv",
            Self::Tennessine => "Ts",
            Self::Oganesson => "Og",
        }
    }
}

impl PrefixLarge {
    /// Returns the signed power of ten represented by this prefix.
    #[must_use]
    pub const fn exponent(self) -> i8 {
        self as i8
    }

    /// Returns the case-sensitive SI symbol; micro uses U+00B5.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Deca => "da",
            Self::Hecto => "h",
            Self::Kilo => "k",
            Self::Mega => "M",
            Self::Giga => "G",
            Self::Tera => "T",
            Self::Peta => "P",
            Self::Exa => "E",
            Self::Zetta => "Z",
            Self::Yotta => "Y",
            Self::Ronna => "R",
            Self::Quetta => "Q",
        }
    }
}

impl PrefixSmall {
    /// Returns the signed power of ten represented by this prefix.
    #[must_use]
    pub const fn exponent(self) -> i8 {
        -(self as i8)
    }

    /// Returns the case-sensitive SI symbol; micro uses U+00B5.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Deci => "d",
            Self::Centi => "c",
            Self::Milli => "m",
            Self::Micro => "µ",
            Self::Nano => "n",
            Self::Pico => "p",
            Self::Femto => "f",
            Self::Atto => "a",
            Self::Zepto => "z",
            Self::Yocto => "y",
            Self::Ronto => "r",
            Self::Quecto => "q",
        }
    }
}

impl Unit {
    /// Returns an SI unit symbol for this quantity (volume uses cubic metres).
    #[must_use]
    pub const fn si_unit_symbol(self) -> &'static str {
        match self {
            Self::Mass => "kg",
            Self::Volume => "m³",
            Self::Length => "m",
            Self::Time => "s",
            Self::Angle => "rad",
            Self::Temperature => "K",
            Self::Energy => "J",
        }
    }
}

impl TaxonomicRank {
    /// Returns the next broader rank in this eight-rank teaching hierarchy.
    #[must_use]
    pub const fn broader(self) -> Option<Self> {
        match self {
            Self::Domain => None,
            Self::Kingdom => Some(Self::Domain),
            Self::Phylum => Some(Self::Kingdom),
            Self::Class => Some(Self::Phylum),
            Self::Order => Some(Self::Class),
            Self::Family => Some(Self::Order),
            Self::Genus => Some(Self::Family),
            Self::Species => Some(Self::Genus),
        }
    }
}

impl Planet {
    /// Returns the planet's position outward from the Sun, from 1 to 8.
    #[must_use]
    pub const fn orbital_order(self) -> u8 {
        self as u8
    }
}

checked_u8_enum!(Element, atomic_number);
checked_u8_enum!(Planet, orbital_order);

impl Element {
    /// Looks up a symbol with exact case; does not trim whitespace.
    #[must_use]
    pub fn from_symbol(code: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.symbol() == code)
    }
}

impl PrefixSmall {
    /// Looks up a symbol with exact case; does not trim whitespace.
    #[must_use]
    pub fn from_symbol(code: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.symbol() == code)
    }
}

impl PrefixLarge {
    /// Looks up a symbol with exact case; does not trim whitespace.
    #[must_use]
    pub fn from_symbol(code: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.symbol() == code)
    }
}

impl PrefixSmall {
    /// Looks up a signed power of ten. Zero has no SI prefix.
    #[must_use]
    pub fn from_exponent(exponent: i8) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.exponent() == exponent)
    }
}

impl PrefixLarge {
    /// Looks up a signed power of ten. Zero has no SI prefix.
    #[must_use]
    pub fn from_exponent(exponent: i8) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.exponent() == exponent)
    }
}

impl TaxonomicRank {
    /// Returns the next narrower rank in this eight-rank teaching hierarchy.
    #[must_use]
    pub const fn narrower(self) -> Option<Self> {
        match self {
            Self::Domain => Some(Self::Kingdom),
            Self::Kingdom => Some(Self::Phylum),
            Self::Phylum => Some(Self::Class),
            Self::Class => Some(Self::Order),
            Self::Order => Some(Self::Family),
            Self::Family => Some(Self::Genus),
            Self::Genus => Some(Self::Species),
            Self::Species => None,
        }
    }
}

dataset_enum! {
    /// All 24 official SI prefixes, in increasing power-of-ten order.
    /// No prefix represents zero exponent; use `None` for an unprefixed unit.
    pub enum SiPrefix {
        Quecto = -30 => "Quecto",
        Ronto = -27 => "Ronto",
        Yocto = -24 => "Yocto",
        Zepto = -21 => "Zepto",
        Atto = -18 => "Atto",
        Femto = -15 => "Femto",
        Pico = -12 => "Pico",
        Nano = -9 => "Nano",
        Micro = -6 => "Micro",
        Milli = -3 => "Milli",
        Centi = -2 => "Centi",
        Deci = -1 => "Deci",
        Deca = 1 => "Deca",
        Hecto = 2 => "Hecto",
        Kilo = 3 => "Kilo",
        Mega = 6 => "Mega",
        Giga = 9 => "Giga",
        Tera = 12 => "Tera",
        Peta = 15 => "Peta",
        Exa = 18 => "Exa",
        Zetta = 21 => "Zetta",
        Yotta = 24 => "Yotta",
        Ronna = 27 => "Ronna",
        Quetta = 30 => "Quetta",
    }
}
impl SiPrefix {
    /// Returns the signed power of ten.
    #[must_use]
    pub const fn exponent(self) -> i8 {
        self as i8
    }
    /// Returns the exact SI symbol; micro is U+00B5, not ASCII u.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Deca => "da",
            Self::Hecto => "h",
            Self::Kilo => "k",
            Self::Mega => "M",
            Self::Giga => "G",
            Self::Tera => "T",
            Self::Peta => "P",
            Self::Exa => "E",
            Self::Zetta => "Z",
            Self::Yotta => "Y",
            Self::Ronna => "R",
            Self::Quetta => "Q",
            Self::Deci => "d",
            Self::Centi => "c",
            Self::Milli => "m",
            Self::Micro => "µ",
            Self::Nano => "n",
            Self::Pico => "p",
            Self::Femto => "f",
            Self::Atto => "a",
            Self::Zepto => "z",
            Self::Yocto => "y",
            Self::Ronto => "r",
            Self::Quecto => "q",
        }
    }
    /// Looks up an exact, case-sensitive SI symbol without trimming.
    #[must_use]
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|prefix| prefix.symbol() == symbol)
    }
    /// Looks up a signed exponent; returns None for zero or a non-SI exponent.
    #[must_use]
    pub fn from_exponent(exponent: i8) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|prefix| prefix.exponent() == exponent)
    }
}
impl From<PrefixSmall> for SiPrefix {
    fn from(value: PrefixSmall) -> Self {
        match value {
            PrefixSmall::Deci => Self::Deci,
            PrefixSmall::Centi => Self::Centi,
            PrefixSmall::Milli => Self::Milli,
            PrefixSmall::Micro => Self::Micro,
            PrefixSmall::Nano => Self::Nano,
            PrefixSmall::Pico => Self::Pico,
            PrefixSmall::Femto => Self::Femto,
            PrefixSmall::Atto => Self::Atto,
            PrefixSmall::Zepto => Self::Zepto,
            PrefixSmall::Yocto => Self::Yocto,
            PrefixSmall::Ronto => Self::Ronto,
            PrefixSmall::Quecto => Self::Quecto,
        }
    }
}
impl From<PrefixLarge> for SiPrefix {
    fn from(value: PrefixLarge) -> Self {
        match value {
            PrefixLarge::Deca => Self::Deca,
            PrefixLarge::Hecto => Self::Hecto,
            PrefixLarge::Kilo => Self::Kilo,
            PrefixLarge::Mega => Self::Mega,
            PrefixLarge::Giga => Self::Giga,
            PrefixLarge::Tera => Self::Tera,
            PrefixLarge::Peta => Self::Peta,
            PrefixLarge::Exa => Self::Exa,
            PrefixLarge::Zetta => Self::Zetta,
            PrefixLarge::Yotta => Self::Yotta,
            PrefixLarge::Ronna => Self::Ronna,
            PrefixLarge::Quetta => Self::Quetta,
        }
    }
}
dataset_enum! {
    /// The seven SI base quantities; distinct from the legacy Unit selection.
    pub enum SiBaseQuantity {
        Time => "Time",
        Length => "Length",
        Mass => "Mass",
        ElectricCurrent => "Electric Current",
        ThermodynamicTemperature => "Thermodynamic Temperature",
        AmountOfSubstance => "Amount of Substance",
        LuminousIntensity => "Luminous Intensity",
    }
}
dataset_enum! {
    /// The seven SI base units, including kilogram as the unit of mass.
    pub enum SiBaseUnit {
        Second => "Second",
        Metre => "Metre",
        Kilogram => "Kilogram",
        Ampere => "Ampere",
        Kelvin => "Kelvin",
        Mole => "Mole",
        Candela => "Candela",
    }
}
impl SiBaseQuantity {
    /// Returns the corresponding SI base unit.
    #[must_use]
    pub const fn unit(self) -> SiBaseUnit {
        match self {
            Self::Time => SiBaseUnit::Second,
            Self::Length => SiBaseUnit::Metre,
            Self::Mass => SiBaseUnit::Kilogram,
            Self::ElectricCurrent => SiBaseUnit::Ampere,
            Self::ThermodynamicTemperature => SiBaseUnit::Kelvin,
            Self::AmountOfSubstance => SiBaseUnit::Mole,
            Self::LuminousIntensity => SiBaseUnit::Candela,
        }
    }
}
impl SiBaseUnit {
    /// Returns the corresponding SI base quantity.
    #[must_use]
    pub const fn quantity(self) -> SiBaseQuantity {
        match self {
            Self::Second => SiBaseQuantity::Time,
            Self::Metre => SiBaseQuantity::Length,
            Self::Kilogram => SiBaseQuantity::Mass,
            Self::Ampere => SiBaseQuantity::ElectricCurrent,
            Self::Kelvin => SiBaseQuantity::ThermodynamicTemperature,
            Self::Mole => SiBaseQuantity::AmountOfSubstance,
            Self::Candela => SiBaseQuantity::LuminousIntensity,
        }
    }
    /// Returns the exact case-sensitive SI symbol.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Second => "s",
            Self::Metre => "m",
            Self::Kilogram => "kg",
            Self::Ampere => "A",
            Self::Kelvin => "K",
            Self::Mole => "mol",
            Self::Candela => "cd",
        }
    }
    /// Looks up an exact SI base-unit symbol without trimming.
    #[must_use]
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|unit| unit.symbol() == symbol)
    }
}
impl Unit {
    /// Returns a corresponding base quantity when this legacy category has one.
    /// Volume, angle and energy are derived quantities and return None.
    #[must_use]
    pub const fn base_quantity(self) -> Option<SiBaseQuantity> {
        match self {
            Self::Time => Some(SiBaseQuantity::Time),
            Self::Length => Some(SiBaseQuantity::Length),
            Self::Mass => Some(SiBaseQuantity::Mass),
            Self::Temperature => Some(SiBaseQuantity::ThermodynamicTemperature),
            Self::Volume | Self::Angle | Self::Energy => None,
        }
    }
}
