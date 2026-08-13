# Testing

How tests are structured and the patterns worth reaching for. Tests run via `s/test` (`cargo test`) and are one of the three checks CI enforces, alongside `cargo fmt --check` and `cargo clippy -- -D warnings` (see [ci.md](ci.md), [scripts.md](scripts.md)). Never commit red. Unit tests live in-module under `#[cfg(test)]`; cross-binary and integration tests live in `tests/`.

## Registry / policy enforcement tests

When a crate has a registry of things that must *all* satisfy a contract, write one test that iterates the registry and asserts the contract for every member. It turns "remember to..." into a compile-and-test guarantee and scales for free as the registry grows - a new member that forgets the contract fails the suite, not code review.

> [`pacharanero/clincalc`](https://github.com/pacharanero/clincalc)'s `every_calculator_records_its_license` iterates `all()` and asserts each calculator returns a non-empty licence and an `http(s)` evidence URL. Adding a calculator without one is a test failure.

## Golden / literature-vector tests

For anything implementing a published method or a spec, test against known input-to-output vectors from the source, not just internal consistency. Encode the vectors as unit tests that cite where they came from, so the test doubles as provenance.

> Each clinical calculator is tested against the band boundaries and worked examples in its primary literature, with the citation in the test.

Keep the logic under test **pure** - no clock, no I/O, no global state; a host stamps timestamps and does I/O when it records a result. Pure logic is deterministic and trivially testable, and it is the same leaf discipline that makes a crate extractable (see [library-extraction.md](library-extraction.md)).

## Conformance suites

When behaviour must be identical across multiple implementations, put the contract in a conformance suite rather than duplicating tests per language. The suite is data: case manifest, fixtures, and expected outputs. Each implementation loads the same cases and compares its own output against the same expected data.

Use [specs.md](specs.md) for the repo shape and documentation rules. In tests, compare semantically where ordering does not matter; do not make JSON object key order or formatting part of the contract unless that is genuinely the feature under test.

Regenerate goldens only for intentional behaviour changes, and review the expected-output diff in the same change.

## Isolation for filesystem and git tests

Tests that touch the working directory or shell out to git must not depend on the developer's environment or on each other:

- Use `tempfile::tempdir()` and operate inside it. Mark tests `#[serial]` (`serial_test`) when they `set_current_dir`, so they do not race each other.
- **Disable commit signing inside the test's throwaway repo** with `git config commit.gpgsign false`. Otherwise a contributor whose global config signs commits (GPG/SSH) gets spurious failures in any test that git-commits. This is a real, repeated failure - bake the line into the test's `git init` helper, not into each test.
- Assert on observable behaviour (files created, stdout, exit code), not on internal helper functions that churn. When a refactor removes a feature, delete its tests rather than faking coverage - a test suite that lies is worse than a smaller honest one.
