//! Geographic datasets covering directions, continents, and civic regions.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

dataset_enum! {
    /// Directions
    pub enum Direction {
        North => "North",
        South => "South",
        East => "East",
        West => "West",
    }
}

dataset_enum! {
    /// Continents of the world
    pub enum Continent {
        NorthAmerica => "North America",
        SouthAmerica => "South America",
        Asia => "Asia",
        Africa => "Africa",
        Europe => "Europe",
        Antarctica => "Antarctica",
        Australia => "Australia",
    }
}

dataset_enum! {
    /// The 27 EU member states, checked against the [EU directory](https://european-union.europa.eu/principles-countries-history/eu-countries_en).
    ///
    /// Membership snapshot: September 2026. Variant names retain the crate's
    /// established spellings (for example, `CzechRepublic`).
    pub enum EU {
        Austria => "Austria",
        Belgium => "Belgium",
        Bulgaria => "Bulgaria",
        Croatia => "Croatia",
        RepublicOfCyprus => "Republic of Cyprus",
        CzechRepublic => "Czech Republic",
        Denmark => "Denmark",
        Estonia => "Estonia",
        Finland => "Finland",
        France => "France",
        Germany => "Germany",
        Greece => "Greece",
        Hungary => "Hungary",
        Ireland => "Ireland",
        Italy => "Italy",
        Latvia => "Latvia",
        Lithuania => "Lithuania",
        Luxembourg => "Luxembourg",
        Malta => "Malta",
        Netherlands => "Netherlands",
        Poland => "Poland",
        Portugal => "Portugal",
        Romania => "Romania",
        Slovakia => "Slovakia",
        Slovenia => "Slovenia",
        Spain => "Spain",
        Sweden => "Sweden",
    }
}

dataset_enum! {
    /// States of the USA
    pub enum States {
        Alabama => "Alabama",
        Alaska => "Alaska",
        Arizona => "Arizona",
        Arkansas => "Arkansas",
        California => "California",
        Colorado => "Colorado",
        Connecticut => "Connecticut",
        Delaware => "Delaware",
        Florida => "Florida",
        Georgia => "Georgia",
        Hawaii => "Hawaii",
        Idaho => "Idaho",
        Illinois => "Illinois",
        Indiana => "Indiana",
        Iowa => "Iowa",
        Kansas => "Kansas",
        Kentucky => "Kentucky",
        Louisiana => "Louisiana",
        Maine => "Maine",
        Maryland => "Maryland",
        Massachusetts => "Massachusetts",
        Michigan => "Michigan",
        Minnesota => "Minnesota",
        Mississippi => "Mississippi",
        Missouri => "Missouri",
        Montana => "Montana",
        Nebraska => "Nebraska",
        Nevada => "Nevada",
        NewHampshire => "New Hampshire",
        NewJersey => "New Jersey",
        NewMexico => "New Mexico",
        NewYork => "New York",
        NorthCarolina => "North Carolina",
        NorthDakota => "North Dakota",
        Ohio => "Ohio",
        Oklahoma => "Oklahoma",
        Oregon => "Oregon",
        Pennsylvania => "Pennsylvania",
        RhodeIsland => "Rhode Island",
        SouthCarolina => "South Carolina",
        SouthDakota => "South Dakota",
        Tennessee => "Tennessee",
        Texas => "Texas",
        Utah => "Utah",
        Vermont => "Vermont",
        Virginia => "Virginia",
        Washington => "Washington",
        WestVirginia => "West Virginia",
        Wisconsin => "Wisconsin",
        Wyoming => "Wyoming",
    }
}

dataset_enum! {
    /// The five major oceans of the world.
    pub enum Ocean {
        /// Arctic Ocean
        Arctic => "Arctic Ocean",
        /// Atlantic Ocean
        Atlantic => "Atlantic Ocean",
        /// Indian Ocean
        Indian => "Indian Ocean",
        /// Pacific Ocean
        Pacific => "Pacific Ocean",
        /// Southern Ocean
        Southern => "Southern Ocean",
    }
}

dataset_enum! {
    /// Provinces and territories of Canada.
    pub enum CanadianProvince {
        /// Alberta
        Alberta => "Alberta",
        /// British Columbia
        BritishColumbia => "British Columbia",
        /// Manitoba
        Manitoba => "Manitoba",
        /// New Brunswick
        NewBrunswick => "New Brunswick",
        /// Newfoundland and Labrador
        NewfoundlandAndLabrador => "Newfoundland and Labrador",
        /// Nova Scotia
        NovaScotia => "Nova Scotia",
        /// Ontario
        Ontario => "Ontario",
        /// Prince Edward Island
        PrinceEdwardIsland => "Prince Edward Island",
        /// Quebec
        Quebec => "Quebec",
        /// Saskatchewan
        Saskatchewan => "Saskatchewan",
        /// Northwest Territories
        NorthwestTerritories => "Northwest Territories",
        /// Nunavut
        Nunavut => "Nunavut",
        /// Yukon
        Yukon => "Yukon",
    }
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

    #[cfg(feature = "strum")]
    #[test]
    fn united_states_has_fifty_states() {
        use strum::EnumCount;

        assert_eq!(<States as EnumCount>::COUNT, 50);
    }
}

#[cfg(test)]
mod geography_counts_tests {
    #[cfg(feature = "strum")]
    #[test]
    fn oceans_have_five_variants() {
        use strum::EnumCount;

        assert_eq!(<super::Ocean as EnumCount>::COUNT, 5);
    }

    #[cfg(feature = "strum")]
    #[test]
    fn canadian_provinces_have_thirteen_variants() {
        use strum::EnumCount;

        assert_eq!(<super::CanadianProvince as EnumCount>::COUNT, 13);
    }
}

dataset_enum! {
    /// A legacy illustrative country-name list, not an ISO country registry.
    ///
    /// Names and inclusion reflect the original dataset; `Myanmar` and `Burma`,
    /// for example, are two names for the same country. Do not infer a count of
    /// sovereign states or use the discriminants as country codes.
    pub enum Country {
        Afghanistan => "Afghanistan",
        Albania => "Albania",
        Algeria => "Algeria",
        Andorra => "Andorra",
        Angola => "Angola",
        AntiguaAndDeps => "Antigua and Deps",
        Argentina => "Argentina",
        Armenia => "Armenia",
        Australia => "Australia",
        Austria => "Austria",
        Azerbaijan => "Azerbaijan",
        Bahamas => "Bahamas",
        Bahrain => "Bahrain",
        Bangladesh => "Bangladesh",
        Barbados => "Barbados",
        Belarus => "Belarus",
        Belgium => "Belgium",
        Belize => "Belize",
        Benin => "Benin",
        Bhutan => "Bhutan",
        Bolivia => "Bolivia",
        BosniaHerzegovina => "Bosnia Herzegovina",
        Botswana => "Botswana",
        Brazil => "Brazil",
        Brunei => "Brunei",
        Bulgaria => "Bulgaria",
        Burkina => "Burkina",
        Burundi => "Burundi",
        Cambodia => "Cambodia",
        Cameroon => "Cameroon",
        Canada => "Canada",
        CapeVerde => "Cape Verde",
        CentralAfricanRepublic => "Central African Republic",
        Chad => "Chad",
        Chile => "Chile",
        China => "China",
        Colombia => "Colombia",
        Comoros => "Comoros",
        RepublicOfTheCongo => "Republic of The Congo",
        DemocraticRepublicOfTheCongo => "Democratic Republic of The Congo",
        CostaRica => "Costa Rica",
        Croatia => "Croatia",
        Cuba => "Cuba",
        Cyprus => "Cyprus",
        CzechRepublic => "Czech Republic",
        Denmark => "Denmark",
        Djibouti => "Djibouti",
        Dominica => "Dominica",
        DominicanRepublic => "Dominican Republic",
        EastTimor => "East Timor",
        Ecuador => "Ecuador",
        Egypt => "Egypt",
        ElSalvador => "El Salvador",
        EquatorialGuinea => "Equatorial Guinea",
        Eritrea => "Eritrea",
        Estonia => "Estonia",
        Ethiopia => "Ethiopia",
        Fiji => "Fiji",
        Finland => "Finland",
        France => "France",
        Gabon => "Gabon",
        Gambia => "Gambia",
        Georgia => "Georgia",
        Germany => "Germany",
        Ghana => "Ghana",
        Greece => "Greece",
        Grenada => "Grenada",
        Guatemala => "Guatemala",
        Guinea => "Guinea",
        GuineaBissau => "Guinea Bissau",
        Guyana => "Guyana",
        Haiti => "Haiti",
        Honduras => "Honduras",
        Hungary => "Hungary",
        Iceland => "Iceland",
        India => "India",
        Indonesia => "Indonesia",
        Iran => "Iran",
        Iraq => "Iraq",
        RepublicOfIreland => "Republic of Ireland",
        Israel => "Israel",
        Italy => "Italy",
        IvoryCoast => "Ivory Coast",
        Jamaica => "Jamaica",
        Japan => "Japan",
        Jordan => "Jordan",
        Kazakhstan => "Kazakhstan",
        Kenya => "Kenya",
        Kiribati => "Kiribati",
        NorthKorea => "North Korea",
        SouthKorea => "South Korea",
        Kosovo => "Kosovo",
        Kuwait => "Kuwait",
        Kyrgyzstan => "Kyrgyzstan",
        Laos => "Laos",
        Latvia => "Latvia",
        Lebanon => "Lebanon",
        Lesotho => "Lesotho",
        Liberia => "Liberia",
        Libya => "Libya",
        Liechtenstein => "Liechtenstein",
        Lithuania => "Lithuania",
        Luxembourg => "Luxembourg",
        Macedonia => "Macedonia",
        Madagascar => "Madagascar",
        Malawi => "Malawi",
        Malaysia => "Malaysia",
        Maldives => "Maldives",
        Mali => "Mali",
        Malta => "Malta",
        MarshallIslands => "Marshall Islands",
        Mauritania => "Mauritania",
        Mauritius => "Mauritius",
        Mexico => "Mexico",
        Micronesia => "Micronesia",
        Moldova => "Moldova",
        Monaco => "Monaco",
        Mongolia => "Mongolia",
        Montenegro => "Montenegro",
        Morocco => "Morocco",
        Mozambique => "Mozambique",
        Myanmar => "Myanmar",
        Burma => "Burma",
        Namibia => "Namibia",
        Nauru => "Nauru",
        Nepal => "Nepal",
        Netherlands => "Netherlands",
        NewZealand => "New Zealand",
        Nicaragua => "Nicaragua",
        Niger => "Niger",
        Nigeria => "Nigeria",
        Norway => "Norway",
        Oman => "Oman",
        Pakistan => "Pakistan",
        Palau => "Palau",
        Panama => "Panama",
        PapuaNewGuinea => "Papua New Guinea",
        Paraguay => "Paraguay",
        Peru => "Peru",
        Philippines => "Philippines",
        Poland => "Poland",
        Portugal => "Portugal",
        Qatar => "Qatar",
        Romania => "Romania",
        RussianFederation => "Russian Federation",
        Rwanda => "Rwanda",
        StKittsAndNevis => "St Kitts and Nevis",
        StLucia => "St Lucia",
        SaintVincentAndTheGrenadines => "Saint Vincent and The Grenadines",
        Samoa => "Samoa",
        SanMarino => "San Marino",
        SaoTomeAndPrincipe => "Sao Tome and Principe",
        SaudiArabia => "Saudi Arabia",
        Senegal => "Senegal",
        Serbia => "Serbia",
        Seychelles => "Seychelles",
        SierraLeone => "Sierra Leone",
        Singapore => "Singapore",
        Slovakia => "Slovakia",
        Slovenia => "Slovenia",
        SolomonIslands => "Solomon Islands",
        Somalia => "Somalia",
        SouthAfrica => "South Africa",
        SouthSudan => "South Sudan",
        Spain => "Spain",
        SriLanka => "Sri Lanka",
        Sudan => "Sudan",
        Suriname => "Suriname",
        Swaziland => "Swaziland",
        Sweden => "Sweden",
        Switzerland => "Switzerland",
        Syria => "Syria",
        Taiwan => "Taiwan",
        Tajikistan => "Tajikistan",
        Tanzania => "Tanzania",
        Thailand => "Thailand",
        Togo => "Togo",
        Tonga => "Tonga",
        TrinidadAndTobago => "Trinidad and Tobago",
        Tunisia => "Tunisia",
        Turkey => "Turkey",
        Turkmenistan => "Turkmenistan",
        Tuvalu => "Tuvalu",
        Uganda => "Uganda",
        Ukraine => "Ukraine",
        UnitedArabEmirates => "United Arab Emirates",
        UnitedKingdom => "United Kingdom",
        UnitedStates => "United States",
        Uruguay => "Uruguay",
        Uzbekistan => "Uzbekistan",
        Vanuatu => "Vanuatu",
        VaticanCity => "Vatican City",
        Venezuela => "Venezuela",
        Vietnam => "Vietnam",
        Yemen => "Yemen",
        Zambia => "Zambia",
        Zimbabwe => "Zimbabwe",
    }
}

impl Direction {
    /// Returns the opposite cardinal direction.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

impl States {
    /// Returns the two-letter postal abbreviation.
    #[must_use]
    pub const fn postal_abbreviation(self) -> &'static str {
        match self {
            Self::Alabama => "AL",
            Self::Alaska => "AK",
            Self::Arizona => "AZ",
            Self::Arkansas => "AR",
            Self::California => "CA",
            Self::Colorado => "CO",
            Self::Connecticut => "CT",
            Self::Delaware => "DE",
            Self::Florida => "FL",
            Self::Georgia => "GA",
            Self::Hawaii => "HI",
            Self::Idaho => "ID",
            Self::Illinois => "IL",
            Self::Indiana => "IN",
            Self::Iowa => "IA",
            Self::Kansas => "KS",
            Self::Kentucky => "KY",
            Self::Louisiana => "LA",
            Self::Maine => "ME",
            Self::Maryland => "MD",
            Self::Massachusetts => "MA",
            Self::Michigan => "MI",
            Self::Minnesota => "MN",
            Self::Mississippi => "MS",
            Self::Missouri => "MO",
            Self::Montana => "MT",
            Self::Nebraska => "NE",
            Self::Nevada => "NV",
            Self::NewHampshire => "NH",
            Self::NewJersey => "NJ",
            Self::NewMexico => "NM",
            Self::NewYork => "NY",
            Self::NorthCarolina => "NC",
            Self::NorthDakota => "ND",
            Self::Ohio => "OH",
            Self::Oklahoma => "OK",
            Self::Oregon => "OR",
            Self::Pennsylvania => "PA",
            Self::RhodeIsland => "RI",
            Self::SouthCarolina => "SC",
            Self::SouthDakota => "SD",
            Self::Tennessee => "TN",
            Self::Texas => "TX",
            Self::Utah => "UT",
            Self::Vermont => "VT",
            Self::Virginia => "VA",
            Self::Washington => "WA",
            Self::WestVirginia => "WV",
            Self::Wisconsin => "WI",
            Self::Wyoming => "WY",
        }
    }
}

impl CanadianProvince {
    /// Returns the two-letter postal abbreviation.
    #[must_use]
    pub const fn postal_abbreviation(self) -> &'static str {
        match self {
            Self::Alberta => "AB",
            Self::BritishColumbia => "BC",
            Self::Manitoba => "MB",
            Self::NewBrunswick => "NB",
            Self::NewfoundlandAndLabrador => "NL",
            Self::NovaScotia => "NS",
            Self::Ontario => "ON",
            Self::PrinceEdwardIsland => "PE",
            Self::Quebec => "QC",
            Self::Saskatchewan => "SK",
            Self::NorthwestTerritories => "NT",
            Self::Nunavut => "NU",
            Self::Yukon => "YT",
        }
    }
}

impl CanadianProvince {
    /// Whether this entry is one of the three territories rather than a province.
    #[must_use]
    pub const fn is_territory(self) -> bool {
        matches!(
            self,
            Self::NorthwestTerritories | Self::Nunavut | Self::Yukon
        )
    }
}

impl Direction {
    /// Returns the clockwise compass bearing, with North = 0 degrees.
    #[must_use]
    pub const fn bearing_degrees(self) -> u16 {
        match self {
            Self::North => 0,
            Self::East => 90,
            Self::South => 180,
            Self::West => 270,
        }
    }
}
