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

Serde stores variant names. Migrate previously stored `"Antartica"` and
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
