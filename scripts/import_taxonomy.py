#!/usr/bin/env python3
"""Import/check the pinned animal taxonomy using Python 3.10+ standard library.

Fetch (resumable): --cache /path/to/cache --fetch
Build from cached release and captured Wikidata: --cache /path/to/cache
Check full source data offline from a Git checkout: --check
"""
import argparse
import os
import shutil
import tempfile
import collections
import concurrent.futures
import csv
import datetime
import gzip
import hashlib
import http.client
import io
import json
from pathlib import Path
import re
import time
import threading
import urllib.error
import urllib.parse
import urllib.request
import zipfile
import generate_taxonomy_paths

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / 'data' / 'taxonomy'
VERSION = '2026-09-11'
DATASET = 316321
DOI = '10.48580/dgz5n'
COL_SHA256 = '7feebad85ed9563c08bc67a5ce227f44afa7fb324ebd15fa8ea9ebfc88a13311'
COL_URL = f'https://download.catalogueoflife.org/col/monthly/{VERSION}_coldp.zip'
UA = 'hodgepodge/0.4 (https://github.com/cmccomb/hodgepodge)'
HEAD = ['id', 'parent_id', 'scientific_name', 'rank', 'extinct', 'english_label', 'wikidata_id', 'wikipedia_url', 'source_id']
WD_HEAD = ['id', 'wikidata_id', 'scientific_name', 'english_label', 'wikipedia_url']
COL_HEAD = ['id', 'parent_id', 'scientific_name', 'rank', 'extinct', 'source_id', 'status']
QUERY = '''SELECT ?item ?col ?name ?article ?label WHERE {
 VALUES ?col { __IDS__ }
 ?item wdt:P10585 ?col; wdt:P105 wd:Q7432; wdt:P225 ?name.
 ?article schema:about ?item; schema:isPartOf <https://en.wikipedia.org/>.
 OPTIONAL { ?item rdfs:label ?label FILTER(LANG(?label) = "en") }
}'''
csv.field_size_limit(10_000_000)


def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat(timespec='seconds')


def write_tsv(path, head, rows):
    lines = ['\t'.join(head)]
    for row in rows:
        if any(any(c in value for c in '\t\r\n') for value in row):
            raise ValueError('Control character in source field')
        lines.append('\t'.join(row))
    raw = ('\n'.join(lines) + '\n').encode('utf-8')
    path.write_bytes(gzip.compress(raw, mtime=0) if path.suffix == '.gz' else raw)


def read_tsv(path):
    opener = gzip.open if path.suffix == '.gz' else open
    with opener(path, 'rt', encoding='utf-8', newline='') as f:
        yield from csv.DictReader(f, delimiter='\t', quoting=csv.QUOTE_NONE)


def download(url, path):
    if path.exists():
        return
    temporary = path.with_suffix(path.suffix + '.part')
    req = urllib.request.Request(url, headers={'User-Agent': UA})
    with urllib.request.urlopen(req, timeout=90) as source, temporary.open('wb') as target:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            target.write(chunk)
    temporary.replace(path)


def extract_col(cache):
    archive = cache / 'col.zip'
    output = cache / 'col-accepted.tsv.gz'
    if sha(archive) != COL_SHA256:
        raise ValueError('COL archive checksum differs from the pinned release')
    provenance = cache / 'col-accepted.meta.json'
    if output.exists() and provenance.exists():
        expected = {'archive_sha256': COL_SHA256, 'normalizer_version': 1, 'sha256': sha(output)}
        if json.loads(provenance.read_text()) == expected:
            return output
    rows = []
    with zipfile.ZipFile(archive) as source:
        metadata = source.read('metadata.yaml').decode('utf-8')
        if f'key: {DATASET}\n' not in metadata or f'version: {VERSION}\n' not in metadata:
            raise ValueError('Archive does not match the pinned release')
        with source.open('NameUsage.tsv') as f:
            for r in csv.DictReader(io.TextIOWrapper(f, encoding='utf-8'), delimiter='\t'):
                if r['col:status'] in ('accepted', 'provisionally accepted') and (r['col:kingdom'] == 'Animalia' or r['col:ID'] in ('N', 'CS5HF')):
                    rows.append([r['col:' + key] for key in ['ID', 'parentID', 'scientificName', 'rank', 'extinct', 'sourceID', 'status']])
    write_tsv(output, COL_HEAD, sorted(rows))
    provenance.write_text(json.dumps({'archive_sha256': COL_SHA256, 'normalizer_version': 1, 'sha256': sha(output)}, indent=2) + '\n')
    print(f'Extracted {len(rows):,} animal classification records', flush=True)
    return output


def eligible(r):
    return r['rank'] == 'species' and r['extinct'] == 'false' and r['status'] == 'accepted'


def fetch_wikidata(cache, col, data=None):
    data = DATA if data is None else data
    ids = sorted(r['id'] for r in col.values() if eligible(r))
    batches = [ids[i:i + 15000] for i in range(0, len(ids), 15000)]
    request_limit = {'ids': 15000}
    folder = cache / 'wikidata-batches'
    folder.mkdir(exist_ok=True)
    (cache / 'query.rq').write_text(QUERY, encoding='utf-8')

    cooldown_path = cache / 'wikidata-cooldown.json'
    cooldown = json.loads(cooldown_path.read_text())['until'] if cooldown_path.exists() else 0
    throttle = {'until': time.monotonic() + max(0, cooldown - time.time())}
    throttle_lock = threading.Lock()

    def fetch(batch):
        query = QUERY.replace('__IDS__', ' '.join(json.dumps(i) for i in batch))
        key = hashlib.sha256(query.encode()).hexdigest()
        path = folder / f'{key}.json.gz'
        if path.exists():
            result = json.loads(gzip.decompress(path.read_bytes()))
            if result['query_sha256'] != key:
                raise ValueError('Cached query hash mismatch')
            return result
        chunks = [batch[i:i + 3000] for i in range(0, len(batch), 3000)]
        def cached(chunk):
            logical = QUERY.replace('__IDS__', ' '.join(json.dumps(i) for i in chunk))
            return (folder / (hashlib.sha256(logical.encode()).hexdigest() + '.json.gz')).exists()
        # Reuse earlier checkpoints. Large new requests amortize planning cost;
        # a timeout lowers the cap for the rest of this run.
        existing = [chunk for chunk in chunks if cached(chunk)] if len(chunks) > 1 else []
        if len(batch) > request_limit['ids'] or existing:
            if existing:
                missing = [i for chunk in chunks if not cached(chunk) for i in chunk]
                parts = [fetch(chunk) for chunk in existing]
                if missing:
                    parts.append(fetch(missing))
            else:
                parts = [fetch(chunk) for chunk in chunks]
            result = {
                'query_sha256': key,
                'first_retrieved_at': min(r.get('first_retrieved_at', r['retrieved_at']) for r in parts),
                'retrieved_at': max(r['retrieved_at'] for r in parts),
                'parts': [r['query_sha256'] for r in parts],
                'rows': sorted({tuple(row) for r in parts for row in r['rows']}),
            }
            temporary = path.with_suffix('.part')
            temporary.write_bytes(gzip.compress(json.dumps(result, sort_keys=True).encode(), mtime=0))
            temporary.replace(path)
            return result
        form = urllib.parse.urlencode({'query': query, 'format': 'json'}).encode()
        for attempt in range(5):
            try:
                with throttle_lock:
                    pause = max(0, throttle['until'] - time.monotonic())
                if pause:
                    time.sleep(pause)
                request = urllib.request.Request('https://query.wikidata.org/sparql', data=form,
                    headers={'User-Agent': UA, 'Accept': 'application/sparql-results+json',
                             'Content-Type': 'application/x-www-form-urlencoded'})
                started = time.monotonic()
                with urllib.request.urlopen(request, timeout=90) as response:
                    # A timed-out query can return HTTP 200 with incomplete JSON.
                    data = json.load(response)
                rows = []
                allowed = set(batch)
                for r in data['results']['bindings']:
                    identifier = r['col']['value']
                    item = r['item']['value'].rsplit('/', 1)[-1]
                    article = r['article']['value']
                    if identifier not in allowed or not re.fullmatch(r'Q[1-9][0-9]*', item) or not article.startswith('https://en.wikipedia.org/wiki/'):
                        raise ValueError('Unexpected Wikidata result')
                    rows.append([identifier, item, r['name']['value'], r.get('label', {}).get('value', ''), article])
                result = {'query_sha256': key, 'optimizer': 'automatic', 'retrieved_at': utc(), 'rows': sorted(set(map(tuple, rows)))}
                temporary = path.with_suffix('.part')
                temporary.write_bytes(gzip.compress(json.dumps(result, sort_keys=True).encode(), mtime=0))
                temporary.replace(path)
                print(f'Fetched {len(batch):,} IDs / {len(rows):,} rows in {time.monotonic() - started:.1f}s', flush=True)
                time.sleep(1)
                return result
            except (OSError, ValueError, KeyError, http.client.HTTPException) as error:
                if isinstance(error, urllib.error.HTTPError):
                    error.close()
                limited = isinstance(error, urllib.error.HTTPError) and error.code == 429
                if len(batch) > 3000 and not limited:
                    request_limit['ids'] = 3000
                    print(f'Reducing request size after {type(error).__name__}: {error}', flush=True)
                    time.sleep(5)
                    return fetch(batch)
                if attempt == 4:
                    raise
                delay = 5 * 2 ** attempt
                if isinstance(error, urllib.error.HTTPError) and error.code == 429:
                    retry_after = error.headers.get('Retry-After', '60')
                    delay = max(delay, int(retry_after) if retry_after.isdigit() else 60)
                    with throttle_lock:
                        throttle['until'] = max(throttle['until'], time.monotonic() + delay)
                        cooldown_path.write_text(json.dumps({'until': time.time() + delay}))
                print(f'Retrying batch {key[:8]} in {delay}s: {error}', flush=True)
                time.sleep(delay)
        raise AssertionError('unreachable')

    results = []
    # Keep load bounded; rate-limit responses set a shared, persisted cooldown.
    with concurrent.futures.ThreadPoolExecutor(max_workers=1) as pool:
        for n, result in enumerate(pool.map(fetch, batches), 1):
            results.append(result)
            print(f'Wikidata {n}/{len(batches)} batches; {len(result["rows"])} rows', flush=True)
    rows = sorted({tuple(row) for r in results for row in r['rows']})
    output = data / 'wikidata.tsv.gz'
    write_tsv(output, WD_HEAD, rows)
    capture = {'endpoint': 'https://query.wikidata.org/sparql', 'license': 'CC0-1.0',
               'first_retrieved_at': min(r.get('first_retrieved_at', r['retrieved_at']) for r in results),
               'last_retrieved_at': max(r['retrieved_at'] for r in results),
               'queried_col_ids': len(ids), 'batches': len(batches),
               'batching': 'Up to 15,000 input IDs per logical batch; reuse smaller checkpoints and reduce request size after timeouts.',
               'query_template_sha256': hashlib.sha256((QUERY + '\n').encode()).hexdigest(),
               'execution': 'Logical query; some batches use optimizer None to evaluate bound IDs first.',
               'normalized_snapshot_sha256': sha(output)}
    (data / 'wikidata-capture.json').write_text(json.dumps(capture, indent=2) + '\n')
    (data / 'query.rq').write_text(QUERY + '\n')


def select(col, candidates):
    reasons = collections.Counter()
    by_id = collections.defaultdict(set)
    for r in candidates:
        current = col.get(r['id'])
        if not current or not eligible(current):
            reasons['ineligible_col_record'] += 1
        elif r['scientific_name'] != current['scientific_name']:
            reasons['scientific_name_mismatch'] += 1
        else:
            by_id[r['id']].add((r['wikidata_id'], r['english_label'], r['wikipedia_url']))
    # Ambiguity is detected before name filtering too: another scientific name
    # on the same item can indicate a taxonomic split, not a safe synonym.
    raw_by_id = collections.defaultdict(set)
    for r in candidates:
        raw_by_id[r['id']].add((r['wikidata_id'], r['scientific_name'], r['wikipedia_url']))
    proposed = {}
    for identifier, matches in by_id.items():
        if len(matches) != 1 or len(raw_by_id[identifier]) != 1:
            reasons['ambiguous_col_match'] += 1
        else:
            proposed[identifier] = next(iter(matches))
    by_item = collections.Counter(value[0] for value in proposed.values())
    selected = {k: v for k, v in proposed.items() if by_item[v[0]] == 1}
    reasons['ambiguous_wikidata_match'] = len(proposed) - len(selected)
    closure = set()
    phyla = collections.Counter()
    for identifier in selected:
        seen = set()
        current = identifier
        is_animal = False
        while current:
            if current in seen:
                raise ValueError(f'Cycle in source ancestry at {current}')
            if current not in col:
                raise ValueError(f'Missing source ancestor {current} for {identifier}')
            seen.add(current)
            row = col[current]
            is_animal |= current == 'N'
            if row['rank'] == 'phylum':
                phyla[row['scientific_name']] += 1
            current = row['parent_id']
        if not is_animal:
            raise ValueError(f'Selected record outside Animalia: {identifier}')
        closure.update(seen)
    return selected, closure, dict(sorted(reasons.items())), dict(sorted(phyla.items()))


def _build(cache, col, data, paths, species):
    candidates = list(read_tsv(data / 'wikidata.tsv.gz'))
    selected, closure, exclusions, phyla = select(col, candidates)
    rows = []
    for identifier in sorted(closure):
        r = col[identifier]
        qid, label, wiki = selected.get(identifier, ('', '', ''))
        rows.append([identifier, r['parent_id'], r['scientific_name'], r['rank'], r['extinct'], label, qid, wiki, r['source_id']])
    write_tsv(data / 'taxa.tsv', HEAD, rows)
    generate_taxonomy_paths.write(data / 'taxa.tsv', paths, species)
    # Compact authoritative input permits deterministic offline regeneration of
    # the shipped selection. Full eligibility audit uses the pinned archive.
    input_ids = closure | {r['id'] for r in candidates if r['id'] in col}
    write_tsv(data / 'col-selection.tsv.gz', COL_HEAD, [[col[i][k] for k in COL_HEAD] for i in sorted(input_ids)])
    sources = sorted({col[i]['source_id'] for i in input_ids if col[i]['source_id']})
    with zipfile.ZipFile(cache / 'col.zip') as archive:
        folder = data / 'sources'
        folder.mkdir(exist_ok=True)
        for old in folder.glob('*.yaml'):
            old.unlink()
        for source in sources:
            if not re.fullmatch(r'[0-9]+', source):
                raise ValueError(f'Unexpected contributing dataset ID: {source}')
            (folder / f'{source}.yaml').write_bytes(archive.read(f'source/{source}.yaml'))
        (data / 'col-metadata.yaml').write_bytes(archive.read('metadata.yaml'))
    counts = {'taxa': len(rows), 'selected_species': len(selected), 'ancestors': len(closure) - len(selected),
              'eligible_col_species': sum(eligible(r) for r in col.values()), 'wikidata_candidate_rows': len(candidates),
              'phyla': len(phyla),
              'eligible_species_without_wikidata_result': sum(eligible(r) for r in col.values()) - len({r['id'] for r in candidates})}
    manifest = {'schema_version': 1, 'selection': 'Accepted, explicitly extant animal species in the pinned COL Base Release, with one exact scientific-name/ID Wikidata match of species rank and an English Wikipedia sitelink; include every source ancestor.',
                'col': {'version': VERSION, 'dataset_key': DATASET, 'doi': DOI, 'archive_url': COL_URL,
                        'archive_sha256': sha(cache / 'col.zip'), 'license': 'CC-BY-4.0'},
                'wikidata': json.loads((data / 'wikidata-capture.json').read_text()),
                'counts': counts, 'exclusions': exclusions,
                'exclusion_count_units': 'Name mismatches and ineligible records count normalized candidate rows; ambiguous matches count COL IDs.',
                'accepted_animal_species_by_extinction_flag': dict(collections.Counter(r['extinct'] or 'unspecified' for r in col.values() if r['rank'] == 'species' and r['status'] == 'accepted')), 'selected_species_by_phylum': phyla,
                'source_dataset_ids': sources,
                'coverage_limits': ['English Wikipedia and upstream coverage bias; not a representative sample.',
                                    'Unknown extinction status excluded, not treated as extant.',
                                    'Exact accepted-name matching excludes synonyms and renamed taxa.',
                                    'Only selected species and their ancestors; children are incomplete.',
                                    'Wikidata batches were captured over a time interval, not an atomic database snapshot.'],
                'sha256': {p.relative_to(data).as_posix(): sha(p) for p in [data / name for name in ['taxa.tsv', 'col-selection.tsv.gz', 'wikidata.tsv.gz', 'wikidata-capture.json', 'query.rq', 'col-metadata.yaml']] + sorted((data / 'sources').glob('*.yaml'))}}
    (data / 'manifest.json').write_text(json.dumps(manifest, indent=2, ensure_ascii=False) + '\n')
    print(json.dumps(counts, indent=2), flush=True)
    check(data, paths, species)


def check(data=None, paths=None, species=None):
    data = DATA if data is None else data
    if not (data / 'col-selection.tsv.gz').exists() or not (data / 'wikidata.tsv.gz').exists():
        raise ValueError("Full source audit requires a repository checkout; import inputs are excluded from the published crate")
    manifest = json.loads((data / 'manifest.json').read_text())
    for name, checksum in manifest['sha256'].items():
        if sha(data / name) != checksum:
            raise ValueError(f'Checksum mismatch: {name}')
    rows = list(read_tsv(data / 'taxa.tsv'))
    col_rows = list(read_tsv(data / 'col-selection.tsv.gz'))
    col = {r['id']: r for r in col_rows}
    if len(col_rows) != len(col):
        raise ValueError('Duplicate source IDs in the normalized input')
    candidates = list(read_tsv(data / 'wikidata.tsv.gz'))
    selected, closure, exclusions, phyla = select(col, candidates)
    expected = []
    for identifier in sorted(closure):
        r = col[identifier]
        qid, label, wiki = selected.get(identifier, ('', '', ''))
        expected.append(dict(zip(HEAD, [identifier, r['parent_id'], r['scientific_name'], r['rank'], r['extinct'], label, qid, wiki, r['source_id']])))
    if rows != expected or len(rows) != manifest['counts']['taxa'] or len(selected) != manifest['counts']['selected_species'] or phyla != manifest['selected_species_by_phylum']:
        raise ValueError('Snapshot does not reproduce from normalized source inputs')
    if len(rows) != len({r['id'] for r in rows}) or exclusions != manifest['exclusions']:
        raise ValueError('Duplicate output IDs or irreproducible exclusion counts')
    if sha(data / 'query.rq') != manifest['wikidata']['query_template_sha256']:
        raise ValueError('Logical query template checksum mismatch')
    print(f'Verified {len(rows):,} taxa / {len(selected):,} species offline', flush=True)
    generate_taxonomy_paths.check(data / 'taxa.tsv', paths, species)


def _install_snapshot(replacements, backup_dir):
    """Install already-validated paths, restoring originals on Python exceptions.

    This is not a filesystem-wide atomic commit against power loss. Keep the
    repository clean and avoid concurrent readers/builds during an explicit refresh.
    """
    moved = []
    try:
        for number, (staged, destination) in enumerate(replacements):
            backup = backup_dir / str(number)
            existed = destination.exists()
            if existed:
                os.replace(destination, backup)
            moved.append((destination, backup, existed))
            os.replace(staged, destination)
    except BaseException:
        for destination, backup, existed in reversed(moved):
            if destination.is_dir():
                shutil.rmtree(destination)
            elif destination.exists():
                destination.unlink()
            if existed:
                os.replace(backup, destination)
        raise


def build(cache, col, *, fetch=False):
    """Stage capture, generation, attribution and validation before replacing files."""
    with tempfile.TemporaryDirectory(prefix='.taxonomy-stage-', dir=ROOT) as temporary:
        stage = Path(temporary)
        data = stage / 'data'
        if DATA.exists():
            shutil.copytree(DATA, data)
        else:
            data.mkdir()
        paths = stage / 'taxonomy_paths.rs'
        species = stage / 'taxonomy_species.rs'
        if fetch:
            fetch_wikidata(cache, col, data)
        _build(cache, col, data, paths, species)
        backups = stage / 'backups'
        backups.mkdir()
        _install_snapshot([(data, DATA), (paths, generate_taxonomy_paths.OUTPUT),
                           (species, generate_taxonomy_paths.SPECIES_OUTPUT)], backups)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--cache', type=Path)
    parser.add_argument('--fetch', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    if args.check:
        check()
        return
    if args.cache is None:
        parser.error('--cache is required unless --check is used')
    args.cache.mkdir(parents=True, exist_ok=True)
    DATA.mkdir(parents=True, exist_ok=True)
    if args.fetch:
        download(COL_URL, args.cache / 'col.zip')
    col = {r['id']: r for r in read_tsv(extract_col(args.cache))}
    build(args.cache, col, fetch=args.fetch)


if __name__ == '__main__':
    main()
