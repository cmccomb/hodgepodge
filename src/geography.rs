//! Geographic datasets covering directions, continents, and civic regions.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

// Enables the optional iterator and variant-count derives.
#[cfg(feature = "strum")]
use strum_macros::{EnumCount, EnumIter};

/// Directions
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Countinents of the world
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Continent {
    NorthAmerica,
    SouthAmerica,
    Asia,
    Africa,
    Europe,
    Antartica,
    Australia,
}

/// Countries in the EU
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EU {
    Austria,
    Belgium,
    Bulgaria,
    Croatia,
    RepublicOfCyprus,
    CzechRepublic,
    Denmark,
    Estonia,
    Finland,
    France,
    Germany,
    Greece,
    Hungary,
    Ireland,
    Italy,
    Latvia,
    Lithuania,
    Luxembourg,
    Malta,
    Netherlands,
    Poland,
    Portugal,
    Romania,
    Slovakia,
    Slovenia,
    SpainAndSweden,
}

/// States of the USA
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum States {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

/// The five major oceans of the world.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ocean {
    /// Arctic Ocean
    Arctic,
    /// Atlantic Ocean
    Atlantic,
    /// Indian Ocean
    Indian,
    /// Pacific Ocean
    Pacific,
    /// Southern Ocean
    Southern,
}

/// Provinces and territories of Canada.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CanadianProvince {
    /// Alberta
    Alberta,
    /// British Columbia
    BritishColumbia,
    /// Manitoba
    Manitoba,
    /// New Brunswick
    NewBrunswick,
    /// Newfoundland and Labrador
    NewfoundlandAndLabrador,
    /// Nova Scotia
    NovaScotia,
    /// Ontario
    Ontario,
    /// Prince Edward Island
    PrinceEdwardIsland,
    /// Quebec
    Quebec,
    /// Saskatchewan
    Saskatchewan,
    /// Northwest Territories
    NorthwestTerritories,
    /// Nunavut
    Nunavut,
    /// Yukon
    Yukon,
}

#[cfg(test)]
mod states_tests {
    use super::States;

    #[test]
    fn states_follow_alphabetical_discriminants() {
        assert_eq!(States::Alabama as usize, 0);
        assert!(States::California as usize > States::Alabama as usize);
        assert_eq!(States::Wyoming as usize, 49);
    }

    #[cfg(feature = "enum-count")]
    #[test]
    fn united_states_has_fifty_states() {
        use strum::EnumCount;

        assert_eq!(<States as EnumCount>::COUNT, 50);
    }
}

#[cfg(test)]
mod geography_counts_tests {
    #[cfg(feature = "enum-count")]
    #[test]
    fn oceans_have_five_variants() {
        use strum::EnumCount;

        assert_eq!(<super::Ocean as EnumCount>::COUNT, 5);
    }

    #[cfg(feature = "enum-count")]
    #[test]
    fn canadian_provinces_have_thirteen_variants() {
        use strum::EnumCount;

        assert_eq!(<super::CanadianProvince as EnumCount>::COUNT, 13);
    }
}

/// Countries of the world
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Country {
    Afghanistan,
    Albania,
    Algeria,
    Andorra,
    Angola,
    AntiguaAndDeps,
    Argentina,
    Armenia,
    Australia,
    Austria,
    Azerbaijan,
    Bahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bhutan,
    Bolivia,
    BosniaHerzegovina,
    Botswana,
    Brazil,
    Brunei,
    Bulgaria,
    Burkina,
    Burundi,
    Cambodia,
    Cameroon,
    Canada,
    CapeVerde,
    CentralAfricanRepublic,
    Chad,
    Chile,
    China,
    Colombia,
    Comoros,
    RepublicOfTheCongo,
    DemocraticRepublicOfTheCongo,
    CostaRica,
    Croatia,
    Cuba,
    Cyprus,
    CzechRepublic,
    Denmark,
    Djibouti,
    Dominica,
    DominicanRepublic,
    EastTimor,
    Ecuador,
    Egypt,
    ElSalvador,
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Ethiopia,
    Fiji,
    Finland,
    France,
    Gabon,
    Gambia,
    Georgia,
    Germany,
    Ghana,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    GuineaBissau,
    Guyana,
    Haiti,
    Honduras,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    RepublicOfIreland,
    Israel,
    Italy,
    IvoryCoast,
    Jamaica,
    Japan,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    NorthKorea,
    SouthKorea,
    Kosovo,
    Kuwait,
    Kyrgyzstan,
    Laos,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Macedonia,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    MarshallIslands,
    Mauritania,
    Mauritius,
    Mexico,
    Micronesia,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Mozambique,
    Myanmar,
    Burma,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    Norway,
    Oman,
    Pakistan,
    Palau,
    Panama,
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    Qatar,
    Romania,
    RussianFederation,
    Rwanda,
    StKittsAndNevis,
    StLucia,
    SaintVincentAndTheGrenadines,
    Samoa,
    SanMarino,
    SaoTomeAndPrincipe,
    SaudiArabia,
    Senegal,
    Serbia,
    Seychelles,
    SierraLeone,
    Singapore,
    Slovakia,
    Slovenia,
    SolomonIslands,
    Somalia,
    SouthAfrica,
    SouthSudan,
    Spain,
    SriLanka,
    Sudan,
    Suriname,
    Swaziland,
    Sweden,
    Switzerland,
    Syria,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    Togo,
    Tonga,
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    Tuvalu,
    Uganda,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdom,
    UnitedStates,
    Uruguay,
    Uzbekistan,
    Vanuatu,
    VaticanCity,
    Venezuela,
    Vietnam,
    Yemen,
    Zambia,
    Zimbabwe,
}
