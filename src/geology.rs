//! Rocks, Earth structure, atmosphere layers, and geologic time.
#![allow(missing_docs)]

dataset_enum! {
    /// The three broad rock classes by formation process.
    pub enum RockClass {
        Igneous => "Igneous",
        Sedimentary => "Sedimentary",
        Metamorphic => "Metamorphic",
    }
}

dataset_enum! {
    /// A teaching selection of common rock names.
    /// Breccia here means sedimentary breccia; the term also occurs in other settings.
    /// Classification describes the listed rock, not every material with a similar name.
    pub enum Rock {
        Granite => "Granite",
        Basalt => "Basalt",
        Obsidian => "Obsidian",
        Pumice => "Pumice",
        Andesite => "Andesite",
        Rhyolite => "Rhyolite",
        Gabbro => "Gabbro",
        Sandstone => "Sandstone",
        Siltstone => "Siltstone",
        Shale => "Shale",
        Conglomerate => "Conglomerate",
        Breccia => "Breccia",
        Limestone => "Limestone",
        Chert => "Chert",
        RockSalt => "Rock Salt",
        Marble => "Marble",
        Quartzite => "Quartzite",
        Slate => "Slate",
        Phyllite => "Phyllite",
        Schist => "Schist",
        Gneiss => "Gneiss",
    }
}

impl Rock {
    /// Returns the formation class under this dataset's documented convention.
    #[must_use]
    pub const fn class(self) -> RockClass {
        match self {
            Self::Granite
            | Self::Basalt
            | Self::Obsidian
            | Self::Pumice
            | Self::Andesite
            | Self::Rhyolite
            | Self::Gabbro => RockClass::Igneous,
            Self::Sandstone
            | Self::Siltstone
            | Self::Shale
            | Self::Conglomerate
            | Self::Breccia
            | Self::Limestone
            | Self::Chert
            | Self::RockSalt => RockClass::Sedimentary,
            Self::Marble
            | Self::Quartzite
            | Self::Slate
            | Self::Phyllite
            | Self::Schist
            | Self::Gneiss => RockClass::Metamorphic,
        }
    }
}

dataset_enum! {
    /// The four-layer introductory Earth model, surface to center.
    /// It separates the core into outer and inner layers; it is not the mechanical
    /// classification into lithosphere, asthenosphere, and deeper regions.
    pub enum EarthLayer {
        Crust = 1 => "Crust",
        Mantle = 2 => "Mantle",
        OuterCore = 3 => "Outer Core",
        InnerCore = 4 => "Inner Core",
    }
}

impl EarthLayer {
    /// Returns the position in the documented order, starting at one.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        self as u8
    }
}

dataset_enum! {
    /// The five principal atmospheric layers, from the ground outward.
    /// The ionosphere overlaps these layers and is not a sixth layer in this scheme.
    pub enum AtmosphereLayer {
        Troposphere = 1 => "Troposphere",
        Stratosphere = 2 => "Stratosphere",
        Mesosphere = 3 => "Mesosphere",
        Thermosphere = 4 => "Thermosphere",
        Exosphere = 5 => "Exosphere",
    }
}

impl AtmosphereLayer {
    /// Returns the position in the documented order, starting at one.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        self as u8
    }
}

dataset_enum! {
    /// The four eons, oldest first, following ICS chart 2024/12.
    /// Hadean has no eras in this chart; Precambrian is not an additional eon.
    pub enum GeologicEon {
        Hadean => "Hadean",
        Archean => "Archean",
        Proterozoic => "Proterozoic",
        Phanerozoic => "Phanerozoic",
    }
}

dataset_enum! {
    /// The ten eras in ICS chart 2024/12, oldest first.
    pub enum GeologicEra {
        Eoarchean => "Eoarchean",
        Paleoarchean => "Paleoarchean",
        Mesoarchean => "Mesoarchean",
        Neoarchean => "Neoarchean",
        Paleoproterozoic => "Paleoproterozoic",
        Mesoproterozoic => "Mesoproterozoic",
        Neoproterozoic => "Neoproterozoic",
        Paleozoic => "Paleozoic",
        Mesozoic => "Mesozoic",
        Cenozoic => "Cenozoic",
    }
}

impl GeologicEra {
    /// Returns the parent eon in this chart.
    #[must_use]
    pub const fn eon(self) -> GeologicEon {
        match self {
            Self::Eoarchean | Self::Paleoarchean | Self::Mesoarchean | Self::Neoarchean => {
                GeologicEon::Archean
            }
            Self::Paleoproterozoic | Self::Mesoproterozoic | Self::Neoproterozoic => {
                GeologicEon::Proterozoic
            }
            Self::Paleozoic | Self::Mesozoic | Self::Cenozoic => GeologicEon::Phanerozoic,
        }
    }
}

dataset_enum! {
    /// The twenty-two periods in ICS chart 2024/12, oldest first.
    pub enum GeologicPeriod {
        Siderian => "Siderian",
        Rhyacian => "Rhyacian",
        Orosirian => "Orosirian",
        Statherian => "Statherian",
        Calymmian => "Calymmian",
        Ectasian => "Ectasian",
        Stenian => "Stenian",
        Tonian => "Tonian",
        Cryogenian => "Cryogenian",
        Ediacaran => "Ediacaran",
        Cambrian => "Cambrian",
        Ordovician => "Ordovician",
        Silurian => "Silurian",
        Devonian => "Devonian",
        Carboniferous => "Carboniferous",
        Permian => "Permian",
        Triassic => "Triassic",
        Jurassic => "Jurassic",
        Cretaceous => "Cretaceous",
        Paleogene => "Paleogene",
        Neogene => "Neogene",
        Quaternary => "Quaternary",
    }
}

impl GeologicPeriod {
    /// Returns the parent era in this chart.
    #[must_use]
    pub const fn era(self) -> GeologicEra {
        match self {
            Self::Siderian | Self::Rhyacian | Self::Orosirian | Self::Statherian => {
                GeologicEra::Paleoproterozoic
            }
            Self::Calymmian | Self::Ectasian | Self::Stenian => GeologicEra::Mesoproterozoic,
            Self::Tonian | Self::Cryogenian | Self::Ediacaran => GeologicEra::Neoproterozoic,
            Self::Cambrian
            | Self::Ordovician
            | Self::Silurian
            | Self::Devonian
            | Self::Carboniferous
            | Self::Permian => GeologicEra::Paleozoic,
            Self::Triassic | Self::Jurassic | Self::Cretaceous => GeologicEra::Mesozoic,
            Self::Paleogene | Self::Neogene | Self::Quaternary => GeologicEra::Cenozoic,
        }
    }
}

dataset_enum! {
    /// The thirty-eight Phanerozoic epochs/series in ICS chart 2024/12.
    /// Early/Late are time equivalents of Lower/Upper. Cambrian Series 2 is unnamed.
    /// Carboniferous epochs link directly to their period, skipping the
    /// Mississippian/Pennsylvanian subperiod rank. Ages and subepochs are excluded.
    pub enum GeologicEpoch {
        Terreneuvian => "Terreneuvian",
        CambrianSeries2 => "Cambrian Series 2",
        Miaolingian => "Miaolingian",
        Furongian => "Furongian",
        EarlyOrdovician => "Early Ordovician",
        MiddleOrdovician => "Middle Ordovician",
        LateOrdovician => "Late Ordovician",
        Llandovery => "Llandovery",
        Wenlock => "Wenlock",
        Ludlow => "Ludlow",
        Pridoli => "Pridoli",
        EarlyDevonian => "Early Devonian",
        MiddleDevonian => "Middle Devonian",
        LateDevonian => "Late Devonian",
        EarlyMississippian => "Early Mississippian",
        MiddleMississippian => "Middle Mississippian",
        LateMississippian => "Late Mississippian",
        EarlyPennsylvanian => "Early Pennsylvanian",
        MiddlePennsylvanian => "Middle Pennsylvanian",
        LatePennsylvanian => "Late Pennsylvanian",
        Cisuralian => "Cisuralian",
        Guadalupian => "Guadalupian",
        Lopingian => "Lopingian",
        EarlyTriassic => "Early Triassic",
        MiddleTriassic => "Middle Triassic",
        LateTriassic => "Late Triassic",
        EarlyJurassic => "Early Jurassic",
        MiddleJurassic => "Middle Jurassic",
        LateJurassic => "Late Jurassic",
        EarlyCretaceous => "Early Cretaceous",
        LateCretaceous => "Late Cretaceous",
        Paleocene => "Paleocene",
        Eocene => "Eocene",
        Oligocene => "Oligocene",
        Miocene => "Miocene",
        Pliocene => "Pliocene",
        Pleistocene => "Pleistocene",
        Holocene => "Holocene",
    }
}

impl GeologicEpoch {
    /// Returns the parent period in this chart.
    #[must_use]
    pub const fn period(self) -> GeologicPeriod {
        match self {
            Self::Terreneuvian | Self::CambrianSeries2 | Self::Miaolingian | Self::Furongian => {
                GeologicPeriod::Cambrian
            }
            Self::EarlyOrdovician | Self::MiddleOrdovician | Self::LateOrdovician => {
                GeologicPeriod::Ordovician
            }
            Self::Llandovery | Self::Wenlock | Self::Ludlow | Self::Pridoli => {
                GeologicPeriod::Silurian
            }
            Self::EarlyDevonian | Self::MiddleDevonian | Self::LateDevonian => {
                GeologicPeriod::Devonian
            }
            Self::EarlyMississippian
            | Self::MiddleMississippian
            | Self::LateMississippian
            | Self::EarlyPennsylvanian
            | Self::MiddlePennsylvanian
            | Self::LatePennsylvanian => GeologicPeriod::Carboniferous,
            Self::Cisuralian | Self::Guadalupian | Self::Lopingian => GeologicPeriod::Permian,
            Self::EarlyTriassic | Self::MiddleTriassic | Self::LateTriassic => {
                GeologicPeriod::Triassic
            }
            Self::EarlyJurassic | Self::MiddleJurassic | Self::LateJurassic => {
                GeologicPeriod::Jurassic
            }
            Self::EarlyCretaceous | Self::LateCretaceous => GeologicPeriod::Cretaceous,
            Self::Paleocene | Self::Eocene | Self::Oligocene => GeologicPeriod::Paleogene,
            Self::Miocene | Self::Pliocene => GeologicPeriod::Neogene,
            Self::Pleistocene | Self::Holocene => GeologicPeriod::Quaternary,
        }
    }
}

checked_u8_enum!(EarthLayer, ordinal);
checked_u8_enum!(AtmosphereLayer, ordinal);

group_members!(RockClass, rocks, Rock, class);
group_members!(GeologicEon, eras, GeologicEra, eon);
group_members!(GeologicEra, periods, GeologicPeriod, era);
group_members!(GeologicPeriod, epochs, GeologicEpoch, period);
