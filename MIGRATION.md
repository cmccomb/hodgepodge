# Migrating from 0.3 to 0.4

Version 0.4 is a breaking dataset release. The minimum Rust version remains 1.71.

## Update planet membership and prefix matches

`Planet` now contains the eight planets from Mercury to Neptune. `Planet::Pluto`
has been removed: Pluto is classified as a dwarf planet. Remove that match arm
and explicitly migrate any stored `"Pluto"` records to your application's dwarf
planet representation before deserializing with 0.4. Parsing or deserializing
`"Pluto"` as `Planet` returns an error; there is no automatic replacement.

`PrefixLarge` adds `Ronna` and `Quetta`; `PrefixSmall` adds `Ronto` and `Quecto`.
Update exhaustive matches. Existing numeric discriminants are preserved,
including **positive** small-prefix magnitudes. Use `exponent()` for signed
powers of ten:

```rust
use hodgepodge::{PrefixLarge, PrefixSmall};
assert_eq!(PrefixSmall::Milli as u8, 3);
assert_eq!(PrefixSmall::Milli.exponent(), -3);
assert_eq!(PrefixLarge::Quetta.exponent(), 30);
assert_eq!(PrefixSmall::Quecto.symbol(), "q");
```

## Choose names or labels deliberately

Every enum now provides `ALL`, `COUNT`, and `label()` without features, plus the
shared `Dataset` trait. `strum` traits remain supported. If importing `EnumCount`
solely for `Type::COUNT`, the import may now be unused; remove it or explicitly
write `<Type as EnumCount>::COUNT` when testing that integration.

`Display`, `as_str()`, `FromStr`, and JSON serialization retain canonical variant names.
`label()` is presentation text and is not a parsing alias. Adding new root
exports can make wildcard imports ambiguous with names from other crates;
qualify those names or import individual types.

```rust
use hodgepodge::States;
assert_eq!(States::NewYork.label(), "New York");
assert_eq!(States::NewYork.as_str(), "NewYork");
assert_eq!(States::COUNT, 50);
assert_eq!(States::ALL.len(), 50);
```

The new optional `rand` feature integrates with rand **0.9**, not rand 0.8 or
0.10. Supply your own compatible RNG. Uniform enum sampling means uniform
variants, not observed frequencies in nature; repeated samples use replacement.
For a hand of unique cards, consume `shuffled_deck()` instead of repeatedly
sampling `Card`. Reproducibility requires fixed RNG and dependency versions.

See [dataset scope](DATASETS.md) for the 206-bone convention, selected muscles,
standard amino-acid alphabet, and versioned geologic chart.

## Optional taxonomy and data attribution

`TaxonomicRank` keeps its existing eight variants. The separate `taxonomy`
feature introduces `taxonomy::Taxon` records with IDs and parent/child navigation.
It also provides one `taxonomy::Species` enum for every selected species, with
the same `Dataset`, parsing, Serde, strum, and rand contracts as the compact enums.
For example, `Species::PantheraLeo` and
`animalia::chordata::mammalia::carnivora::felidae::panthera::Leo` identify the same
variant. Use `.taxon()` to access source metadata, and group-module `TAXON`
constants for classification groups. See [naming rules](DATASETS.md#generated-species-names-and-paths).

Additional source ranks are preserved as strings; `rank()` returns `None` for
ranks outside the introductory enum. Taxon handles do not implement the enum
`Dataset`, Serde, or random-sampling contracts. Persist the COL ID together with
the source release version. Species discriminants and declaration order are
snapshot-specific; do not persist numeric casts or binary enum indexes across
releases. `Taxon::key()` stores the COL ID together with its source version and
resolves only against that exact release. Its JSON/binary Serde representation
contains strings, independent of `Species` ordering.

The software license remains MIT OR Apache-2.0. Because the package includes
Catalogue of Life classification data, its combined license expression is now
`(MIT OR Apache-2.0) AND CC-BY-4.0`. Review the bundled attribution notice if your
application redistributes the taxonomy. The feature gates compilation of the
data, not its presence in the source package.

---

# Migrating from 0.2 to 0.3

Version 0.3 corrects dataset values and names and gives every enum the same basic
API. It is a breaking release. Update the dependency to `hodgepodge = "0.3"`
and use Rust 1.71 or newer.

## Replace color casts with `rgb()`

In 0.2, color discriminants were used as RGB values. CSS aliases could not share
discriminants, so the crate assigned them altered colors. In 0.3, every color
enum separates variant identity from its color value.

```rust,ignore
// 0.2
let rgb = CSS::Aqua as u32; // 0x0100ff: incorrect for aqua
```

```rust
use hodgepodge::CSS;

let rgb: u32 = CSS::Aqua.rgb();
assert_eq!(rgb, 0x00ffff);
assert_eq!(CSS::Aqua.rgb(), CSS::Cyan.rgb());
assert_ne!(CSS::Aqua, CSS::Cyan);
```

This applies to **CSS, RGB, CMYK, and ROYGBIV**. Integer casts still compile, but
their values are enum identifiers, not RGB values. Do not persist those
identifiers. `rgb()` is a `const fn` returning `u32` and is suitable for constants
and RGB comparisons. To compare visual colors, compare `rgb()` results; enum
equality compares names.

Existing `format!("{color:x}")` calls continue producing six-digit hex strings.
Alias output is now exact. Alternate formatting (`{color:#x}`) adds `0x`, and
width and alignment are honored.

## Update corrected variant names

| 0.2 name | 0.3 replacement |
| --- | --- |
| `Continent::Antartica` | `Continent::Antarctica` |
| `CSS::Fuschia` | `CSS::Fuchsia` |
| `EU::SpainAndSweden` | `EU::Spain` and `EU::Sweden` |

Update exhaustive matches accordingly. The EU dataset now has 27 members.
There is no automatic replacement for a combined Spain/Sweden record: select
the intended country or split the record using application context.

JSON stores variant names. Binary Serde formats may instead store variant indexes. Migrate previously stored `"Antartica"` and
`"Fuschia"` strings and resolve `"SpainAndSweden"` records before deserializing
them with 0.3. Other variant strings remain unchanged; color aliases retain
their distinct names even when their RGB values agree.

## Use the common API

All 28 enum types now implement `Copy`, `Clone`, `PartialEq`, `Eq`, `Hash`,
`Display`, and `FromStr`, without enabling features. `as_str()` returns a static
variant name. These names contain no inserted spaces: `NorthAmerica` stays
`NorthAmerica`.

```rust
use hodgepodge::{Continent, ParseEnumError};

fn main() -> Result<(), ParseEnumError> {
    let continent: Continent = "northamerica".parse()?;
    assert_eq!(continent.as_str(), "NorthAmerica");
    assert_eq!(continent.to_string(), "NorthAmerica");
    Ok(())
}
```

Parsing accepts ASCII case differences but rejects surrounding whitespace,
abbreviations, numeric values, and inserted separators. Errors use the shared
`ParseEnumError` type. Serde's separate name format remains case-sensitive.

## Use the additional datasets

The following enums were merged after 0.2.0 and are included in 0.3.0:

| Module | Enums |
| --- | --- |
| `time` | `Season`, `Quarter` |
| `games` | `DiceFace`, `ChessPiece` |
| `geography` | `Ocean`, `CanadianProvince` |

All are also available at the crate root. `CanadianProvince` includes the ten
provinces and three territories. Seasons use a Northern Hemisphere convention.
`ChessPiece::ordinal()` reports a list position, not a material point value.

## Compiler and feature compatibility

The minimum Rust version increases from 1.66.1 to 1.71 to match the currently
resolved optional derive and development dependencies. CI exercises this
minimum and newer toolchains. The default library still has zero dependencies.
The `strum`, `serde`, `enum-iter`, and `enum-count` feature names are retained.

Other dataset membership and numeric conventions are unchanged in this release.
See [dataset scope](DATASETS.md) for the retained historical and illustrative
lists before using them as current reference data.

## Additive relationship APIs

The new `INFO` descriptor is part of `Dataset`; downstream custom implementations
must provide it. All library implementations do so. Canonical enum names and
existing external-code parsing rules remain unchanged; new named constructors
provide reverse code lookup. `Country` and `Unit` retain their legacy variants;
`IsoCountry` and `SiBaseQuantity`/`SiBaseUnit` provide explicitly scoped modern
views. `Season::months()` retains its original Northern Hemisphere convention.

The full importer audit now runs from a repository checkout. Two large source
input files are excluded from `.crate` packages; runtime data, attribution and
code-generation inputs remain included. See `DATASETS.md` for the package budget.
