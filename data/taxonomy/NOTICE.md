# Animal taxonomy: attribution and scope

The classification in `taxa.tsv` and `col-selection.tsv.gz` is adapted from:

Bánki, O., Roskov, Y., Döring, M., Ower, G., Hernández Robles, D. R., Plata
Corredor, C. A., Stjernegaard Jeppesen, T., Örn, A., Pape, T., Hobern, D.,
Garnett, S., Little, H., DeWalt, R. E., Miller, J., Orrell, T., et al. (2026).
*Catalogue of Life*, version 2026-09-11. Catalogue of Life Foundation,
Amsterdam, Netherlands. https://doi.org/10.48580/dgz5n

© Catalogue of Life and its contributors. This extracted classification is
licensed under Creative Commons Attribution 4.0 International (CC BY 4.0):
https://creativecommons.org/licenses/by/4.0/

The release's original metadata, including its contributor list, is preserved
in `col-metadata.yaml`. Metadata and citations for the contributing checklists
used in this selection are preserved in `sources/`. A record's `source_id`
identifies the corresponding file and ChecklistBank dataset.

Changes by hodgepodge: select accepted animal species explicitly marked extant;
join exact scientific names and Catalogue of Life IDs to Wikidata species with
English Wikipedia sitelinks; omit ambiguous joins; include all source ancestors;
retain selected fields and normalize to UTF-8 TSV sorted by source ID.

English labels, Wikidata identifiers, and Wikipedia URLs come from the Wikidata
contributors' structured data, available under CC0 1.0:
https://www.wikidata.org/wiki/Wikidata:Licensing
https://creativecommons.org/publicdomain/zero/1.0/

No Wikipedia article text or images are copied. English labels are display
labels and may themselves be scientific names. The capture interval, query,
checksums, counts, and exclusions are recorded in the accompanying manifest.
Wikidata was queried in batches, not captured as an atomic database dump.

The selection reflects English Wikipedia coverage, source coverage, exact-name
matching, and the availability of an explicit extinction flag. It is not a
representative sample of biodiversity or a complete classification. Empty
children mean no children in this selection. An unspecified extinction flag is
preserved as unspecified; it is not evidence of extant status.

The software remains available under MIT OR Apache-2.0. The package's combined
license expression includes CC-BY-4.0 because it distributes this attributed
dataset. Reuse of the classification should retain this notice and source
attribution. Enabling the Rust feature controls compilation, not the licensing
of the source files distributed in the package.

Packaging note: `col-selection.tsv.gz` and the captured `wikidata.tsv.gz` import
inputs remain in the matching Git revision but are omitted from the `.crate`
to reduce downloads. The runtime selection, manifest, capture metadata, this
notice and all contributing-source metadata remain bundled. Full importer
reproduction requires the repository checkout; generation-only verification
works from the unpacked package.
