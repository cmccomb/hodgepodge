# Hodgepodge guide

Install with `hodgepodge = "0.4"`; see the [README](README.md) for optional features.

## Browse values and metadata

`as_str()` and `Display` return the canonical Rust variant name. `label()` returns
presentation text with word boundaries. `FromStr` ignores ASCII case but accepts
only complete canonical names, without surrounding whitespace or inserted spaces.
Labels, numeric strings, and postal abbreviations are not parsing aliases.

```rust
use hodgepodge::{Bone, CanadianProvince, Element, States};

assert_eq!(Bone::COUNT, 206);
assert_eq!(States::NewYork.label(), "New York");
assert_eq!(States::NewYork.postal_abbreviation(), "NY");
assert_eq!("newyork".parse::<States>()?, States::NewYork);
assert_eq!(CanadianProvince::Nunavut.postal_abbreviation(), "NU");
assert!(CanadianProvince::Nunavut.is_territory());
for element in Element::ALL {
    println!("{} ({}) — {}", element.label(), element.symbol(), element.atomic_number());
}
# Ok::<(), hodgepodge::ParseEnumError>(())
```

The `Dataset` trait exposes this common metadata to generic code:

```rust
use hodgepodge::{Dataset, Month};

fn labels<T: Dataset>() -> Vec<&'static str> {
    T::ALL.iter().map(|value| value.label()).collect()
}
assert_eq!(labels::<Month>()[0], "January");
assert_eq!(Month::try_from(12_u8)?, Month::December);
assert!(Month::try_from(0_u8).is_err());
# Ok::<(), hodgepodge::EnumValueError>(())
```

## Follow relationships

```rust
use hodgepodge::{AminoAcid, Bone, DnaBase, GeologicEpoch, GeologicPeriod, Rock, RockClass, SkeletalDivision};

assert_eq!(Bone::LeftFemur.division(), SkeletalDivision::Appendicular);
assert_eq!(DnaBase::Adenine.complement(), DnaBase::Thymine);
assert_eq!(AminoAcid::try_from('W')?, AminoAcid::Tryptophan);
assert_eq!(AminoAcid::Tryptophan.three_letter_code(), "Trp");
assert_eq!(Rock::Granite.class(), RockClass::Igneous);
assert_eq!(GeologicEpoch::Holocene.period(), GeologicPeriod::Quaternary);
# Ok::<(), hodgepodge::ParseEnumError>(())
```

## Explore the animal tree

Enable `taxonomy` to navigate **80,868 animal species across 21 phyla**, with
**100,851 taxa including their ancestors**, from the dated Catalogue of Life
selection:

```rust
# #[cfg(feature = "taxonomy")]
# {
use hodgepodge::taxonomy::{animalia, Species};
use animalia::chordata::mammalia::carnivora::felidae::panthera;

let lion = Species::PantheraLeo;
assert_eq!(lion, panthera::Leo);
assert_eq!(lion.label(), "Panthera leo");
assert_eq!(lion.taxon().parent(), Some(panthera::TAXON));

let animals: Vec<Species> = vec![panthera::Leo, Species::ApisMellifera];
assert_eq!(animals.len(), 2);
for ancestor in lion.taxon().lineage() {
    println!("{}: {}", ancestor.source_rank(), ancestor);
}
# }
```

Every species has the same `Species` type. The hierarchical names re-export real
enum variants, so they also work in `match` patterns. `Species` supports the
common `Dataset` API (`ALL`, `COUNT`, `as_str()`, and `label()`), canonical-name
parsing, and optional Serde, strum, and rand integrations. `label()` returns the
scientific name; `taxon().english_label()` provides the optional Wikidata label.
Sampling is uniform over the selected species, not over branches of the tree.

`Species::by_id("4CGXP")` supports source-ID lookup. `species.taxon()` returns
the complete record; `taxon.species()` returns `Some(Species)` for a selected
species and `None` for a group. `Taxon::all()`, `roots()`, `named()`, `children()`,
and `ancestors()` provide runtime traversal. Group modules expose `TAXON`.

Paths use the principal ranks, beginning at `animalia` or
`eukaryota::animalia`. The record retains intermediate ranks in its ancestry;
`TaxonomicRank` remains the eight-rank enum. Missing source ranks are skipped.
When a genus is absent, the leaf alias uses the full binomial, such as
`isopoda::PorcellioLaevis`. Duplicate group names gain a `_col_<id>` suffix.
See [naming and versioning rules](DATASETS.md#generated-species-names-and-paths).

The selection requires accepted, explicitly extant animal species in Catalogue
of Life 2026-09-11, an exact, unambiguous Wikidata scientific-name/ID match, and an
English Wikipedia article. Every source ancestor is retained. Children cover
the included selection, and English labels may themselves be scientific names.
[Provenance and coverage](data/taxonomy/manifest.json) document the counts,
exclusions, capture interval, and source hashes.

Export a self-contained searchable, expanding browser:

```sh
cargo run --example taxonomy_browser --features taxonomy > animals.html
```

The default build excludes the taxonomy data and generated enum. With the
feature enabled, the TSV is embedded and navigation indexes are built once on
first use; canonical-name parsing builds a sorted index on its first call.
The large enum also has a substantial build cost: a local Rust 1.93.1
`cargo check --all-features` took about 113 seconds and 1.1 GiB peak memory
on an Apple M1 with 16 GiB RAM and cached dependencies. Complete rustdoc output
is hundreds of megabytes, including a roughly 33 MiB `Species` page.
These measurements are illustrative, not performance guarantees.
The source package includes runtime taxonomy data, generated code and attribution
regardless of enabled features. Two large import-audit inputs remain in Git and
are excluded from the published crate. CI enforces an 8 MiB compressed-package
budget; full source reproduction uses a checkout of the matching Git revision.

Classification data is [CC BY 4.0](data/taxonomy/NOTICE.md); Wikidata fields are
CC0. Software remains MIT OR Apache-2.0. The package license expression includes
the data license. Keep the attribution when redistributing the classification.

## Simulate and deal cards

The optional `rand` integration uses **rand 0.9**. Every enum samples uniformly
by variant; `Card` samples uniformly over 52 suit/rank pairs. Repeated samples
are with replacement. Shuffle a complete deck and consume it to deal without
replacement. CSS aliases remain separate outcomes even when their RGB values
match. Sampling an enum does not model real-world frequencies.

For seeded examples, add `rand = "0.9"` and `rand_chacha = "0.9"` to your app.
Reproduce a run with the same RNG algorithm, seed, dependency versions, and call
sequence; the crate does not promise identical streams across future upgrades.

```rust
# #[cfg(feature = "rand")]
# {
use hodgepodge::{shuffled_deck, CoinSide, DiceFace};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

let mut rng = ChaCha8Rng::seed_from_u64(42);
let roll: DiceFace = rng.random();
let toss: CoinSide = rng.random();
let deck = shuffled_deck(&mut rng);
let hand = &deck[..5];
println!("Roll: {roll}; toss: {toss}; hand: {hand:?}");
# }
```

`standard_deck()` needs no features and returns a fixed `[Card; 52]` with no heap
allocation. It contains no jokers. Rank ordinals and chess-piece ordinals remain
list positions; `ChessPiece::material_value()` separately exposes a conventional
score with `None` for the king.

## Work with exact colors

```rust
use hodgepodge::CSS;

assert_ne!(CSS::Aqua, CSS::Cyan);
assert_eq!(CSS::Aqua.rgb(), CSS::Cyan.rgb());
assert_eq!(CSS::Tomato.rgb_channels(), [255, 99, 71]);
println!("{}: #{:x}", CSS::Tomato.label(), CSS::Tomato);
```

Use `rgb()` for packed `0xRRGGBB` values. Color discriminants are identifiers.
Hex formatting produces six lowercase digits; `#` adds Rust's `0x` prefix.

## Save and recover fixtures

With `serde_json`, enums serialize to exact canonical names, and cards to `suit`
and `rank` fields. Serde name deserialization is case-sensitive; `FromStr` is not.
Other Serde formats may encode numeric variant indexes. Enum names and indexes
can change across dataset versions; use `Taxon::key()` for versioned taxonomy
storage rather than serializing a `Species` value for use across snapshots.

```rust
use hodgepodge::{Card, Rank, Suit};

let card = Card { suit: Suit::Hearts, rank: Rank::Ace };
let json = serde_json::to_string(&card)?;
assert_eq!(json, r#"{"suit":"Hearts","rank":"Ace"}"#);
assert_eq!(serde_json::from_str::<Card>(&json)?, card);
# Ok::<(), serde_json::Error>(())
```

## Connect datasets and inspect their sources

Existing parsing still uses canonical variant names. Named code lookups keep
postal codes, element symbols and other external identifiers explicit.

```rust
use hodgepodge::{AminoAcid, Bone, Codon, CodonMeaning, Element, GeologicEon,
    Hemisphere, IsoCountry, Month, SiBaseQuantity, SiPrefix, States};

assert_eq!(Element::from_symbol("Fe"), Some(Element::Iron));
assert_eq!(States::from_postal_abbreviation("pa"), Some(States::Pennsylvania));
assert_eq!(Bone::LeftFemur.mirrored(), Bone::RightFemur);
assert_eq!(GeologicEon::Phanerozoic.eras().count(), 3);
assert_eq!(Codon::Aug.standard_meaning(), CodonMeaning::AminoAcid(AminoAcid::Methionine));
assert_eq!(SiBaseQuantity::ElectricCurrent.unit().symbol(), "A");
assert_eq!(SiPrefix::from_symbol("m"), Some(SiPrefix::Milli));
assert_eq!(IsoCountry::Canada.canadian_provinces().count(), 13);
assert_eq!(Month::January.season(Hemisphere::Southern).label(), "Summer");
assert!(!Bone::INFO.sources.is_empty());
```

Every enum dataset has `INFO`, also available through `Dataset::INFO` and the
`DATASETS` catalog. It reports scope, coverage, source references and the license
of the shipped representation. A curated selection is explicitly distinguished
from a complete table or a compatibility list.

## Runnable examples

```sh
cargo run --example explore_datasets
cargo run --example taxonomy_browser --features taxonomy > animals.html
cargo run --example dice_simulation --features rand
cargo run --example deal_cards --features rand
cargo run --example lookup
cargo run --example palette --features strum
cargo run --example fixtures --features serde
```

The dice simulation rolls two dice 10,000 times. The card example deals four
non-overlapping five-card hands. Both use seeded ChaCha8 generators.
