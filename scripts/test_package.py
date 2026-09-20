"""The package guard must reject oversize or accidentally stripped releases."""
import contextlib
import io
import json
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest import mock
import check_package


class PackageTests(unittest.TestCase):
    def test_default_path_tracks_the_current_cargo_version_and_target(self):
        manifest = Path(check_package.__file__).resolve().parents[1] / 'Cargo.toml'
        metadata = {'target_directory': '/tmp/custom-target', 'packages': [
            {'manifest_path': str(manifest), 'name': 'hodgepodge', 'version': '0.5.0'}]}
        with mock.patch.object(check_package.subprocess, 'check_output', return_value=json.dumps(metadata)):
            self.assertEqual(check_package.default_package(), Path('/tmp/custom-target/package/hodgepodge-0.5.0.crate'))

    def test_budget_and_manifest_requirements(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'test.crate'
            def package(names):
                with tarfile.open(path, 'w:gz') as archive:
                    for name in sorted(names):
                        info = tarfile.TarInfo('hodgepodge-test/' + name)
                        payload = b'{"source_dataset_ids": ["123"]}' if name.endswith('manifest.json') else b'x'
                        info.size = len(payload)
                        archive.addfile(info, io.BytesIO(payload))
            required = check_package.REQUIRED | {'data/taxonomy/sources/123.yaml'}
            package(required)
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(check_package.check(path), path.stat().st_size)
            with mock.patch.object(check_package, 'MAX_BYTES', 1):
                with self.assertRaisesRegex(ValueError, 'above'):
                    check_package.check(path)
            package(required - {'data/taxonomy/NOTICE.md'})
            with self.assertRaisesRegex(ValueError, 'missing required'):
                check_package.check(path)
            package(required - {'data/taxonomy/sources/123.yaml'})
            with self.assertRaisesRegex(ValueError, 'attribution'):
                check_package.check(path)
            package(required | check_package.EXCLUDED)
            with self.assertRaisesRegex(ValueError, 'repository-only'):
                check_package.check(path)


if __name__ == '__main__':
    unittest.main()
