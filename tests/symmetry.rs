use hodgepodge::*;
use std::collections::HashSet;

#[test]
fn checked_numbers_accept_exactly_the_documented_values() {
    macro_rules! check {
        ($type:ty, $method:ident) => {
            for number in u8::MIN..=u8::MAX {
                let expected = <$type>::ALL
                    .iter()
                    .copied()
                    .find(|value| value.$method() == number);
                let actual = <$type>::try_from(number);
                assert_eq!(actual.as_ref().ok().copied(), expected);
                if let Err(error) = actual {
                    assert_eq!(error.value(), number);
                }
            }
        };
    }
    check!(Element, atomic_number);
    check!(Planet, orbital_order);
    check!(Rank, ordinal);
    check!(Medal, place);
    check!(Ordinal, number);
    check!(Quarter, number);
    check!(EarthLayer, ordinal);
    check!(AtmosphereLayer, ordinal);
    check!(InfernoCircle, number);
    assert_eq!(Element::try_from(26), Ok(Element::Iron));
    assert_eq!(InfernoCircle::try_from(9), Ok(InfernoCircle::Treachery));
}

#[test]
fn reference_codes_have_exact_reverse_lookups() {
    for row in include_str!("fixtures/elements.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let element = Element::try_from(fields[0].parse::<u8>().unwrap()).unwrap();
        assert_eq!(Element::from_symbol(fields[1]), Some(element));
    }
    for state in States::ALL {
        assert_eq!(
            States::from_postal_abbreviation(state.postal_abbreviation()),
            Some(*state)
        );
        assert_eq!(
            States::from_postal_abbreviation(&state.postal_abbreviation().to_ascii_lowercase()),
            Some(*state)
        );
    }
    for province in CanadianProvince::ALL {
        assert_eq!(
            CanadianProvince::from_postal_abbreviation(province.postal_abbreviation()),
            Some(*province)
        );
    }
    for amino in AminoAcid::ALL {
        assert_eq!(
            AminoAcid::from_three_letter_code(amino.three_letter_code()),
            Some(*amino)
        );
        assert_eq!(
            AminoAcid::from_three_letter_code(&amino.three_letter_code().to_ascii_uppercase()),
            Some(*amino)
        );
    }
    assert_eq!(Element::from_symbol("Co"), Some(Element::Cobalt));
    assert_eq!(Element::from_symbol("CO"), None);
    for bad in ["", " CA", "CA ", "DC", "ZZ"] {
        assert_eq!(States::from_postal_abbreviation(bad), None);
    }
    for bad in ["", "Sec", "Pyl", " Ala", "Alanine", "*", "Asx"] {
        assert_eq!(AminoAcid::from_three_letter_code(bad), None);
    }
    assert!("CA".parse::<States>().is_err());
    assert!("Fe".parse::<Element>().is_err());
    assert!("Ala".parse::<AminoAcid>().is_err());
}

#[test]
fn groups_partition_the_existing_selections() {
    macro_rules! partition {
        ($parent:ty, $method:ident, $child:ty, $relation:ident) => {{
            let mut seen = HashSet::new();
            for parent in <$parent>::ALL {
                for child in parent.$method() {
                    assert_eq!(child.$relation(), *parent);
                    assert!(seen.insert(child));
                }
            }
            assert_eq!(seen, <$child>::ALL.iter().copied().collect());
        }};
    }
    partition!(GeologicEon, eras, GeologicEra, eon);
    partition!(GeologicEra, periods, GeologicPeriod, era);
    partition!(GeologicPeriod, epochs, GeologicEpoch, period);
    partition!(RockClass, rocks, Rock, class);
    partition!(BoneRegion, bones, Bone, region);
    partition!(SkeletalDivision, bones, Bone, division);
    partition!(SkeletalDivision, regions, BoneRegion, division);
    partition!(MuscleRegion, muscles, Muscle, region);
    assert_eq!(GeologicEon::Hadean.eras().count(), 0);
    assert_eq!(SkeletalDivision::Axial.bones().count(), 80);
    assert_eq!(SkeletalDivision::Appendicular.bones().count(), 126);
}

#[test]
fn mirrors_preserve_identity_and_reverse_perspective() {
    for bone in Bone::ALL {
        let opposite = bone.mirrored();
        assert_eq!(opposite.mirrored(), *bone);
        assert_eq!(opposite.side(), bone.side().opposite());
        assert_eq!(opposite.region(), bone.region());
        assert_eq!(opposite.division(), bone.division());
        assert_eq!(opposite == *bone, bone.side() == BodySide::Midline);
    }
    assert_eq!(Bone::LeftFemur.mirrored(), Bone::RightFemur);
    assert_eq!(Bone::Sternum.mirrored(), Bone::Sternum);
    for a in RockPaperScissors::ALL {
        for b in RockPaperScissors::ALL {
            assert_eq!(a.outcome_against(*b).reversed(), b.outcome_against(*a));
        }
    }
    for rank in TaxonomicRank::ALL {
        if let Some(narrower) = rank.narrower() {
            assert_eq!(narrower.broader(), Some(*rank));
        }
        if let Some(broader) = rank.broader() {
            assert_eq!(broader.narrower(), Some(*rank));
        }
    }
}

#[test]
fn reverse_colors_keep_aliases() {
    let aqua: HashSet<_> = CSS::matching_rgb(0x0000_ffff).collect();
    assert_eq!(aqua, HashSet::from([CSS::Aqua, CSS::Cyan]));
    assert_eq!(CSS::matching_rgb(0x0100_0000).count(), 0);
    for color in CSS::ALL {
        assert!(CSS::matching_rgb(color.rgb()).any(|item| item == *color));
    }
}

#[test]
fn calendars_wrap_and_respect_every_fiscal_start_and_hemisphere() {
    for day in Day::ALL {
        assert_eq!(day.next().previous(), *day);
        assert_eq!(day.previous().next(), *day);
        assert_eq!(Day::from_abbreviation(day.abbreviation()), Some(*day));
    }
    assert_eq!(Day::Sunday.next(), Day::Monday);
    for month in Month::ALL {
        assert_eq!(month.next().previous(), *month);
        assert_eq!(month.previous().next(), *month);
        assert_eq!(Month::from_abbreviation(month.abbreviation()), Some(*month));
    }
    for hemisphere in Hemisphere::ALL {
        let mut months = HashSet::new();
        for season in Season::ALL {
            for month in season.months_in(*hemisphere) {
                assert_eq!(month.season(*hemisphere), *season);
                assert!(months.insert(month));
            }
        }
        assert_eq!(months.len(), 12);
    }
    assert_eq!(
        Season::Winter.months(),
        Season::Winter.months_in(Hemisphere::Northern)
    );
    assert_eq!(Month::January.season(Hemisphere::Southern), Season::Summer);
    for start in Month::ALL {
        let mut fiscal_months = Vec::new();
        for quarter in Quarter::ALL {
            for month in quarter.months(*start) {
                assert_eq!(month.quarter(*start), *quarter);
                fiscal_months.push(month);
            }
        }
        assert_eq!(fiscal_months[0], *start);
        assert_eq!(fiscal_months.iter().collect::<HashSet<_>>().len(), 12);
        for pair in fiscal_months.windows(2) {
            assert_eq!(pair[0].next(), pair[1]);
        }
    }
    assert_eq!(
        Quarter::Q1.months(Month::July),
        [Month::July, Month::August, Month::September]
    );
    assert_eq!(Month::January.quarter(Month::July), Quarter::Q3);
    assert_eq!(Month::February.days_in_year(1900), 28);
    assert_eq!(Month::February.days_in_year(2000), 29);
    assert_eq!(Month::February.days_in_year(2024), 29);
    assert_eq!(Month::February.days_in_year(2100), 28);
    assert_eq!(
        (0..400)
            .filter(|year| is_gregorian_leap_year(*year))
            .count(),
        97
    );
    assert_eq!(
        (-400..0)
            .filter(|year| is_gregorian_leap_year(*year))
            .count(),
        97
    );
}

#[test]
fn unified_si_prefixes_and_base_units_match_reference_tables() {
    for line in include_str!("fixtures/si-prefixes.tsv").lines() {
        let row: Vec<_> = line.split('\t').collect();
        let prefix: SiPrefix = row[1].parse().unwrap();
        let exponent: i8 = row[3].parse().unwrap();
        assert_eq!(prefix.symbol(), row[2]);
        assert_eq!(prefix.exponent(), exponent);
        assert_eq!(SiPrefix::from_exponent(exponent), Some(prefix));
        assert_eq!(SiPrefix::from_symbol(row[2]), Some(prefix));
        if exponent > 0 {
            let old: PrefixLarge = row[1].parse().unwrap();
            assert_eq!(SiPrefix::from(old), prefix);
            assert_eq!(PrefixLarge::from_exponent(exponent), Some(old));
            assert_eq!(PrefixLarge::from_symbol(row[2]), Some(old));
        } else {
            let old: PrefixSmall = row[1].parse().unwrap();
            assert_eq!(SiPrefix::from(old), prefix);
            assert_eq!(PrefixSmall::from_exponent(exponent), Some(old));
            assert_eq!(PrefixSmall::from_symbol(row[2]), Some(old));
        }
    }
    for exponent in i8::MIN..=i8::MAX {
        assert_eq!(
            SiPrefix::from_exponent(exponent).is_some(),
            SiPrefix::ALL.iter().any(|p| p.exponent() == exponent)
        );
    }
    assert_eq!(SiPrefix::from_symbol("M"), Some(SiPrefix::Mega));
    assert_eq!(SiPrefix::from_symbol("m"), Some(SiPrefix::Milli));
    for bad in ["", "u", "μ", " K", "K", "µ "] {
        assert_eq!(SiPrefix::from_symbol(bad), None);
    }
    for line in include_str!("fixtures/si-base-units.tsv").lines() {
        let row: Vec<_> = line.split('\t').collect();
        let quantity: SiBaseQuantity = row[0].parse().unwrap();
        let unit: SiBaseUnit = row[1].parse().unwrap();
        assert_eq!(quantity.unit(), unit);
        assert_eq!(unit.quantity(), quantity);
        assert_eq!(unit.symbol(), row[2]);
        assert_eq!(SiBaseUnit::from_symbol(row[2]), Some(unit));
    }
    assert_eq!(Unit::Volume.base_quantity(), None);
    assert_eq!(
        Unit::Temperature.base_quantity(),
        Some(SiBaseQuantity::ThermodynamicTemperature)
    );
}

#[test]
fn every_standard_codon_matches_ncbi_and_reverse_mappings() {
    let mut seen = HashSet::new();
    let mut stop_count = 0;
    for line in include_str!("fixtures/standard-genetic-code.tsv").lines() {
        let row: Vec<_> = line.split('\t').collect();
        let codon = Codon::from_sequence(row[0]).unwrap();
        assert!(seen.insert(codon));
        assert_eq!(codon.sequence(), row[0]);
        assert_eq!(Codon::from_bases(codon.bases()), codon);
        assert_eq!(codon.reverse_complement().reverse_complement(), codon);
        assert_eq!(codon.is_standard_start(), row[2] == "M");
        let meaning = if row[1] == "*" {
            stop_count += 1;
            CodonMeaning::Stop
        } else {
            CodonMeaning::AminoAcid(AminoAcid::try_from(row[1].chars().next().unwrap()).unwrap())
        };
        assert_eq!(codon.standard_meaning(), meaning);
    }
    assert_eq!(seen.len(), 64);
    assert_eq!(stop_count, 3);
    assert_eq!(
        AminoAcid::ALL
            .iter()
            .map(|a| a.standard_codons().count())
            .sum::<usize>(),
        61
    );
    assert_eq!(
        Codon::Aug.standard_meaning(),
        CodonMeaning::AminoAcid(AminoAcid::Methionine)
    );
    assert_eq!(
        Codon::Uug.standard_meaning(),
        CodonMeaning::AminoAcid(AminoAcid::Leucine)
    );
    assert!(Codon::Uug.is_standard_start());
    for a in RnaBase::ALL {
        assert_eq!(a.complement().complement(), *a);
        assert_eq!(a.coding_dna().coding_rna(), *a);
        assert_eq!(RnaBase::try_from(a.symbol()), Ok(*a));
        for b in RnaBase::ALL {
            for c in RnaBase::ALL {
                assert_eq!(Codon::from_bases([*a, *b, *c]).bases(), [*a, *b, *c]);
            }
        }
    }
    for bad in ["", "ATG", "NNN", "AU", "AUGG", " AUG", "AUG ", "ÅUG"] {
        assert_eq!(Codon::from_sequence(bad), None);
    }
    assert!(RnaBase::try_from('T').is_err());
}

#[test]
fn iso_codes_match_sources_and_geography_bridges_preserve_aliases() {
    let mut codes = HashSet::new();
    let mut alpha3 = HashSet::new();
    let mut numeric = HashSet::new();
    for line in include_str!("fixtures/iso-countries.tsv").lines() {
        let row: Vec<_> = line.split('\t').collect();
        let country: IsoCountry = row[0].parse().unwrap();
        assert_eq!(country.label(), row[1]);
        assert_eq!(country.alpha2(), row[2]);
        assert_eq!(country.alpha3(), row[3]);
        assert_eq!(country.numeric_code(), row[4]);
        assert_eq!(
            IsoCountry::from_alpha2(&row[2].to_ascii_lowercase()),
            Some(country)
        );
        assert_eq!(IsoCountry::from_alpha3(row[3]), Some(country));
        assert_eq!(IsoCountry::from_numeric_code(row[4]), Some(country));
        assert!(codes.insert(row[2]));
        assert!(alpha3.insert(row[3]));
        assert!(numeric.insert(row[4]));
    }
    assert_eq!(codes.len(), 249);
    assert_eq!(codes.len(), IsoCountry::COUNT);
    assert_eq!(
        IsoCountry::from_alpha2("US"),
        Some(IsoCountry::UnitedStates)
    );
    assert_eq!(
        IsoCountry::from_numeric_code("004"),
        Some(IsoCountry::Afghanistan)
    );
    assert_eq!(IsoCountry::from_numeric_code("4"), None);
    assert_eq!(IsoCountry::from_alpha2("XK"), None);
    assert_eq!(IsoCountry::from_alpha2("UK"), None);
    assert_eq!(IsoCountry::from_alpha2(" US"), None);
    assert_eq!(Country::Burma.iso_country(), Country::Myanmar.iso_country());
    assert_eq!(IsoCountry::Myanmar.legacy_countries().count(), 2);
    assert_eq!(Country::Kosovo.iso_country(), None);
    for legacy in Country::ALL {
        if let Some(iso) = legacy.iso_country() {
            assert!(iso.legacy_countries().any(|value| value == *legacy));
        } else {
            assert_eq!(*legacy, Country::Kosovo);
        }
    }
    for member in EU::ALL {
        assert_eq!(member.iso_country().eu_member(), Some(*member));
        assert_eq!(member.country().iso_country(), Some(member.iso_country()));
        assert_eq!(Country::from(*member), member.country());
    }
    assert_eq!(EU::Ireland.country(), Country::RepublicOfIreland);
    assert_eq!(EU::RepublicOfCyprus.iso_country().alpha2(), "CY");
    assert_eq!(EU::CzechRepublic.iso_country().alpha2(), "CZ");
    assert_eq!(IsoCountry::UnitedStates.states().count(), 50);
    assert_eq!(IsoCountry::Canada.canadian_provinces().count(), 13);
    assert_eq!(IsoCountry::Canada.states().count(), 0);
    assert_eq!(Country::UnitedStates.states().count(), 50);
    for state in States::ALL {
        assert_eq!(state.country(), Country::UnitedStates);
    }
    for province in CanadianProvince::ALL {
        assert_eq!(province.iso_country(), IsoCountry::Canada);
    }
}
