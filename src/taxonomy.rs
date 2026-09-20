//! A versioned animal taxonomy, enabled by the `taxonomy` feature.
//!
//! Species are selected by an unambiguous Wikidata / Catalogue of Life match,
//! an English Wikipedia article, and accepted, explicitly extant status in the
//! pinned Catalogue of Life release. All their ancestors are included, retaining
//! intermediate ranks. Children describe this selection, not every known taxon.
//!
//! Data is bundled: no network access or additional dependencies are needed.
//! Strings borrow the embedded TSV; navigation indexes are initialized once on
//! first use. IDs are Catalogue of Life IDs within the [source release](crate::taxonomy::SOURCE_VERSION).
//!
//! ```
//! use hodgepodge::taxonomy::{animalia, Species};
//! use animalia::chordata::mammalia::carnivora::felidae::panthera;
//! let lion = Species::PantheraLeo;
//! assert_eq!(lion, panthera::Leo);
//! assert_eq!(lion.taxon().parent(), Some(panthera::TAXON));
//! assert_eq!(lion.taxon().lineage()[0].scientific_name(), "Eukaryota");
//! ```
//!
//! [`Species`](crate::taxonomy::Species) implements the common [`Dataset`](crate::Dataset) contract.
//! Hierarchical aliases are real enum variants and can appear in patterns.
//! Paths use the principal ranks; the full record retains intermediate ranks.
//! Missing source ranks are skipped, and duplicate group names gain source-ID
//! suffixes. Group modules expose a `TAXON` constant for metadata and traversal.
//! The kingdom is available as [`animalia`](crate::taxonomy::animalia) or
//! [`eukaryota::animalia`](crate::taxonomy::eukaryota::animalia).
//!
//! The extracted classification is CC BY 4.0, attributed in the bundled
//! `data/taxonomy/NOTICE.md`; Wikidata fields are CC0. See that notice and the
//! import manifest for selection rules, source hashes, and coverage limits.

use crate::TaxonomicRank;
use std::sync::OnceLock;

include!("taxonomy_paths.rs");
include!("taxonomy_species.rs");

#[path = "taxonomy_species_impl.rs"]
mod species_impl;

/// The pinned Catalogue of Life Base Release date.
pub const SOURCE_VERSION: &str = "2026-09-11";
/// The source release DOI.
pub const SOURCE_DOI: &str = "https://doi.org/10.48580/dgz5n";
/// The source's `ChecklistBank` dataset key.
pub const SOURCE_DATASET: u32 = 316_321;
/// Machine-readable provenance, checksums, selection counts, and coverage.
pub const MANIFEST: &str = include_str!("../data/taxonomy/manifest.json");
/// Attribution and data licensing information that must accompany reuse.
pub const NOTICE: &str = include_str!("../data/taxonomy/NOTICE.md");
/// Normalized UTF-8 TSV, including a header; useful for exporting the snapshot.
#[must_use]
pub fn snapshot_tsv() -> &'static str {
    TSV
}

const TSV: &str = include_str!("../data/taxonomy/taxa.tsv");

#[derive(Debug)]
struct Record {
    id: &'static str,
    name: &'static str,
    rank: &'static str,
    parent: Option<usize>,
    label: Option<&'static str>,
    wikidata: Option<&'static str>,
    wikipedia: Option<&'static str>,
    extinct: Option<bool>,
    source_id: Option<&'static str>,
}

struct Index {
    records: Vec<Record>,
    children: Vec<Vec<usize>>,
}

fn nonempty(value: &'static str) -> Option<&'static str> {
    (!value.is_empty()).then_some(value)
}

fn index() -> &'static Index {
    static INDEX: OnceLock<Index> = OnceLock::new();
    INDEX.get_or_init(|| {
        let rows: Vec<Vec<&str>> = TSV
            .lines()
            .skip(1)
            .map(|row| row.split('\t').collect())
            .collect();
        let records: Vec<Record> = rows
            .iter()
            .map(|row| Record {
                id: row[0],
                source_id: nonempty(row[8]),
                name: row[2],
                rank: row[3],
                parent: nonempty(row[1]).map(|id| {
                    rows.binary_search_by_key(&id, |r| r[0])
                        .expect("bundled taxonomy parent")
                }),
                extinct: match row[4] {
                    "true" => Some(true),
                    "false" => Some(false),
                    _ => None,
                },
                label: nonempty(row[5]),
                wikidata: nonempty(row[6]),
                wikipedia: nonempty(row[7]),
            })
            .collect();
        let mut children = vec![Vec::new(); records.len()];
        for (i, record) in records.iter().enumerate() {
            if let Some(parent) = record.parent {
                children[parent].push(i);
            }
        }
        for siblings in &mut children {
            siblings.sort_unstable_by_key(|&i| (records[i].name, records[i].id));
        }
        Index { records, children }
    })
}

/// A lightweight handle into the bundled taxonomy snapshot.
///
/// Handles are `Copy`; persist [`id()`](Self::id) together with the source
/// version rather than a handle's internal position. Source IDs and taxonomy
/// can change across releases. Handles are not enum variants and do not
/// implement the enum-specific `Dataset`, Serde, or random-sampling contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Taxon(usize);

impl Taxon {
    fn record(self) -> &'static Record {
        &index().records[self.0]
    }

    /// Every included taxon, in lexicographic Catalogue of Life ID order.
    pub fn all() -> impl ExactSizeIterator<Item = Self> + DoubleEndedIterator {
        (0..index().records.len()).map(Self)
    }

    /// The number of included species plus their ancestor records.
    #[must_use]
    pub fn count() -> usize {
        index().records.len()
    }

    /// Look up a case-sensitive Catalogue of Life ID in this snapshot.
    #[must_use]
    pub fn by_id(id: &str) -> Option<Self> {
        index()
            .records
            .binary_search_by_key(&id, |r| r.id)
            .ok()
            .map(Self)
    }

    /// Match a scientific name or English Wikidata label, ignoring ASCII case.
    ///
    /// Multiple taxa may share a label. This scans the bundled selection and
    /// returns every exact match; it does not resolve synonyms or fuzzy names.
    pub fn named(name: &str) -> impl Iterator<Item = Self> + '_ {
        Self::all().filter(move |taxon| {
            taxon.scientific_name().eq_ignore_ascii_case(name)
                || taxon
                    .english_label()
                    .is_some_and(|label| label.eq_ignore_ascii_case(name))
        })
    }

    /// Included taxa with no parent in the source classification.
    pub fn roots() -> impl Iterator<Item = Self> {
        Self::all().filter(|taxon| taxon.record().parent.is_none())
    }

    /// The Catalogue of Life ID, scoped to [`SOURCE_VERSION`].
    #[must_use]
    pub fn id(self) -> &'static str {
        self.record().id
    }

    /// The accepted scientific name from Catalogue of Life, without authorship.
    #[must_use]
    pub fn scientific_name(self) -> &'static str {
        self.record().name
    }

    /// The exact source rank, including intermediate ranks such as `subfamily`.
    #[must_use]
    pub fn source_rank(self) -> &'static str {
        self.record().rank
    }

    /// The corresponding principal rank, or `None` for other source ranks.
    #[must_use]
    pub fn rank(self) -> Option<TaxonomicRank> {
        match self.source_rank() {
            "domain" => Some(TaxonomicRank::Domain),
            "kingdom" => Some(TaxonomicRank::Kingdom),
            "phylum" => Some(TaxonomicRank::Phylum),
            "class" => Some(TaxonomicRank::Class),
            "order" => Some(TaxonomicRank::Order),
            "family" => Some(TaxonomicRank::Family),
            "genus" => Some(TaxonomicRank::Genus),
            "species" => Some(TaxonomicRank::Species),
            _ => None,
        }
    }

    /// English Wikidata label for a selected species, when supplied.
    ///
    /// Labels may be scientific names; they are not guaranteed common names.
    #[must_use]
    pub fn english_label(self) -> Option<&'static str> {
        self.record().label
    }

    /// The Wikidata entity ID for a selected species.
    #[must_use]
    pub fn wikidata_id(self) -> Option<&'static str> {
        self.record().wikidata
    }

    /// The English Wikipedia article URL captured in the Wikidata snapshot.
    #[must_use]
    pub fn wikipedia_url(self) -> Option<&'static str> {
        self.record().wikipedia
    }

    /// Whether this record met the species-selection rule.
    ///
    /// Other records provide ancestry only; lack of selection does not imply
    /// that a Wikipedia article or living member is absent.
    #[must_use]
    pub fn is_selected_species(self) -> bool {
        self.wikidata_id().is_some()
    }

    /// The typed species value, or `None` for a classification-group record.
    #[must_use]
    pub fn species(self) -> Option<Species> {
        Species::from_taxon(self)
    }

    /// Source extinction flag; `None` preserves an unspecified value.
    #[must_use]
    pub fn extinct(self) -> Option<bool> {
        self.record().extinct
    }

    /// Contributing checklist ID, matching `data/taxonomy/sources/<id>.yaml`.
    ///
    /// Some higher taxa were assigned by the Catalogue of Life editors and
    /// have no contributing checklist ID.
    #[must_use]
    pub fn source_dataset_id(self) -> Option<&'static str> {
        self.record().source_id
    }

    /// The immediate source parent, retaining intermediate ranks.
    #[must_use]
    pub fn parent(self) -> Option<Self> {
        self.record().parent.map(Self)
    }

    /// Included direct children, ordered by scientific name, then source ID.
    ///
    /// An empty result means no children in this selection, not necessarily
    /// that this taxon is terminal in the complete source database.
    pub fn children(self) -> impl ExactSizeIterator<Item = Self> + DoubleEndedIterator {
        index().children[self.0].iter().copied().map(Self)
    }

    /// Ancestors from immediate parent up to the root, excluding this taxon.
    pub fn ancestors(self) -> impl Iterator<Item = Self> {
        std::iter::successors(self.parent(), |taxon| taxon.parent())
    }

    /// The root-to-self path, including this taxon.
    #[must_use]
    pub fn lineage(self) -> Vec<Self> {
        let mut path = vec![self];
        path.extend(self.ancestors());
        path.reverse();
        path
    }
}

impl std::fmt::Display for Taxon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.scientific_name())
    }
}
