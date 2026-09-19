# Dataset scope and maintenance

`hodgepodge` supplies small datasets for code examples and fixtures. The API makes
their values convenient to use; the following scope determines what those values
mean. Version 0.3 verifies CSS colors and EU membership against independent
sources and preserves the other lists with explicit teaching conventions.

For current scientific references, see [NASA's planet classification](https://science.nasa.gov/solar-system/planets/)
and [BIPM's SI prefixes](https://www.bipm.org/en/measurement-units/si-prefixes).
The retained `Planet` and prefix enums intentionally differ from those current lists.

| Dataset | Scope and numeric meaning |
| --- | --- |
| `CSS` | All 148 opaque CSS named colors. `rgb()` is exact; aliases share values. `transparent` and `currentcolor` are excluded. |
| `RGB`, `CMYK`, `ROYGBIV` | RGB swatches for familiar palettes. CMYK names represent RGB swatches, not an ink-conversion model. Rainbow swatches are a chosen palette, not unique physical wavelengths. |
| `EU` | The 27 EU members as of September 2026. Existing names such as `CzechRepublic` remain API identifiers. |
| `Continent` | The seven-continent teaching convention, using Australia as the continent name. |
| `Direction`, `States` | Four cardinal directions and the 50 US states; discriminants are list positions. |
| `Ocean`, `CanadianProvince` | Five named oceans; ten Canadian provinces and three territories. |
| `Country` | A legacy illustrative list with historical names and aliases, including both Myanmar and Burma. It is not an ISO registry or a unique count of sovereign states. |
| `Element` | 118 elements, with atomic numbers as discriminants. `Wolfram` is the identifier for tungsten. |
| `Planet` | The original historical nine-body list including Pluto; discriminants reflect that order. It is not the current eight-planet classification. |
| `PrefixLarge`, `PrefixSmall` | The pre-2022 SI prefix set through yotta and yocto. Small-prefix discriminants are positive exponent magnitudes (milli = 3 means 10^-3). |
| `Unit`, `TaxonomicRank` | Physical quantity categories and an eight-rank introductory taxonomy. `Unit` is not the seven SI base units. |
| `Day`, `Month` | Monday-first weekdays and January-first months, numbered from one. |
| `Season`, `Quarter` | Northern Hemisphere seasonal order and four numbered fiscal quarters. No fiscal start month or season-to-date mapping is implied. |
| `Suit`, `Rank`, `DiceFace`, `ChessPiece` | Card suits, ace-first rank positions, die pips, and chess piece list positions. Rank and chess ordinals are not game-specific scores. |
| `Medal`, `Ordinal`, `BetterThanRust` | Three medals, English ordinals 1–30, and the original programming-language joke. |

## Reference checks

The [fixture notes](tests/fixtures/README.md) record the exact CSS specification
and EU directory used for tests. Every entry is tested with default features;
`strum` tests also compare the complete sets. Name and JSON round trips cover
every variant of every enum when their relevant features are enabled.

## Future data refreshes

The maintainers own future updates to the legacy country, planet, and SI-prefix
lists. They are deliberately deferred from this release because changing their
classification or membership requires a separately documented source and
migration decision. Revisit each when a consumer needs current reference data
or when planning the next breaking dataset release. Do not describe those lists
as current registries in the meantime.

For any membership change, record the source, retrieval date, naming convention,
and implications for exhaustive matches and stored Serde names. Preserve
reference fixtures independently of the implementation. Numeric values without
an explicit documented meaning should be treated as enum identifiers.
