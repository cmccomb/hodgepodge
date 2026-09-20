#![cfg(feature = "taxonomy")]
use hodgepodge::taxonomy::{Species, Taxon, MANIFEST, SOURCE_DATASET, SOURCE_VERSION};
use hodgepodge::TaxonomicRank;
use std::collections::HashSet;

#[test]
fn named_paths_are_enum_variants_with_the_same_metadata() {
    use animalia::chordata::mammalia::carnivora::felidae::panthera;
    use hodgepodge::taxonomy::{animalia, eukaryota};

    const LION: Species = panthera::Leo;
    assert_eq!(LION, Species::PantheraLeo);
    assert!(matches!(LION, panthera::Leo));
    assert_eq!(LION.taxon(), Taxon::by_id("4CGXP").unwrap());
    assert_eq!(LION.label(), "Panthera leo");
    assert_eq!(LION.taxon().parent(), Some(panthera::TAXON));
    let mut animals: Vec<Species> = vec![LION];
    animals.push(Species::ApisMellifera);
    assert_eq!(animals[1].taxon().scientific_name(), "Apis mellifera");
    assert_eq!(eukaryota::animalia::TAXON, animalia::TAXON);
    assert_eq!(animalia::TAXON.parent(), Some(eukaryota::TAXON));
}

#[test]
fn named_paths_preserve_duplicate_genera_and_missing_source_ranks() {
    use arthropoda::insecta::lepidoptera::geometridae;
    use hodgepodge::taxonomy::animalia::arthropoda;

    let first = geometridae::iridopsis_col_92cgw::TAXON;
    let second = geometridae::iridopsis_col_c7ngg::TAXON;
    assert_ne!(first, second);
    assert_eq!(first.scientific_name(), second.scientific_name());
    assert_eq!(first.id(), "92CGW");
    assert_eq!(second.id(), "C7NGG");
    let woodlouse = arthropoda::malacostraca::isopoda::PorcellioLaevis.taxon();
    assert_eq!(woodlouse.id(), "4LW9G");
    assert_eq!(woodlouse.parent().unwrap().id(), "8PR7H");
    assert_eq!(woodlouse.parent().unwrap().source_rank(), "section zoology");
    assert_eq!(
        woodlouse.ancestors().find(|taxon| taxon.rank().is_some()),
        Some(arthropoda::malacostraca::isopoda::TAXON)
    );
    assert!(!woodlouse
        .ancestors()
        .any(|taxon| taxon.rank() == Some(TaxonomicRank::Genus)));
}

#[test]
fn every_species_round_trips_through_metadata_and_canonical_names() {
    assert_eq!(<Species as hodgepodge::Dataset>::COUNT, 80_868);
    assert_eq!(<Species as hodgepodge::Dataset>::ALL, Species::ALL);
    let mut records = HashSet::new();
    for &species in Species::ALL {
        let taxon = species.taxon();
        assert!(records.insert(taxon));
        assert!(taxon.is_selected_species());
        assert_eq!(taxon.species(), Some(species));
        assert_eq!(Species::from_taxon(taxon), Some(species));
        assert_eq!(Species::by_id(taxon.id()), Some(species));
        assert_eq!(species.as_str().parse(), Ok(species));
        assert_eq!(species.as_str().to_ascii_lowercase().parse(), Ok(species));
        assert_eq!(species.label(), taxon.scientific_name());
    }
    assert_eq!(records.len(), Species::COUNT);
    assert_eq!(Species::by_id("N"), None);
    assert_eq!(Species::by_id("missing"), None);
    assert!("Panthera leo".parse::<Species>().is_err());
    assert!(" PantheraLeo".parse::<Species>().is_err());
    assert!("PantheraLeo ".parse::<Species>().is_err());
    assert_eq!(format!("{:>14}", Species::PantheraLeo), "   PantheraLeo");
    assert_eq!(format!("{:?}", Species::PantheraLeo), "PantheraLeo");
}

#[cfg(feature = "strum")]
#[test]
fn species_strum_iteration_matches_the_complete_dataset() {
    use strum::{EnumCount, IntoEnumIterator};
    assert_eq!(<Species as EnumCount>::COUNT, Species::COUNT);
    assert_eq!(Species::iter().len(), Species::COUNT);
    assert!(Species::iter().eq(Species::ALL.iter().copied()));
    assert!(Species::iter().rev().eq(Species::ALL.iter().rev().copied()));
}

#[cfg(feature = "rand")]
#[test]
fn species_sampling_uses_the_uniform_species_index_not_taxonomic_branches() {
    use rand::{Rng, SeedableRng};
    let mut sampled = rand_chacha::ChaCha8Rng::seed_from_u64(42);
    let mut indexed = sampled.clone();
    for _ in 0..1_000 {
        let species: Species = sampled.random();
        assert_eq!(
            species,
            Species::ALL[indexed.random_range(0..Species::COUNT)]
        );
    }
}

#[cfg(feature = "serde")]
#[test]
fn species_serde_preserves_enum_names_and_rejects_aliases_and_payloads() {
    use serde::de::value::{EnumAccessDeserializer, Error, U32Deserializer};
    use serde::Deserialize;
    for &species in Species::ALL {
        let json = serde_json::to_string(&species).unwrap();
        assert_eq!(json, format!("\"{}\"", species.as_str()));
        assert_eq!(serde_json::from_str::<Species>(&json).unwrap(), species);
        let ordinal = U32Deserializer::<Error>::new(species as u32);
        assert_eq!(
            Species::deserialize(EnumAccessDeserializer::new(ordinal)).unwrap(),
            species
        );
    }
    for invalid in [
        "\"pantheraleo\"",
        "\"Leo\"",
        "\"Panthera leo\"",
        "123",
        "{\"PantheraLeo\":1}",
    ] {
        assert!(
            serde_json::from_str::<Species>(invalid).is_err(),
            "{invalid}"
        );
    }
    let out_of_range = U32Deserializer::<Error>::new(u32::MAX);
    assert!(Species::deserialize(EnumAccessDeserializer::new(out_of_range)).is_err());
}

#[test]
fn snapshot_counts_and_scope_match_manifest() {
    let manifest: serde_json::Value = serde_json::from_str(MANIFEST).unwrap();
    assert_eq!(manifest["col"]["dataset_key"], SOURCE_DATASET);
    assert_eq!(manifest["col"]["version"], SOURCE_VERSION);
    assert_eq!(
        manifest["counts"]["taxa"].as_u64().unwrap(),
        Taxon::count() as u64
    );
    let selected: Vec<_> = Taxon::all().filter(|t| t.is_selected_species()).collect();
    assert_eq!(
        manifest["counts"]["selected_species"].as_u64().unwrap(),
        selected.len() as u64
    );
    assert!(
        selected.len() > 10_000,
        "the import must remain substantial"
    );
    let mut entities = HashSet::new();
    for taxon in selected {
        assert_eq!(taxon.rank(), Some(TaxonomicRank::Species));
        assert_eq!(taxon.extinct(), Some(false));
        assert!(entities.insert(taxon.wikidata_id().unwrap()));
        assert!(taxon
            .wikipedia_url()
            .unwrap()
            .starts_with("https://en.wikipedia.org/wiki/"));
        assert!(taxon.ancestors().any(|t| t.id() == "N"));
    }
}

#[test]
fn every_parent_child_link_is_reciprocal_and_every_path_reaches_root() {
    let roots: Vec<_> = Taxon::roots().collect();
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].scientific_name(), "Eukaryota");
    let mut reached = HashSet::new();
    let mut pending = roots.clone();
    while let Some(parent) = pending.pop() {
        assert!(reached.insert(parent), "duplicate or cyclic tree edge");
        for child in parent.children() {
            assert_eq!(child.parent(), Some(parent));
            pending.push(child);
        }
    }
    assert_eq!(reached.len(), Taxon::count());
    for taxon in Taxon::all() {
        assert_eq!(Taxon::by_id(taxon.id()), Some(taxon));
        let path: Vec<_> = taxon.ancestors().take(64).collect();
        assert!(path.len() < 64);
        if taxon.parent().is_some() {
            assert_eq!(path.last(), roots.first());
        }
    }
}

#[test]
fn familiar_species_have_source_paths_and_intermediate_ranks() {
    let lion = Taxon::by_id("4CGXP").unwrap();
    assert_eq!(lion.scientific_name(), "Panthera leo");
    assert_eq!(lion.wikidata_id(), Some("Q140"));
    assert_eq!(lion.extinct(), Some(false));
    let lineage: Vec<_> = lion.lineage().iter().map(|t| t.scientific_name()).collect();
    for name in [
        "Eukaryota",
        "Animalia",
        "Chordata",
        "Mammalia",
        "Carnivora",
        "Felidae",
        "Panthera",
        "Panthera leo",
    ] {
        assert!(lineage.contains(&name), "missing {name}");
    }
    assert!(Taxon::named("PANTHERA LEO").any(|t| t == lion));
    assert!(Taxon::all().any(|t| t.source_rank() == "subfamily" && t.rank().is_none()));
    assert_eq!(Taxon::by_id("not-an-id"), None);
    assert_eq!(Taxon::named("not a taxon").count(), 0);
    assert_eq!(lion.children().len(), 0);
}

#[test]
fn durable_keys_reject_wrong_releases_and_unknown_ids() {
    use hodgepodge::taxonomy::{Species, TaxonKey, SOURCE_VERSION};
    let lion = Species::PantheraLeo.taxon();
    let key = lion.key();
    assert_eq!(key.resolve(), Some(lion));
    assert_eq!(key.source_version, SOURCE_VERSION);
    assert_eq!(key.col_id, lion.id());
    let wrong_release = TaxonKey {
        source_version: "1900-01-01".into(),
        ..key.clone()
    };
    assert_eq!(wrong_release.resolve(), None);
    let wrong_id = TaxonKey {
        col_id: "not-a-col-id".into(),
        ..key
    };
    assert_eq!(wrong_id.resolve(), None);
}

#[cfg(feature = "serde")]
#[test]
fn durable_keys_round_trip_in_json_and_binary_without_species_indexes() {
    use hodgepodge::taxonomy::{Species, TaxonKey};
    let key = Species::PantheraLeo.taxon().key();
    let json = serde_json::to_string(&key).unwrap();
    assert!(json.contains("source_version"));
    assert!(json.contains("col_id"));
    assert_eq!(serde_json::from_str::<TaxonKey>(&json).unwrap(), key);
    let binary = bincode::serialize(&key).unwrap();
    assert_eq!(bincode::deserialize::<TaxonKey>(&binary).unwrap(), key);
    assert_eq!(key.resolve(), Some(Species::PantheraLeo.taxon()));
    // Enum serialization itself is intentionally unchanged and format-specific.
    let species_bytes = bincode::serialize(&Species::PantheraLeo).unwrap();
    assert_eq!(species_bytes.len(), 4);
    assert_eq!(
        bincode::deserialize::<Species>(&species_bytes).unwrap(),
        Species::PantheraLeo
    );
}
