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

## References added for 0.4

Checked September 19, 2026. These files contain factual names/codes and our Rust
identifier mappings, not reproduced source prose or illustrations.

The optional animal taxonomy has separate, larger source inputs under
`data/taxonomy/`: the pinned Catalogue of Life selection, captured Wikidata
rows, contributing-checklist metadata, attribution, and a checksummed manifest.
`scripts/import_taxonomy.py --check` rebuilds the selected tree from those
inputs offline. See [taxonomy scope and update instructions](../../DATASETS.md#taxonomy-selection-and-updates).

- `elements.tsv`: atomic number, symbol, and source spelling, extracted directly
  from the [CIAAW Standard Atomic Weights 2024 table](https://ciaaw.org/atomic-weights.htm).
  No atomic weights are copied. Tests map aluminium/caesium/tungsten to the
  existing Rust identifiers Aluminum/Cesium/Wolfram.
- `si-prefixes.tsv`: [BIPM's complete SI-prefix table](https://www.bipm.org/en/measurement-units/si-prefixes),
  transcribed as enum family, name, symbol, and signed power of ten. Small-prefix
  discriminants retain their historical positive magnitudes.
- `us-states.tsv`: 50 state names and abbreviations from
  [USPS Publication 28, Appendix B](https://pe.usps.com/text/pub28/28apb.htm).
  DC, territories, possessions, and military addresses are outside `States`.
- `canada.tsv`: all 13 province/territory postal abbreviations from
  [Canada Post's addressing guidelines](https://www.canadapost-postescanada.ca/cpc/en/support/articles/addressing-guidelines/symbols-and-abbreviations.page).
- `amino-acids.tsv`: twenty standard names and one-/three-letter symbols,
  cross-checked against [NCBI sequence alphabets](https://www.ncbi.nlm.nih.gov/IEB/ToolBox/SDKDOCS/BIOSEQ.HTML).
  The larger NCBI alphabets also include special/ambiguous codes that this enum
  deliberately excludes. DNA A–T/C–G pairing follows
  [NHGRI's base-pair reference](https://www.genome.gov/genetics-glossary/Base-Pair).
- `bones.tsv`: curated individual-bone inventory, with region and anatomical side,
  following OpenStax *Anatomy and Physiology 2e*,
  [skeletal divisions](https://openstax.org/books/anatomy-and-physiology-2e/pages/7-1-divisions-of-the-skeletal-system),
  sections 7.2–7.4 (skull, spine, thoracic cage), and 8.1–8.4 (girdles and limbs).
  Names are normalized to singular CamelCase, paired entries gain Left/Right,
  and ribs/vertebrae/digits have explicit numbers. Independent regional counts,
  side counts, and absence of middle phalanges in digit 1 are tested.
- `muscles.tsv`: a curated selection of 92 named skeletal muscle types, grouped
  into broad teaching locations using OpenStax sections
  [11.3](https://openstax.org/books/anatomy-and-physiology-2e/pages/11-3-axial-muscles-of-the-head-neck-and-back),
  [11.4](https://openstax.org/books/anatomy-and-physiology-2e/pages/11-4-axial-muscles-of-the-abdominal-wall-and-thorax),
  [11.5](https://openstax.org/books/anatomy-and-physiology-2e/pages/11-5-muscles-of-the-pectoral-girdle-and-upper-limbs),
  and [11.6](https://openstax.org/books/anatomy-and-physiology-2e/pages/11-6-appendicular-muscles-of-the-pelvic-girdle-and-lower-limbs).
  English/Latin naming order is normalized (for example GemellusSuperior).
  Additional naming checks use primary anatomical studies of
  [human head muscles](https://pmc.ncbi.nlm.nih.gov/articles/PMC5389032/),
  [limb muscle networks](https://pmc.ncbi.nlm.nih.gov/articles/PMC4599883/),
  [extensor carpi radialis](https://pubmed.ncbi.nlm.nih.gov/7808722/), and
  [cervical musculature](https://pmc.ncbi.nlm.nih.gov/articles/PMC5972401/).
  This is a maintained teaching selection, not a claim of complete coverage.
- `biomes.txt`: the fourteen terrestrial biome headings in Olson and Dinerstein,
  *The Global 200: Priority Ecoregions for Global Conservation* (2002), Table 1,
  [WWF-hosted paper](https://files.worldwildlife.org/wwfcmsprod/files/Publication/file/5xdxix5fsv_The_Global_200_Priority_Ecoregions_for_Global_Conservation.pdf).
  Punctuation is removed and “Boreal forests/taiga” becomes `BorealForestsAndTaiga`.
- `rocks.tsv`: a teaching selection of names and formation classes, checked
  against National Park Service references on
  [igneous](https://www.nps.gov/subjects/geology/igneous.htm),
  [sedimentary](https://www.nps.gov/subjects/geology/sedimentary.htm), and
  [metamorphic](https://www.nps.gov/subjects/geology/metamorphic.htm) rocks,
  plus [USGS sedimentary rock examples](https://www.usgs.gov/faqs/what-are-sedimentary-rocks).
  [USGS defines sedimentary breccia](https://apps.usgs.gov/thesaurus/term-simple.php?code=2.1.6&thcode=4),
  the specific convention for `Breccia` here; its
  [evaporite reference](https://www.usgs.gov/publications/evaporite-karst-problems-and-studies-usa)
  identifies rock salt as an evaporite.
- `geologic-time.tsv`: era/eon, period/era, and epoch/period relationships from
  [ICS International Chronostratigraphic Chart, 2024/12](https://stratigraphy.org/ICSchart/ChronostratChart2024-12.pdf).
  Lower/Upper become Early/Late; period names disambiguate repeated epoch names.
  Cambrian Series 2 and Carboniferous subperiod conventions are in `DATASETS.md`.
  Numerical boundaries, ages/stages, and subepochs are not transcribed.

The small ordered lists have inline reference expectations in tests:
[NASA's eight planets](https://science.nasa.gov/solar-system/planets/),
[NASA's four Earth layers](https://science.nasa.gov/earth/facts/#h-structure),
[NASA's five main atmosphere layers](https://science.nasa.gov/earth/earth-atmosphere/earths-atmosphere-a-multi-layered-cake/),
and the [University of Texas Danteworlds *Inferno* guide](https://danteworlds.laits.utexas.edu/index2.html).

OpenStax references are by J. Gordon Betts and colleagues, *Anatomy and Physiology
2e*, Rice University (2022). Only anatomical facts and our chosen identifiers are
used here; source text and illustrations are not included in the crate.

## Relationship additions (checked 2026-09-20)

- `standard-genetic-code.tsv`: RNA codon, ordinary amino-acid/stop meaning, and
  initiation marker extracted from NCBI standard table 1. Source uses T; this
  RNA fixture uses U. Tests distinguish elongation from initiation semantics.
- `si-base-units.tsv`: seven base quantities, corresponding units and symbols
  transcribed from BIPM. Existing prefix fixtures also test the unified view.
- `iso-countries.tsv`: explicit Rust identifier, source label, ISO alpha-2,
  alpha-3 and three-digit numeric code. The 248 English UN M49 table entries
  are supplemented with Taiwan from the W3C Locations Extension ISO table.
  See `DATASETS.md` for scope, source links, naming rules and legacy aliases.
  `iso-countries-source.json` records capture/check metadata and checksums.
