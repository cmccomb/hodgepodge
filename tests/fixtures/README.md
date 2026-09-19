# Dataset references

These fixtures are checked in so tests run offline. Review changes against the
reference source; do not regenerate expected values from the Rust enums.

- `css-colors.tsv`: all 148 opaque names and RGB values from the named-color table
  in [CSS Color 4, September 13, 2026](https://www.w3.org/TR/2026/CRD-css-color-4-20260913/#named-colors).
  Extracted from that table on September 19, 2026, sorted by name. Values retain
  aliases with identical RGB values. `transparent` and `currentcolor` are outside
  this table and outside the enum.
- `eu-members.txt`: the 27 members in the
  [European Union country directory](https://european-union.europa.eu/principles-countries-history/eu-countries_en),
  checked September 19, 2026. Spaces are removed for Rust identifiers; the
  existing API names `RepublicOfCyprus` and `CzechRepublic` represent Cyprus and
  Czechia. Membership is a dated snapshot, not an automatically updated registry.

The tests check every fixture entry with default features. With `strum`, they
also compare the complete enumerated sets to reject missing or extra entries.
