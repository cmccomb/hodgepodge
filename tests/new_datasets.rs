use hodgepodge::*;
use std::collections::HashSet;

#[test]
fn adult_skeleton_has_the_expected_regional_and_bilateral_counts() {
    let regions = [
        (BoneRegion::Skull, 22),
        (BoneRegion::Ear, 6),
        (BoneRegion::Neck, 1),
        (BoneRegion::VertebralColumn, 26),
        (BoneRegion::ThoracicCage, 25),
        (BoneRegion::PectoralGirdle, 4),
        (BoneRegion::UpperLimb, 6),
        (BoneRegion::Hand, 54),
        (BoneRegion::PelvicGirdle, 2),
        (BoneRegion::LowerLimb, 8),
        (BoneRegion::Foot, 52),
    ];
    assert_eq!(Bone::COUNT, 206);
    for (region, count) in regions {
        assert_eq!(
            Bone::ALL.iter().filter(|b| b.region() == region).count(),
            count
        );
    }
    for (side, count) in [
        (BodySide::Left, 86),
        (BodySide::Right, 86),
        (BodySide::Midline, 34),
    ] {
        assert_eq!(Bone::ALL.iter().filter(|b| b.side() == side).count(), count);
    }
    assert_eq!(
        Bone::ALL
            .iter()
            .filter(|b| b.division() == SkeletalDivision::Axial)
            .count(),
        80
    );
    assert_eq!(
        Bone::ALL
            .iter()
            .filter(|b| b.division() == SkeletalDivision::Appendicular)
            .count(),
        126
    );
    for side in ["Left", "Right"] {
        for number in 1..=12 {
            let rib: Bone = format!("{side}Rib{number}").parse().unwrap();
            assert_eq!(rib.region(), BoneRegion::ThoracicCage);
        }
        for limb in ["Hand", "Foot"] {
            assert!(format!("{side}{limb}MiddlePhalanx1")
                .parse::<Bone>()
                .is_err());
            for digit in 2..=5 {
                assert!(format!("{side}{limb}MiddlePhalanx{digit}")
                    .parse::<Bone>()
                    .is_ok());
            }
        }
    }
}

#[test]
fn anatomy_matches_reviewed_reference_inventory() {
    let mut bones = HashSet::new();
    for row in include_str!("fixtures/bones.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let bone: Bone = fields[0].parse().unwrap();
        assert_eq!(bone.region().as_str(), fields[1]);
        assert_eq!(bone.side().as_str(), fields[2]);
        assert!(bones.insert(bone));
    }
    assert_eq!(bones, Bone::ALL.iter().copied().collect());
    let mut muscles = HashSet::new();
    for row in include_str!("fixtures/muscles.tsv").lines() {
        let (name, region) = row.split_once('\t').unwrap();
        let muscle: Muscle = name.parse().unwrap();
        assert_eq!(muscle.region().as_str(), region);
        assert!(muscles.insert(muscle));
    }
    assert_eq!(muscles.len(), 92);
    assert_eq!(muscles, Muscle::ALL.iter().copied().collect());
}

#[test]
fn dna_base_symbols_and_complements_are_exact() {
    for (name, symbol, complement) in [
        (DnaBase::Adenine, 'A', DnaBase::Thymine),
        (DnaBase::Cytosine, 'C', DnaBase::Guanine),
        (DnaBase::Guanine, 'G', DnaBase::Cytosine),
        (DnaBase::Thymine, 'T', DnaBase::Adenine),
    ] {
        assert_eq!(name.symbol(), symbol);
        assert_eq!(DnaBase::try_from(symbol.to_ascii_lowercase()), Ok(name));
        assert_eq!(name.complement(), complement);
        assert_eq!(name.complement().complement(), name);
    }
    for c in ['U', 'N', 'X', '-', ' ', 'é'] {
        assert!(DnaBase::try_from(c).is_err());
    }
    assert!("A".parse::<DnaBase>().is_err()); // Canonical names remain distinct from symbols.
}

#[test]
fn amino_acids_match_both_reference_codes_and_reject_nonstandard_codes() {
    let mut seen = HashSet::new();
    let mut codes = HashSet::new();
    for row in include_str!("fixtures/amino-acids.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let amino: AminoAcid = fields[0].parse().unwrap();
        let code = fields[2].chars().next().unwrap();
        assert_eq!(amino.three_letter_code(), fields[1]);
        assert_eq!(amino.one_letter_code(), code);
        assert_eq!(AminoAcid::try_from(code), Ok(amino));
        assert_eq!(AminoAcid::try_from(code.to_ascii_lowercase()), Ok(amino));
        assert!(seen.insert(amino));
        assert!(codes.insert(code));
    }
    assert_eq!(seen.len(), 20);
    assert_eq!(seen, AminoAcid::ALL.iter().copied().collect());
    for c in ['B', 'J', 'O', 'U', 'X', 'Z', '*', '-', 'é'] {
        assert!(AminoAcid::try_from(c).is_err());
    }
}

#[test]
fn geology_hierarchy_matches_the_versioned_chart() {
    let mut eras = HashSet::new();
    let mut periods = HashSet::new();
    let mut epochs = HashSet::new();
    for row in include_str!("fixtures/geologic-time.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        match fields[0] {
            "GeologicEra" => {
                let value: GeologicEra = fields[1].parse().unwrap();
                assert_eq!(value.eon().as_str(), fields[2]);
                assert!(eras.insert(value));
            }
            "GeologicPeriod" => {
                let value: GeologicPeriod = fields[1].parse().unwrap();
                assert_eq!(value.era().as_str(), fields[2]);
                assert!(periods.insert(value));
            }
            "GeologicEpoch" => {
                let value: GeologicEpoch = fields[1].parse().unwrap();
                assert_eq!(value.period().as_str(), fields[2]);
                assert!(epochs.insert(value));
            }
            _ => panic!("unknown reference rank"),
        }
    }
    assert_eq!(eras.len(), 10);
    assert_eq!(periods.len(), 22);
    assert_eq!(epochs.len(), 38);
    assert_eq!(eras, GeologicEra::ALL.iter().copied().collect());
    assert_eq!(periods, GeologicPeriod::ALL.iter().copied().collect());
    assert_eq!(epochs, GeologicEpoch::ALL.iter().copied().collect());
    assert_eq!(
        GeologicEpoch::Holocene.period().era().eon(),
        GeologicEon::Phanerozoic
    );
    assert!(GeologicEra::ALL
        .iter()
        .all(|era| era.eon() != GeologicEon::Hadean));
    assert!("Anthropocene".parse::<GeologicEpoch>().is_err());
}

#[test]
fn biomes_and_rock_classes_match_reference_lists() {
    let biomes: HashSet<Biome> = include_str!("fixtures/biomes.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(biomes.len(), 14);
    assert_eq!(biomes, Biome::ALL.iter().copied().collect());
    let mut rocks = HashSet::new();
    for row in include_str!("fixtures/rocks.tsv").lines() {
        let (name, class) = row.split_once('\t').unwrap();
        let rock: Rock = name.parse().unwrap();
        assert_eq!(rock.class().as_str(), class);
        assert!(rocks.insert(rock));
    }
    assert_eq!(rocks.len(), 21);
    assert_eq!(rocks, Rock::ALL.iter().copied().collect());
}

#[test]
fn ordered_layers_and_literary_circles_match_their_conventions() {
    assert_eq!(
        EarthLayer::ALL,
        &[
            EarthLayer::Crust,
            EarthLayer::Mantle,
            EarthLayer::OuterCore,
            EarthLayer::InnerCore
        ]
    );
    assert_eq!(
        AtmosphereLayer::ALL,
        &[
            AtmosphereLayer::Troposphere,
            AtmosphereLayer::Stratosphere,
            AtmosphereLayer::Mesosphere,
            AtmosphereLayer::Thermosphere,
            AtmosphereLayer::Exosphere
        ]
    );
    assert_eq!(
        InfernoCircle::ALL,
        &[
            InfernoCircle::Limbo,
            InfernoCircle::Lust,
            InfernoCircle::Gluttony,
            InfernoCircle::Greed,
            InfernoCircle::Anger,
            InfernoCircle::Heresy,
            InfernoCircle::Violence,
            InfernoCircle::Fraud,
            InfernoCircle::Treachery
        ]
    );
    for (index, value) in EarthLayer::ALL.iter().enumerate() {
        assert_eq!(usize::from(value.ordinal()), index + 1);
    }
    for (index, value) in AtmosphereLayer::ALL.iter().enumerate() {
        assert_eq!(usize::from(value.ordinal()), index + 1);
    }
    for (index, value) in InfernoCircle::ALL.iter().enumerate() {
        assert_eq!(usize::from(value.number()), index + 1);
    }
}
