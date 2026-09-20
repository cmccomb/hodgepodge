"""Naming and ancestry tests for generated Rust taxonomy namespaces."""
import unittest
import generate_taxonomy_paths as paths


def row(identifier, parent, name, rank):
    return dict(id=identifier, parent_id=parent, scientific_name=name, rank=rank)


class PathTests(unittest.TestCase):
    def setUp(self):
        self.rows = sorted([
            row('D', '', 'Eukaryota', 'domain'),
            row('N', 'D', 'Animalia', 'kingdom'),
            row('F', 'N', 'Felidae', 'family'),
            row('S', 'F', 'Pantherinae', 'subfamily'),
            row('G', 'S', 'Panthera', 'genus'),
            row('L', 'G', 'Panthera leo', 'species'),
        ], key=lambda r: r['id'])

    def test_principal_paths_skip_intermediate_ranks_without_changing_source(self):
        parents, names = paths.layout(self.rows)
        self.assertEqual(parents['G'], 'F')
        self.assertNotIn('S', names)
        self.assertEqual(names['L'], 'Leo')
        self.assertEqual(next(r for r in self.rows if r['id'] == 'G')['parent_id'], 'S')

    def test_missing_genus_uses_the_full_species_name_and_no_invented_parent(self):
        self.rows.append(row('Z', 'F', 'Unplaced example', 'species'))
        parents, names = paths.layout(self.rows)
        self.assertEqual(parents['Z'], 'F')
        self.assertEqual(names['Z'], 'UnplacedExample')

    def test_same_genus_name_keeps_distinct_source_records_and_descendants(self):
        self.rows.extend([row('X', 'F', 'Panthera', 'genus'), row('Z', 'X', 'Panthera other', 'species')])
        parents, names = paths.layout(self.rows)
        self.assertEqual(names['G'], 'panthera_col_g')
        self.assertEqual(names['X'], 'panthera_col_x')
        self.assertEqual(parents['L'], 'G')
        self.assertEqual(parents['Z'], 'X')

    def test_same_epithet_is_allowed_in_different_genera(self):
        self.rows.extend([row('X', 'F', 'Other', 'genus'), row('Z', 'X', 'Other leo', 'species')])
        _, names = paths.layout(self.rows)
        self.assertEqual(names['L'], 'Leo')
        self.assertEqual(names['Z'], 'Leo')

    def test_normalization_handles_keywords_punctuation_and_initial_digits(self):
        for source, expected in [('type', 'type_taxon'), ('Self', 'self_taxon'), ('A-b c', 'a_b_c'), ('Éxample', 'example'), ('123', 'taxon_123')]:
            self.assertEqual(paths.identifier(source), expected)

    def test_normalization_collisions_are_disambiguated(self):
        self.rows.extend([row('X', 'F', 'A-b', 'genus'), row('Z', 'F', 'A b', 'genus')])
        _, names = paths.layout(self.rows)
        self.assertEqual(names['X'], 'a_b_col_x')
        self.assertEqual(names['Z'], 'a_b_col_z')

    def test_invalid_or_reordered_source_cannot_silently_change_handles(self):
        for rows in [list(reversed(self.rows)), self.rows + [self.rows[-1]], self.rows + [row('Z', 'missing', 'Missing', 'genus')], self.rows + [row('Z', 'Z', 'Cycle', 'genus')]]:
            with self.assertRaises(ValueError):
                paths.layout(rows)

    def test_aliases_reference_real_variants_and_metadata_indexes_match_source(self):
        generated = paths.render(self.rows, 'test-checksum')
        index = next(i for i, r in enumerate(self.rows) if r['id'] == 'L')
        self.assertIn('pub use crate::taxonomy::Species::PantheraLeo as Leo;', generated)
        self.assertIn('pub use eukaryota::animalia;', generated)
        self.assertIn('pub const TAXON:', generated)
        self.assertNotIn('pub mod pantherinae', generated)
        species = paths.render_species(self.rows, 'test-checksum')
        self.assertIn('    PantheraLeo,', species)
        self.assertIn('pub const COUNT: usize = 1;', species)
        self.assertIn(f'Taxon({index}),', species)

    def test_global_species_name_collisions_use_source_ids(self):
        self.rows.append(row('Z', 'G', 'Panthera leo', 'species'))
        names = paths.species_names(self.rows)
        self.assertEqual(names['L'], 'PantheraLeoColL')
        self.assertEqual(names['Z'], 'PantheraLeoColZ')
        _, aliases = paths.layout(self.rows)
        self.assertEqual(aliases['L'], 'LeoColL')
        self.assertEqual(aliases['Z'], 'LeoColZ')

    def test_species_names_preserve_subgenus_words_and_handle_self(self):
        self.assertEqual(paths.variant_identifier('Cicindela (Calomera) aphrodisia'), 'CicindelaCalomeraAphrodisia')
        self.assertEqual(paths.variant_identifier('self'), 'SelfTaxon')
        self.assertEqual(paths.variant_identifier('type'), 'Type')


if __name__ == '__main__':
    unittest.main()
