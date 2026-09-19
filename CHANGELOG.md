# Changelog

## 0.3.0 — Unreleased

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
