# Changelog

## 0.4.1 — 2026-09-20

- Use explicit versioned documentation links in the concise README so the guide,
  source notes, migration instructions, examples, and attribution remain reachable
  from crates.io and docs.rs as well as GitHub. Library code and datasets are unchanged.

## 0.4.0 — 2026-09-20

### Added

- Shared source/scope/license descriptors, reverse code and numeric lookups,
  parent-to-member traversal, bone mirrors, and rank/outcome reversal helpers.
- RNA bases, all 64 RNA codons and NCBI standard-code translation; seven SI base
  quantities/units, a unified 24-prefix view, and 249 ISO country/area entries.
- Explicit hemisphere/fiscal-calendar mappings, Gregorian leap-year lengths,
  calendar cycling, and modern/legacy geography bridges.
- Versioned `TaxonKey` storage that rejects mismatched source releases.
- Staged taxonomy refreshes with exception rollback and failure-injection tests.
- An 8 MiB release-package budget retaining runtime data and attribution while
  excluding two repository-only import inputs; cached, less redundant CI coverage.

- Shared `Dataset` metadata and feature-free `ALL`, `COUNT`, and human-readable
  `label()` for all 56 compact enums, with canonical parsing and format-specific Serde representations.
- Standard 52-card decks, optional seeded RNG integration for every dataset enum and
  `Card`, deck shuffling, coins, rock–paper–scissors, and round outcomes.
- The conventional 206 adult bones with side/region/division metadata, a selected
  set of 92 skeletal muscle types, DNA complements, and 20 standard amino acids.
- Fourteen terrestrial biomes, 21 rock names and their classes, Earth and atmosphere
  layers, four ranks of geologic time with parent relationships, and Dante's circles.
- Element symbols/atomic numbers, SI symbols/exponents, postal codes, compass
  opposites/bearings, checked day/month/die conversions, calendar metadata,
  RGB channels, chess material points, and further numeric helpers.
- Optional `taxonomy` feature: 80,868 animal species across 21 phyla and their
  ancestors (100,851 taxa), with stable-in-snapshot IDs,
  complete ancestral paths, intermediate ranks, labels, Wikipedia links, and
  traversal; a resumable import pipeline, source audit, and expanding browser.
- A shared `Species` enum and generated taxonomic modules that re-export its
  variants, with `Dataset`, parsing, Serde, strum, and uniform rand integration.
  Species convert to the complete `Taxon` record; group modules expose `TAXON`.
- Source fixtures, expanded feature coverage, and four runnable teaching examples.

### Changed

- Package licensing now includes CC BY 4.0 for the bundled classification data;
  software retains MIT OR Apache-2.0. Attribution and source metadata are included.
- `Planet` now contains eight planets; `Pluto` is removed.
- SI-prefix enums include ronna, quetta, ronto, and quecto.
- See `MIGRATION.md` for exhaustive matches, stored names, and optional rand 0.9 use.


## 0.3.0 — 2026-09-19

This release corrects CSS colors and EU membership and gives every enum a common
API for examples, lookup, and fixtures. See [MIGRATION.md](MIGRATION.md) before
upgrading from 0.2.

### Breaking changes

- Color enums use `rgb() -> u32` for packed RGB values. Integer discriminants
  no longer encode colors. CSS aliases now return exact, shared RGB values.
- Rename `Continent::Antartica` to `Antarctica` and `CSS::Fuschia` to `Fuchsia`.
- Replace `EU::SpainAndSweden` with separate `Spain` and `Sweden` variants.
- Increase the minimum supported Rust version from 1.66.1 to 1.71.

### Added

- Consistent `Copy`, `Clone`, `PartialEq`, `Eq`, and `Hash` implementations.
- `Display`, ASCII case-insensitive `FromStr`, `as_str()`, and `ParseEnumError`
  with no new default dependencies.
- Six previously unpublished datasets: `Season`, `Quarter`, `DiceFace`,
  `ChessPiece`, `Ocean`, and `CanadianProvince`.
- Complete examples for iteration, map lookup, and JSON fixture round trips.
- Independent reference fixtures for all CSS color values and EU members.
- Explicit dataset scope and migration documentation.

### Maintenance

- Run push CI on the actual default branch, `master`.
- Test the minimum compiler and newer toolchains across default, individual,
  combined, legacy-alias, and all-feature configurations, including doctests.
- Check documentation warnings, run teaching examples, and verify the package.
- Remove the unstable documentation lint and fix optional-feature documentation
  links so default and full documentation builds are warning-free.
- Include the Apache-2.0 license text to match the existing dual-license manifest.
- Clarify historical dataset scope, chess ordinals, season conventions, and
  misleading test descriptions without changing those retained datasets.

## 0.2.0 — 2025-11-16

Published release with 22 enum datasets, optional `strum` and `serde` support,
and dataset tests. The six enums listed above were merged after publication.
