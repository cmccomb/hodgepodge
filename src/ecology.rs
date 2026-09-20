//! Terrestrial habitat categories using the WWF biome convention.
#![allow(missing_docs)]

dataset_enum! {
    /// The fourteen WWF terrestrial biomes.
    /// These are broad ecological categories, not terrain shapes or all aquatic habitats.
    pub enum Biome {
        TropicalAndSubtropicalMoistBroadleafForests => "Tropical and Subtropical Moist Broadleaf Forests",
        TropicalAndSubtropicalDryBroadleafForests => "Tropical and Subtropical Dry Broadleaf Forests",
        TropicalAndSubtropicalConiferousForests => "Tropical and Subtropical Coniferous Forests",
        TemperateBroadleafAndMixedForests => "Temperate Broadleaf and Mixed Forests",
        TemperateConiferousForests => "Temperate Coniferous Forests",
        BorealForestsAndTaiga => "Boreal Forests and Taiga",
        TropicalAndSubtropicalGrasslandsSavannasAndShrublands => "Tropical and Subtropical Grasslands Savannas and Shrublands",
        TemperateGrasslandsSavannasAndShrublands => "Temperate Grasslands Savannas and Shrublands",
        FloodedGrasslandsAndSavannas => "Flooded Grasslands and Savannas",
        MontaneGrasslandsAndShrublands => "Montane Grasslands and Shrublands",
        Tundra => "Tundra",
        MediterraneanForestsWoodlandsAndScrub => "Mediterranean Forests Woodlands and Scrub",
        DesertsAndXericShrublands => "Deserts and Xeric Shrublands",
        Mangroves => "Mangroves",
    }
}
