[![CI](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml/badge.svg)](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hodgepodge.svg)](https://crates.io/crates/hodgepodge)
[![docs.rs](https://docs.rs/hodgepodge/badge.svg)](https://docs.rs/hodgepodge)

# hodgepodge

Ready-made Rust datasets for lessons, prototypes, simulations, and test fixtures.
Explore colors, chemistry, anatomy, geography, geologic time, and games using
49 compact enums with a consistent API and useful metadata. An optional animal
taxonomy adds a shared `Species` enum, hierarchical paths, source records,
complete ancestral paths, and Wikipedia links.

**This checkout is the unreleased 0.4.0 candidate.** The published release is
[0.3.0](https://crates.io/crates/hodgepodge/0.3.0). See the [migration guide](MIGRATION.md),
[changelog](CHANGELOG.md), and [dataset scope and sources](DATASETS.md).

Every enum supports `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`, `Display`, and
`FromStr`. `ALL`, `COUNT`, `as_str()`, and `label()` need no optional features.
The default library has no dependencies and supports Rust 1.71 or newer.

## Try this checkout

Use a path dependency while 0.4 is being prepared:

```toml
[dependencies]
hodgepodge = { path = "../hodgepodge" }
```

Add `features = ["rand"]` for caller-supplied random sampling, `serde` for
serialization, `strum` for its iterator and count traits, or `taxonomy` for the
bundled animal tree. Taxonomy adds no dependencies or runtime network access.

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
The source package includes the dataset and its compact reproduction inputs
regardless of which features a downstream app enables.

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

With `serde`, enums serialize to exact canonical names, and cards to `suit` and
`rank` fields. Serde deserialization is case-sensitive; `FromStr` is not.

```rust
use hodgepodge::{Card, Rank, Suit};

let card = Card { suit: Suit::Hearts, rank: Rank::Ace };
let json = serde_json::to_string(&card)?;
assert_eq!(json, r#"{"suit":"Hearts","rank":"Ace"}"#);
assert_eq!(serde_json::from_str::<Card>(&json)?, card);
# Ok::<(), serde_json::Error>(())
```

## Datasets

| Area | Contents and metadata |
| --- | --- |
| Anatomy | 206 individual adult bones: side, region, skeletal division; 92 selected skeletal muscle types: broad location |
| Biology | Four DNA bases: symbols and complements; 20 standard amino acids: one- and three-letter codes |
| Earth | 14 WWF terrestrial biomes; 21 rocks and three rock classes; four Earth layers; five atmosphere layers |
| Geologic time | Four eons, ten eras, 22 periods, 38 epochs; parent relationships from ICS chart 2024/12 |
| Chemistry and space | 118 elements: symbols and atomic numbers; eight planets: orbital order; all 24 SI prefixes: symbols and signed exponents |
| Geography | 50 US states and 13 Canadian provinces/territories: postal codes; EU members, continents, oceans, directions, legacy country names |
| Games | Cards and decks, dice, coins, rock–paper–scissors, chess pieces |
| Animal taxonomy (optional) | A dated Catalogue of Life tree with complete ancestry, intermediate ranks, Wikidata labels, and Wikipedia links |
| Time and miscellaneous | Weekdays, months, seasons, quarters, medals, ordinals, taxonomy, physical quantities, Dante's nine circles |
| Colors | CSS's 148 named colors, RGB, CMYK, ROYGBIV: packed RGB values and channels |

Anatomy and scientific datasets have explicit conventions; see [DATASETS.md](DATASETS.md)
for their boundaries. The legacy `Country` list is illustrative, not an ISO registry.

## Features

| Feature | Effect |
| --- | --- |
| Default | Common traits, all values, counts, labels, metadata, parsing; no dependencies |
| `strum` | `EnumIter`, `EnumCount`, and re-exported helper traits |
| `enum-iter`, `enum-count` | Compatibility names that each enable `strum` |
| `serde` | `Serialize` and `Deserialize` on every enum and `Card` |
| `rand` | rand 0.9 `StandardUniform` sampling for every enum and `Card`; deck shuffling |
| `taxonomy` | Bundled animal records and traversal; no extra dependencies; CC BY 4.0 classification data |

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

## Development

```sh
python3 -m unittest discover -s scripts -p 'test_*.py'
python3 scripts/import_taxonomy.py --check
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic
bash scripts/test-features.sh
RUSTUP_TOOLCHAIN=1.71.0 bash scripts/test-features.sh
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo package --all-features
```

The test script covers every combination of the four primary features, legacy
feature aliases, examples, and documentation. Reference fixtures run offline.

Software: MIT OR Apache-2.0. Taxonomy classification data: CC BY 4.0, with
[attribution](data/taxonomy/NOTICE.md). Wikidata fields: CC0.
