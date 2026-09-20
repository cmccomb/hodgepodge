# Contributing

## Development

```sh
python3 -m unittest discover -s scripts -p 'test_*.py'
python3 scripts/import_taxonomy.py --check
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic
bash scripts/test-features.sh
RUSTUP_TOOLCHAIN=1.71.0 bash scripts/test-features.sh
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo package --all-features
python3 scripts/check_package.py
```

The test script exhaustively tests compact feature combinations and legacy
aliases, checks isolated taxonomy integrations, and runs taxonomy-only and
all-feature tests plus documentation examples. CI runs the full suite on stable
and the MSRV, with compilation checks on beta/nightly. Source fixtures run offline
from a repository checkout.


## Releases

Update the changelog and versioned documentation links, run the checks above,
and publish from the reviewed default-branch commit. The compressed package must
remain below 8 MiB with runtime data and complete attribution retained.
