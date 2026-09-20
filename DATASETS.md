# Dataset scope and maintenance

`hodgepodge` supplies familiar datasets for code examples and fixtures. Every enum
has canonical names, presentation labels, a complete `ALL` slice, and `COUNT`.
Counts describe the declared dataset under the conventions below. They are not
necessarily counts of distinct real-world entities: for example, CSS aliases
have different names, and muscle types do not enumerate left/right instances.

## Scope

| Dataset | Scope and metadata |
| --- | --- |
| `CSS` | All 148 opaque CSS named colors. Aliases share exact `rgb()` values. `transparent` and `currentcolor` are excluded. |
| `RGB`, `CMYK`, `ROYGBIV` | Chosen RGB swatches for familiar palettes. CMYK is not an ink-conversion model; rainbow swatches do not specify wavelengths. All color types expose `rgb_channels()`. |
| `EU` | The 27 EU members as of September 2026; existing names such as `CzechRepublic` are preserved. |
| `States` | The 50 US states and USPS abbreviations; excludes DC and territories. |
| `CanadianProvince` | Ten provinces and three territories, Canada Post abbreviations, and `is_territory()`. The existing type name is retained. |
| `Continent`, `Ocean` | Seven-continent convention using Australia; five named oceans. |
| `Direction` | Four cardinal directions, opposites, and clockwise bearings in degrees from North. |
| `Country` | Legacy illustrative list including historical names and aliases, such as both Myanmar and Burma. Not an ISO registry or a unique count of sovereign states. |
| `Element` | 118 elements, atomic numbers, and chemical symbols. Existing identifiers `Aluminum`, `Cesium`, and `Wolfram` remain; Wolfram is tungsten (`W`). Atomic weights are not supplied. |
| `Planet` | Eight solar-system planets, Mercury–Neptune, with one-based orbital order. Pluto is excluded as a dwarf planet. |
| `PrefixLarge`, `PrefixSmall` | All 24 SI prefixes including the four 2022 additions; case-sensitive symbols and signed exponents. Existing small-prefix discriminants remain positive magnitudes. Micro's symbol is `µ` (U+00B5). |
| `Unit` | Seven physical quantity categories and an SI unit symbol for each. These categories are not the seven SI base quantities; volume uses `m³` and plane angle uses `rad`. |
| `TaxonomicRank` | Eight introductory ranks, Domain–Species, with a next-broader-rank relationship. Intermediate ranks are excluded from this enum. |
| `taxonomy::Species`, `taxonomy::Taxon` (optional) | One enum for 80,868 selected species, with hierarchical aliases and the common dataset API. Source records include complete Catalogue of Life ancestry, exact ranks, labels, links, extinction flags, and traversal. See the versioned manifest and taxonomy notes below. |
| `Day`, `Month` | Monday-first weekdays and January-first months, numbered from one; checked `TryFrom<u8>`, English abbreviations, Saturday/Sunday weekend convention, and common-year Gregorian month lengths. |
| `Season`, `Quarter` | Northern Hemisphere meteorological seasons and their months; quarter numbers 1–4 without implying a fiscal start month. |
| `Suit`, `Rank`, `Card` | Standard 52-card French-suited deck without jokers. `standard_deck()` orders Hearts, Clubs, Spades, Diamonds, then Ace–King. Rank ordinals are not game scores. |
| `DiceFace`, `CoinSide` | Six die faces with checked pip conversions; two sides of an ideal coin. |
| `RockPaperScissors`, `RoundOutcome` | Three-move game, with outcome from the receiving move's perspective. |
| `ChessPiece` | Six piece types; ordinals remain list positions. Conventional material values are 1/3/3/5/9, with `None` for the king. These values are a teaching convention, not a position evaluation. |
| `DnaBase` | A, C, G, T, their symbols, and complementary pairs. No RNA uracil, ambiguity codes, or modified bases. |
| `AminoAcid` | Twenty standard proteinogenic amino acids and their one- and three-letter codes. No selenocysteine, pyrrolysine, ambiguous residues, or stop codes. `TryFrom<char>` parses one-letter codes; `FromStr` still parses full names. |
| `Bone` | Conventional adult 206-bone skeleton: 80 axial, 126 appendicular. Left/right bones are individual variants; side, region, and division are explicit. |
| `BodySide`, `BoneRegion`, `SkeletalDivision` | Anatomical side from the person's perspective, eleven bone regions, and axial/appendicular divisions. |
| `Muscle`, `MuscleRegion` | Selected 92 named skeletal muscle types and four broad locations. Not an exhaustive muscle inventory; paired instances and muscle heads are not separate entries. Locations do not encode origins, insertions, or actions. |
| `Biome` | Fourteen WWF terrestrial biomes. Broad ecological habitats, not topographic landforms or a complete aquatic classification. |
| `RockClass`, `Rock` | Three formation classes and 21 selected common rock names. `Breccia` specifically means sedimentary breccia here; breccias also form in other settings. |
| `EarthLayer` | Introductory four-layer model: crust, mantle, outer core, inner core. Not the mechanical lithosphere/asthenosphere classification. |
| `AtmosphereLayer` | Five principal layers: troposphere through exosphere. The overlapping ionosphere is excluded from this scheme. Ordinals are order, not fixed altitudes. |
| `GeologicEon`, `GeologicEra`, `GeologicPeriod`, `GeologicEpoch` | ICS chart 2024/12: four eons, ten eras, 22 periods, 38 Phanerozoic epochs/series. Parent relationships and oldest-first declaration order; no boundary dates. |
| `InfernoCircle` | Nine circles in Dante's *Inferno*, numbered from Limbo to Treachery. Includes Anger for wrath/sullenness and Greed for avarice/prodigality. Excludes the vestibule, internal subdivisions, *Purgatorio*, and *Paradiso*. |
| `Medal`, `Ordinal`, `BetterThanRust` | Podium places, English ordinals 1–30, and the original programming-language joke. |

## Taxonomy selection and updates

The animal dataset uses Catalogue of Life **Base Release 2026-09-11**, dataset
316321, DOI [10.48580/dgz5n](https://doi.org/10.48580/dgz5n). It is a partial
Eukaryota → Animalia tree, not an inventory of every domain or kingdom.

The bundled snapshot contains **80,868 selected species across 21 phyla** and
**19,983 ancestor records**, for **100,851 taxa**. The importer queried all
**689,062 eligible species**; the captured Wikidata results contained 82,165
candidate rows before exact-name and ambiguity filtering.

The importer considers every accepted animal species explicitly marked extant
in that release. It queries Wikidata in bounded, cached batches for species-rank
items with that COL ID, a scientific name, and an English Wikipedia sitelink.
Only exact accepted-name matches with one unambiguous item/article are included.
It retains every source ancestor, including intermediate and unranked nodes.
Unknown extinction flags, provisionally accepted species, extinct species,
synonyms, and ambiguous joins are excluded. No popularity ranking or manual
familiarity quota is applied.

The [manifest](data/taxonomy/manifest.json) reports final counts, represented
phyla, exclusions, source hashes, and the Wikidata capture interval. Coverage is
biased by English Wikipedia, source coverage, exact names, and recorded
extinction status. It is not a representative biodiversity sample. IDs are
scoped to the pinned source release; persist the version with the ID.

The original release and contributing-checklist metadata accompany the
[attribution notice](data/taxonomy/NOTICE.md). The taxonomy data is CC BY 4.0;
Wikidata enrichment is CC0. No Wikipedia prose or images are copied.

Maintainers can validate the shipped tree offline from a Git checkout:

```sh
python3 -m unittest discover -s scripts -p 'test_*.py'
python3 scripts/import_taxonomy.py --check
```

To reproduce the full import, use Python 3.10+ and a cache directory with space
for the approximately 1 GB release archive:

```sh
python3 scripts/import_taxonomy.py --cache /path/to/taxonomy-cache --fetch
```

The importer is resumable, backs off failed requests, rejects incomplete JSON,
and makes no network requests without `--fetch`. Captured normalized Wikidata
inputs and a compact source extract are checked into Git for offline reproduction.
`wikidata.tsv.gz` and `col-selection.tsv.gz` are excluded from the published crate;
use the matching Git revision for the full importer check. The crate still
contains `taxa.tsv`, generated Rust, the manifest, query/capture metadata,
license notice and all contributing-source attribution. The generation-only
check (`python3 scripts/generate_taxonomy_paths.py --check`) works from either
a Git checkout or an unpacked crate.
Rerunning `--fetch` against an empty cache captures newer Wikidata statements;
that is an explicit data update, not guaranteed to reproduce the old capture.
Rebuilding with the original archive and checked-in enrichment omits `--fetch`.
Refreshes stage capture, generation, attribution and validation before installation.
Caught installation failures restore the preceding snapshot. This is not a
filesystem-wide power-loss transaction: do not run refreshes concurrently with
builds, and keep the Git checkout clean before refreshing.

A new COL release requires reviewing the pinned constants, source identities,
removed/renamed IDs, coverage changes, and downstream stored-ID migration.

## Generated species names and paths

The optional `Species` enum has one variant for every selected species and
implements the common `Dataset` API. Its canonical variant names are generated
from scientific names in PascalCase (`Panthera leo` → `PantheraLeo`). Labels
retain the accepted scientific spelling. Parsing ignores ASCII case but accepts
only complete canonical variant names. Serde uses a normal unit-enum
representation: JSON stores exact canonical names; binary formats may store
variant indexes. Neither is promised as a stable identity across snapshots. English labels and short path aliases are not
serialization or parsing aliases. Numeric discriminants are snapshot-specific.

Taxonomic modules re-export the same variants: `panthera::Leo` and
`Species::PantheraLeo` are identical values. Paths retain domain, kingdom,
phylum, class, order, family, and genus when present. Intermediate ranks remain
in `Taxon` ancestry. Each group module exposes its source record as `TAXON`.
No missing ranks are invented: 68 selected species have no source genus, so
their aliases retain the complete binomial under the nearest available group.

Module names use lowercase ASCII with underscores. Variant names and aliases
use PascalCase. Punctuation separates words; accents are folded to ASCII.
Leading digits gain a `Taxon` prefix, and reserved identifiers are adjusted.
Collisions are resolved with source IDs (`_col_<id>` for modules, `Col<Id>` for
variants). Two pairs of genus records, Iridopsis and Pseudocoremia, collide after
intermediate ranks are omitted; their modules retain separate source IDs.
The generator rejects any remaining collision rather than merging records.

The importer regenerates both the enum and aliases. To regenerate those from
the existing bundled TSV without downloading any data, run:

```sh
python3 scripts/generate_taxonomy_paths.py
```

The offline importer check verifies both generated files exactly against their
source data. New source snapshots can add/remove variants, rename paths, or
change numeric values; review these as public API changes. Persist Catalogue
of Life IDs together with the source version when durable identity is needed, using `Taxon::key()` and
`TaxonKey::resolve()` for exact-release validation.

### Build and documentation costs

The generated taxonomy sources total about 36.4 MiB. The optional feature
keeps them out of default compilation, but enabling it adds substantial compiler
work. On an Apple M1 with 16 GiB RAM, Rust 1.93.1 checked all features in about
113 seconds with 1.1 GiB peak memory and cached dependencies. Initial debug test
compilation took about 6.6 minutes on Rust 1.93.1 and 9.7 minutes on Rust 1.71.0
while those two runs shared the machine. Incremental runs can be much faster.

The complete rustdoc output is hundreds of megabytes. The `Species` enum page
alone is about 33 MiB; the expandable browser is a more convenient way to explore
the animal data. Deep module aliases retain Rust path navigation without
rendering a separate full species definition under every genus. These local
measurements depend on the compiler, machine, feature set, and build cache.

## Anatomy conventions

The 206-bone model counts the fused adult sacrum, coccyx, sternum, and each hip
bone once. It excludes anatomical variation and accessory sesamoids, while
including the two patellae. Teeth are not bones. Cervical vertebrae 1 and 2 are
the atlas and axis. Ribs are numbered 1–12 per side; digits 1–5 start at the thumb
or great toe, neither of which has a middle phalanx. Left/right always refer to
the person whose skeleton is described.

The source-derived regional counts are 22 skull bones, six ear ossicles, one
hyoid, 26 vertebral-column bones, 25 thoracic-cage bones, four pectoral-girdle
bones, six upper-limb long bones, 54 hand bones, two hip bones, eight remaining
lower-limb bones, and 52 foot bones.

## Geologic conventions

The time enums use the named divisions in the dated ICS chart. Lower/Upper
series become Early/Late epoch identifiers. `CambrianSeries2` retains the
chart's unnamed Series 2. Carboniferous epochs map directly to `Carboniferous`,
skipping the Mississippian/Pennsylvanian subperiod rank. No eras are invented for
Hadean and no epochs for the Precambrian. Precambrian is not an additional eon;
stages/ages and subepochs are not included. Anthropocene is not an epoch in
this chart. Consumers needing updated numerical boundaries should use the ICS
reference directly.

## Sampling and reproducibility

The optional `rand` integration samples enum **variants** uniformly, with
replacement. It does not imply natural abundance, equal habitat area, or equal
RGB-color probability where aliases exist. `Card` sampling also uses replacement;
consume a shuffled deck to deal unique cards. All randomness comes from the
caller's rand 0.9 RNG. For repeated experiments retain the algorithm, seed,
dependency versions, and sequence of calls.

## Sources and maintenance

[Fixture notes](tests/fixtures/README.md) link the primary references and explain
normalization. Tests run offline and compare all entries and complete sets;
common API tests cover names, labels, iteration metadata, and optional Serde
round trips for every enum. No reference fixture should be regenerated from the
Rust implementation when auditing a data change.

For future membership changes, record the source, retrieval date, naming
convention, and implications for exhaustive matches and stored Serde names.
The remaining legacy `Country` list needs its own explicit migration decision
before modernization. Other numeric discriminants without a documented meaning
remain enum identifiers, not stable external codes.

## Dataset relationships and descriptors

All 56 compact enum datasets and the optional `taxonomy::Species` implement
`Dataset::INFO`; inherent `Type::INFO` and the public `DATASETS` slice expose the
same descriptor. Each descriptor has a qualified ID, scope, coverage category,
source references with recorded editions/check dates, and a representation
license. Referenced publications retain their own copyright/licensing terms.
Library-defined conventions explicitly cite this document rather than imply an
external standard. Helpers such as `DatasetInfo`, errors and `CodonMeaning` are
not separate enum datasets.

Code lookups are separate from canonical `FromStr`: element symbols and SI
symbols require exact case; postal codes, three-letter amino-acid codes, calendar
abbreviations and ISO alphabetic codes ignore ASCII case. None trims whitespace.
ISO numeric codes require three digits, retaining leading zeroes. Checked `u8`
conversion covers day/month/die, elements, planets, card ranks, medals, ordinals,
quarters, Earth/atmospheric layers and Inferno circles, using documented numeric
meanings rather than declaration indexes.

Parent groups enumerate their members for geology, rocks and anatomy. Traversal
covers this library's selection: an empty Hadean era list does not invent eras.
Bone mirroring exchanges left/right counterparts and leaves unpaired midline
bones unchanged. Color-to-RGB lookup returns all matching aliases. Taxonomic
ranks support both `broader()` and `narrower()` in the eight-rank teaching model.

## RNA and the standard genetic code

`RnaBase` contains A/C/G/U. `DnaBase::coding_rna()` converts a **coding-strand**
base, replacing T with U, and `RnaBase::coding_dna()` reverses this. Template-strand
transcription additionally needs complementing and antiparallel orientation.
`complement()` implements canonical Watson-Crick pairs, not wobble pairing.

`Codon` covers all 64 RNA triplets written 5-prime to 3-prime. Canonical variant
names use PascalCase (`Aug`); `sequence()`/`label()` return uppercase RNA (`AUG`).
`from_sequence()` rejects DNA T, ambiguity symbols and whitespace. `bases()` and
`from_bases()` convert typed triplets; `reverse_complement()` reverses orientation
and complements all three bases. `standard_meaning()` uses NCBI **table 1** during
elongation and returns an amino acid or stop. It is not a universal genetic code.
`is_standard_start()` reports the table's AUG/UUG/CUG initiation markers; actual
initiation is context-dependent and incorporates methionine. `standard_codons()`
provides the reverse amino-acid mapping under the same elongation convention.

Source: [NCBI genetic codes](https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi#SG1),
page updated 2024-09-23, checked 2026-09-20. Only factual table assignments are
transcribed, with T replaced by U for RNA.

## SI quantities and prefixes

`SiBaseQuantity` and `SiBaseUnit` cover the seven corresponding SI quantities and
units, with bidirectional `unit()`/`quantity()` mapping and exact unit symbols.
`Unit` remains the legacy quantity selection; `base_quantity()` returns `None`
for volume, angle and energy. `SiPrefix` unifies all 24 prefixes in increasing
signed-exponent order and accepts conversions from `PrefixSmall`/`PrefixLarge`.
Exponent zero is an absent prefix, not a 25th official prefix. Prefix symbols
are case-sensitive and micro uses U+00B5; ASCII u and Greek mu are not aliases.
These datasets describe units/prefixes; they are not a dimension-checked quantity
arithmetic or automatic unit-conversion engine. In particular, prefixing mass
uses gram conventions rather than mechanically prefixing the symbol kg.

Sources: BIPM [base units](https://www.bipm.org/en/measurement-units/si-base-units)
and [prefixes](https://www.bipm.org/en/measurement-units/si-prefixes), checked 2026-09-20.

## Country and area codes

`IsoCountry` contains 249 assigned ISO 3166-1 country/area entries with alpha-2,
alpha-3 and numeric codes. This includes territories and does not assert that
all entries are independent sovereign states. The 248-row English
[UN M49 table](https://unstats.un.org/unsd/methodology/m49/overview/) was captured
2026-09-20. Taiwan's TW/TWN/158 entry is supplemented from the
[W3C DPV Locations Extension ISO table](https://www.w3.org/community/reports/dpvcg/CG-FINAL-loc-20240801/)
(2024-08-01), which references ISO OBP and EU vocabularies. Unassigned/reserved
codes and user-assigned XK are excluded. Names/codes are factual transcriptions;
source prose is not copied.

Labels retain source names. Rust identifiers use ASCII PascalCase with explicit
short forms for long names (for example `UnitedStates`, `UnitedKingdom`, `Iran`,
`NorthKorea`, `VaticanCity`, `HongKong`, and `Taiwan`). `Nauru` retains the familiar
Rust spelling while the captured UN label is `Naoero`. The complete mapping is
recorded in `tests/fixtures/iso-countries.tsv`; compare names and codes before
updating this dated snapshot. Enum indexes are not ISO numeric codes.

The legacy `Country` list and its parsing remain unchanged. `iso_country()`
returns a reviewed mapping, with Burma/Myanmar sharing one entry and Kosovo
returning `None` because this dataset has no assigned ISO code for it.
`IsoCountry::legacy_countries()` retains every alias rather than choosing one.
EU members map to both representations; US states and Canadian subdivisions
expose their country, and those countries enumerate their included subdivisions.

## Calendar conventions

`Season::months()` retains Northern Hemisphere meteorological months.
`months_in(Hemisphere)` and `Month::season(Hemisphere)` make that choice explicit;
local tropical wet/dry seasons are outside scope. `Quarter::months(start)` and
`Month::quarter(start)` require a fiscal start month; January gives calendar
quarters. Day/month `next()` and `previous()` wrap. `days_in_year(year)` uses the
proleptic Gregorian calendar, with astronomical year zero allowed.

Sources: [NOAA seasons](https://www.ncei.noaa.gov/news/meteorological-versus-astronomical-seasons)
and [US Naval Observatory leap years](https://aa.usno.navy.mil/faq/leap_years),
checked 2026-09-20.

## Publication size budget

CI runs `scripts/check_package.py` against the actual compressed `.crate` and
rejects packages above **8 MiB**, leaving headroom below the registry's default
10 MiB limit. It also checks that runtime taxonomy and attribution remain present
and the two repository-only import inputs remain excluded. This controls download
size; compilation and rustdoc costs of the full generated enum remain substantial.
