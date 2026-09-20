[![CI](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml/badge.svg)](https://github.com/cmccomb/hodgepodge/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hodgepodge.svg)](https://crates.io/crates/hodgepodge)
[![docs.rs](https://docs.rs/hodgepodge/badge.svg)](https://docs.rs/hodgepodge)

# hodgepodge

Ready-made Rust datasets for teaching, prototypes, simulations, and test fixtures.
**56 compact enums** cover colors, chemistry, anatomy, biology, geography, geologic
time, calendars, and games. Optional taxonomy adds **80,868 animal species**.

Rust **1.71+**. No default dependencies.

## Quick start

```toml
[dependencies]
hodgepodge = "0.4"
```

```rust
use hodgepodge::{Bone, Codon, Element, IsoCountry, States};

assert_eq!(Element::from_symbol("Fe"), Some(Element::Iron));
assert_eq!(Bone::COUNT, 206);
assert_eq!(Bone::LeftFemur.mirrored(), Bone::RightFemur);
assert_eq!(States::NewYork.postal_abbreviation(), "NY");
assert_eq!(IsoCountry::Canada.canadian_provinces().count(), 13);
assert_eq!(Codon::Aug.sequence(), "AUG");

for element in Element::ALL {
    println!("{}: {}", element.label(), element.atomic_number());
}
```

Every enum dataset supports comparison, hashing, display, parsing, `ALL`, `COUNT`,
`label()`, and source/scope metadata through `INFO`. Parsing accepts canonical
variant names without spaces, ignoring ASCII case; named constructors handle
external codes. Relationships connect bones to regions, epochs to periods,
codons to amino acids, countries to subdivisions, and more.

## Optional features

| Feature | Adds |
| --- | --- |
| `serde` | Serialization for dataset enums, cards, codon meanings, and taxonomy keys |
| `rand` | Uniform sampling and deck shuffling with a caller-supplied rand 0.9 RNG |
| `strum` | Iterator/count traits; also enabled by legacy `enum-iter` and `enum-count` |
| `taxonomy` | Bundled animal records and nested species paths; no runtime network access |

Enable features with `hodgepodge = { version = "0.4", features = ["taxonomy"] }`.

## Animal taxonomy

```rust
# #[cfg(feature = "taxonomy")]
# {
use hodgepodge::taxonomy::{animalia, Species};
use animalia::chordata::mammalia::carnivora::felidae::panthera;

let lion = panthera::Leo;
assert_eq!(lion, Species::PantheraLeo);
assert_eq!(lion.taxon().parent(), Some(panthera::TAXON));
println!("{}", lion.taxon().wikipedia_url().unwrap());
# }
```

This dated selection includes every source ancestor, not every animal species.
Taxonomy adds substantial compile/documentation cost; its source files remain in
the download even when disabled. For persistent taxonomy identity, use
`Taxon::key()`; serialized enum names/indexes may change between snapshots.

## More

- [API reference](https://docs.rs/hodgepodge) and [usage guide](https://github.com/cmccomb/hodgepodge/blob/v0.4.1/GUIDE.md)
- [Dataset scope and sources](https://github.com/cmccomb/hodgepodge/blob/v0.4.1/DATASETS.md)
- [Migration guide](https://github.com/cmccomb/hodgepodge/blob/v0.4.1/MIGRATION.md) and [changelog](https://github.com/cmccomb/hodgepodge/blob/v0.4.1/CHANGELOG.md)
- [Runnable examples](https://github.com/cmccomb/hodgepodge/tree/v0.4.1/examples) and [contributing](https://github.com/cmccomb/hodgepodge/blob/v0.4.1/CONTRIBUTING.md)

Software: **MIT OR Apache-2.0**. Taxonomy classification: **CC BY 4.0**;
Wikidata enrichment: **CC0**. Retain the bundled [attribution](https://github.com/cmccomb/hodgepodge/blob/v0.4.1/data/taxonomy/NOTICE.md).
