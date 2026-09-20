"""Offline importer tests for joins, ancestry, and rejected source data."""
import unittest
from unittest import mock
import contextlib
import http.client
import io
import json
from pathlib import Path
import tempfile
import import_taxonomy as importer


def taxon(identifier, parent, name, rank, extinct='false', status='accepted'):
    return dict(zip(importer.COL_HEAD, [identifier, parent, name, rank, extinct, 'test', status]))


def candidate(identifier, item, name):
    return dict(zip(importer.WD_HEAD, [identifier, item, name, 'label', 'https://en.wikipedia.org/wiki/' + item]))


class ImportTests(unittest.TestCase):
    def setUp(self):
        self.col = {r['id']: r for r in [
            taxon('root', '', 'Eukaryota', 'domain', ''),
            taxon('N', 'root', 'Animalia', 'kingdom', ''),
            taxon('phylum', 'N', 'Test phylum', 'phylum', ''),
            taxon('sub', 'phylum', 'Test subfamily', 'subfamily', ''),
            taxon('a', 'sub', 'Genus a', 'species'),
            taxon('b', 'sub', 'Genus b', 'species'),
            taxon('dead', 'sub', 'Genus dead', 'species', 'true'),
            taxon('unknown', 'sub', 'Genus unknown', 'species', ''),
            taxon('provisional', 'sub', 'Genus provisional', 'species', 'false', 'provisionally accepted'),
        ]}

    def test_exact_join_preserves_every_intermediate_ancestor(self):
        selected, closure, _, phyla = importer.select(self.col, [candidate('a', 'Q1', 'Genus a')])
        self.assertEqual(set(selected), {'a'})
        self.assertEqual(closure, {'root', 'N', 'phylum', 'sub', 'a'})
        self.assertEqual(phyla, {'Test phylum': 1})

    def test_missing_extinction_flag_and_provisional_status_are_not_eligible(self):
        candidates = [candidate(i, 'Q1', self.col[i]['scientific_name']) for i in ['dead', 'unknown', 'provisional']]
        selected, _, reasons, _ = importer.select(self.col, candidates)
        self.assertFalse(selected)
        self.assertEqual(reasons['ineligible_col_record'], 3)

    def test_name_mismatch_does_not_silently_resolve_a_synonym(self):
        selected, _, reasons, _ = importer.select(self.col, [candidate('a', 'Q1', 'Old genus a')])
        self.assertFalse(selected)
        self.assertEqual(reasons['scientific_name_mismatch'], 1)

    def test_multiple_entities_for_one_col_id_are_excluded(self):
        selected, _, reasons, _ = importer.select(self.col, [candidate('a', 'Q1', 'Genus a'), candidate('a', 'Q2', 'Genus a')])
        self.assertFalse(selected)
        self.assertEqual(reasons['ambiguous_col_match'], 1)

    def test_one_entity_for_multiple_accepted_species_is_excluded(self):
        selected, _, reasons, _ = importer.select(self.col, [candidate('a', 'Q1', 'Genus a'), candidate('b', 'Q1', 'Genus b')])
        self.assertFalse(selected)
        self.assertEqual(reasons['ambiguous_wikidata_match'], 2)

    def test_multiple_names_on_one_item_are_not_arbitrarily_chosen(self):
        selected, _, _, _ = importer.select(self.col, [candidate('a', 'Q1', 'Genus a'), candidate('a', 'Q1', 'Old genus a')])
        self.assertFalse(selected)

    def test_repeated_identical_source_rows_are_deduplicated(self):
        row = candidate('a', 'Q1', 'Genus a')
        selected, _, _, _ = importer.select(self.col, [row, row])
        self.assertEqual(set(selected), {'a'})

    def test_missing_parent_and_cycles_fail_closed(self):
        for parent in ['missing', 'a']:
            with self.subTest(parent=parent):
                self.col['sub']['parent_id'] = parent
                with self.assertRaises(ValueError):
                    importer.select(self.col, [candidate('a', 'Q1', 'Genus a')])

    def test_non_animal_branch_is_rejected(self):
        self.col['phylum']['parent_id'] = 'root'
        with self.assertRaisesRegex(ValueError, 'outside Animalia'):
            importer.select(self.col, [candidate('a', 'Q1', 'Genus a')])

    def test_order_of_candidates_cannot_change_output(self):
        candidates = [candidate('a', 'Q1', 'Genus a'), candidate('b', 'Q2', 'Genus b')]
        self.assertEqual(importer.select(self.col, candidates), importer.select(self.col, list(reversed(candidates))))

    def test_interrupted_or_truncated_response_is_retried_and_cached(self):
        payload = json.dumps({'results': {'bindings': [{
            'col': {'value': 'a'}, 'item': {'value': 'http://www.wikidata.org/entity/Q1'},
            'name': {'value': 'Genus a'}, 'label': {'value': 'A \"quoted\" label'},
            'article': {'value': 'https://en.wikipedia.org/wiki/Example'},
        }]}}).encode()
        for failure in [http.client.IncompleteRead(b''), io.BytesIO(b'{"results":')]:
            with self.subTest(failure=type(failure).__name__), tempfile.TemporaryDirectory() as temporary:
                cache = Path(temporary)
                data = cache / 'data'
                data.mkdir()
                with mock.patch.object(importer, 'DATA', data), mock.patch.object(importer.time, 'sleep'), contextlib.redirect_stdout(io.StringIO()):
                    with mock.patch.object(importer.urllib.request, 'urlopen', side_effect=[failure, io.BytesIO(payload)]) as network:
                        importer.fetch_wikidata(cache, {'a': self.col['a']})
                        self.assertEqual(network.call_count, 2)
                    rows = list(importer.read_tsv(data / 'wikidata.tsv.gz'))
                    self.assertEqual(rows[0]['english_label'], 'A "quoted" label')
                    before = (data / 'wikidata.tsv.gz').read_bytes()
                    with mock.patch.object(importer.urllib.request, 'urlopen', side_effect=AssertionError('network on cache hit')):
                        importer.fetch_wikidata(cache, {'a': self.col['a']})
                    self.assertEqual((data / 'wikidata.tsv.gz').read_bytes(), before)

    def test_tsv_preserves_quotes_and_rejects_control_characters(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'test.tsv'
            importer.write_tsv(path, ['label'], [['"quoted"']])
            self.assertEqual(list(importer.read_tsv(path)), [{'label': '"quoted"'}])
            for value in ['bad\tvalue', 'bad\nvalue', 'bad\rvalue']:
                with self.assertRaises(ValueError):
                    importer.write_tsv(path, ['label'], [[value]])

    def test_large_request_failure_splits_without_losing_ids_and_resumes_offline(self):
        col = {f'C{i}': taxon(f'C{i}', 'N', f'Genus species{i}', 'species') for i in range(15001)}
        successful_ids = set()
        first = True
        def response(request, **_):
            nonlocal first
            query = importer.urllib.parse.parse_qs(request.data.decode())['query'][0]
            requested = query.split('VALUES ?col {', 1)[1].split('}', 1)[0]
            identifiers = [json.loads(value) for value in requested.split()]
            if first:
                first = False
                self.assertEqual(len(identifiers), 15000)
                raise importer.urllib.error.HTTPError(request.full_url, 502, 'test interruption', {}, None)
            self.assertLessEqual(len(identifiers), 3000)
            self.assertFalse(successful_ids.intersection(identifiers))
            successful_ids.update(identifiers)
            return io.BytesIO(b'{"results":{"bindings":[]}}')
        with tempfile.TemporaryDirectory() as temporary:
            cache = Path(temporary)
            data = cache / 'data'
            data.mkdir()
            with mock.patch.object(importer, 'DATA', data), mock.patch.object(importer.time, 'sleep'), contextlib.redirect_stdout(io.StringIO()):
                with mock.patch.object(importer.urllib.request, 'urlopen', side_effect=response):
                    importer.fetch_wikidata(cache, col)
                self.assertEqual(successful_ids, set(col))
                with mock.patch.object(importer.urllib.request, 'urlopen', side_effect=AssertionError('network on cache hit')):
                    importer.fetch_wikidata(cache, col)
                capture = json.loads((data / 'wikidata-capture.json').read_text())
                self.assertEqual(capture['queried_col_ids'], 15001)
                self.assertEqual(capture['query_template_sha256'], importer.sha(data / 'query.rq'))


if __name__ == '__main__':
    unittest.main()
