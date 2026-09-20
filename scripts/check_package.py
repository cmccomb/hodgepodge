#!/usr/bin/env python3
"""Enforce an 8 MiB compressed release budget and required taxonomy attribution."""
import argparse
import json
from pathlib import Path
import tarfile
import subprocess

MAX_BYTES = 8 * 1024 * 1024
REQUIRED = {'data/taxonomy/taxa.tsv', 'data/taxonomy/NOTICE.md',
            'data/taxonomy/manifest.json', 'data/taxonomy/col-metadata.yaml',
            'src/taxonomy_paths.rs', 'src/taxonomy_species.rs', 'LICENSE-APACHE', 'LICENSE'}
EXCLUDED = {'data/taxonomy/col-selection.tsv.gz', 'data/taxonomy/wikidata.tsv.gz'}


def check(path):
    size = path.stat().st_size
    if size > MAX_BYTES:
        raise ValueError(f'Package is {size:,} bytes, above the {MAX_BYTES:,}-byte budget')
    with tarfile.open(path, 'r:gz') as archive:
        members = {member.name.split('/', 1)[1]: member for member in archive.getmembers() if '/' in member.name}
        names = set(members)
        manifest_member = members.get('data/taxonomy/manifest.json')
        manifest = json.load(archive.extractfile(manifest_member)) if manifest_member else {}
    if not REQUIRED.issubset(names):
        raise ValueError(f'Package is missing required files: {sorted(REQUIRED - names)}')
    if names & EXCLUDED:
        raise ValueError(f'Package contains repository-only import inputs: {sorted(names & EXCLUDED)}')
    sources = {f'data/taxonomy/sources/{source}.yaml' for source in manifest.get('source_dataset_ids', [])}
    if not sources or not sources.issubset(names):
        raise ValueError('Contributing-source attribution metadata is missing or incomplete')
    print(f'Package: {size:,} bytes ({size / 1024**2:.2f} MiB); '
          f'{MAX_BYTES - size:,} bytes below the 8 MiB budget. Runtime taxonomy and attribution retained.')
    return size


def default_package():
    """Use Cargo's current version/target directory, never a stale versioned path."""
    manifest = Path(__file__).resolve().parents[1] / 'Cargo.toml'
    metadata = json.loads(subprocess.check_output([
        'cargo', 'metadata', '--no-deps', '--format-version=1', '--offline',
        '--manifest-path', str(manifest)], text=True))
    package = next(package for package in metadata['packages']
                   if Path(package['manifest_path']).resolve() == manifest)
    return Path(metadata['target_directory']) / 'package' / f"{package['name']}-{package['version']}.crate"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('package', type=Path, nargs='?', help='Defaults to Cargo current package/version/target directory')
    package = parser.parse_args().package
    check(default_package() if package is None else package)


if __name__ == '__main__':
    main()
