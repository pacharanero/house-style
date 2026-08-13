# CI

## What CI must enforce (Rust)

A single `ci.yml` running on `push` to `main` and on `pull_request` (plus `workflow_dispatch`), with `permissions: contents: read`:

1. `cargo fmt --all --check`
2. `cargo clippy --all-targets -- -D warnings` (and one run per feature flag where the crate has features, e.g. `--features serve`)
3. `cargo test` (and per-feature where relevant)
4. `cargo audit` - security-advisory scan of the locked dependency tree (see below)
5. REUSE compliance (see [licensing.md](licensing.md)) - `fsfe/reuse-action`
6. `zizmor --strict-collection .` - static security analysis of workflows, local actions, and Dependabot configuration (see below)

Toolchain via `dtolnay/rust-toolchain` (stable, components `rustfmt, clippy`); cache via `Swatinem/rust-cache`. Checks 1-3 are the same ones every contributor runs locally before committing, so CI never surprises.

## GitHub Actions security

Every active repository with GitHub Actions runs [Zizmor](https://zizmor.sh/) as a separate blocking CI job. Use its default `regular` persona: it is designed for high-signal CI findings such as template injection, excessive permissions, persisted credentials, cache poisoning, vulnerable actions, and unpinned or misleading refs. Reserve `pedantic` and `auditor` for manual reviews because they deliberately include code smells and likely false positives.

Install a pinned Zizmor release through `taiki-e/install-action`, which downloads the upstream binary and verifies its checksum. Set `fallback: none` so installation cannot silently switch to cargo-binstall or a source build. Pass the read-only workflow token to enable online audits, including ref/version validation. Pinning both the Action and Zizmor means a new audit rule arrives through a reviewed dependency bump rather than unexpectedly turning every default branch red.

```yaml
  workflow-security:
    name: GitHub Actions security
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
        with:
          persist-credentials: false
      - uses: taiki-e/install-action@a6b2e2dcd845ddd7f509ce4f3ed3d922b80cc5d9 # v2.84.0
        with:
          tool: zizmor@1.27.0
          fallback: none
      - run: zizmor --strict-collection .
        env:
          GH_TOKEN: ${{ github.token }}
```

This job assumes the workflow-level default is `permissions: contents: read`. Every checkout used only to build, test, scan, or package sets `persist-credentials: false`. Give write permissions to the smallest possible job instead of the whole workflow; for example, a Pages build job remains read-only while only its deploy job receives `pages: write` and `id-token: write`.

Zizmor's static analysis does not inspect shell scripts or other files invoked by workflows, and it cannot resolve every runtime-generated value. It complements SHA pinning, review, least privilege, and ordinary code scanning; it does not replace them.

## Security audit (Rust)

Run `cargo audit` as its own CI job so a dependency with a published [RustSec](https://rustsec.org) advisory fails the build - caught on the next push rather than waiting on a Dependabot email (and `cargo audit` catches advisories Dependabot doesn't, e.g. `unsound` ones, and picks up an advisory whose fixed-version range was revised after Dependabot's first alert). Install the prebuilt binary rather than building from source:

```yaml
  audit:
    name: Security audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@<sha> # vX.Y.Z
        with:
          persist-credentials: false
      - uses: taiki-e/install-action@<sha> # vX.Y.Z
        with:
          tool: cargo-audit
          fallback: none
      - run: cargo audit
```

- It is a **blocking gate**, not advisory: a red audit means a vulnerable crate is in `Cargo.lock`. The fix is almost always a lockfile bump (`cargo update -p <crate> --precise <fixed>`; bump the parent when a `0.x` req pins it), then commit `Cargo.lock` as `chore(deps):`.
- A new advisory can turn `main` red with no code change - that is the point. Fix the dependency promptly.
- For a genuinely unfixable transitive advisory (no patched version yet, not reachable in practice), quarantine it in `audit.toml` (`[advisories] ignore = ["RUSTSEC-XXXX-NNNN"]`) with a comment explaining why and a link - never by disabling the job. Re-audit and drop the ignore once a fix ships.
- Keep it as a distinct job (not another step in the test job) so the failure signal is unambiguous and it runs in parallel.
- Keep the workflow small, explicit, and equivalent to the documented local checks.

For broader security workflow rules - authorised scope, secrets hygiene, reproducible scans, and findings handling - see [security.md](security.md).

## GitHub Actions pinning (non-negotiable)

Pin actions to a full commit SHA with a trailing `# vX.Y.Z` comment:

```yaml
uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
uses: dtolnay/rust-toolchain@e97e2d8cc328f1b50210efc529dca0028893a2d9 # v1
uses: taiki-e/install-action@a6b2e2dcd845ddd7f509ce4f3ed3d922b80cc5d9 # v2.84.0
```

Before adding or bumping an action, fetch its repo to confirm the current latest stable tag and its SHA - never pin from memory. Use the `pin-github-action` tool's convention.

## Dependabot

`.github/dependabot.yml`: configure version updates for every ecosystem present, including `cargo`, `github-actions`, `npm`, and `pip` where applicable. Dependabot keeps the pinned SHAs and package versions current; CI then proves the bump is safe. See [dependencies.md](dependencies.md) for dependency admission and version/lockfile policy.

Use a cooldown for version updates so a dependency release has time to be noticed, yanked, or superseded before it lands in our repos. The default must be at least 1 day; prefer longer for low-urgency SemVer updates (`semver-patch-days: 3`, `semver-minor-days: 7`, `semver-major-days: 14` is a good baseline). Dependabot cooldown does **not** apply to security updates, so high-severity or otherwise urgent security fixes still arrive immediately.

Batch routine noise. For each ecosystem, group minor and patch **version** updates into a weekly PR instead of opening one PR per dependency. Keep major updates separate unless the repo has a clear reason to batch them. If a monorepo has the same ecosystem in multiple directories, use `directories` plus `groups.<name>.group-by: dependency-name` where that gives one coherent bump instead of repeated directory-by-directory PRs.

Baseline shape:

```yaml
version: 2

updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    cooldown:
      default-days: 1
      semver-patch-days: 3
      semver-minor-days: 7
      semver-major-days: 14
    groups:
      routine-minor-and-patch:
        applies-to: version-updates
        patterns:
          - "*"
        update-types:
          - "minor"
          - "patch"

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    cooldown:
      default-days: 1
    groups:
      github-actions-minor-and-patch:
        applies-to: version-updates
        patterns:
          - "*"
        update-types:
          - "minor"
          - "patch"
```

For JS/Tauri frontends, add an equivalent `npm` block. For Python projects, add a `pip` block. Do not try to encode vulnerability severity into grouping rules; Dependabot grouping does not filter by severity. Treat high/critical security PRs as urgent review work outside the weekly routine-update rhythm.

## Local git hooks (optional but encouraged)

Repos should ship tracked local hooks under `.githooks/`, wired in by an opt-in `s/install-hooks` script:

```bash
git config core.hooksPath .githooks
```

The default pre-commit hook runs `s/lint`, so the same formatting and clippy checks as CI run before the commit is created. Keep this fast: full tests stay in `s/test` and CI unless a repo has a very small test suite. Document the one-line setup in `CONTRIBUTING.md` or `s/README.md`.

## Docs and release CI

Docs deploy and release pipelines are their own workflows - see [docs.md](docs.md) and [distribution.md](distribution.md). Same SHA-pinning rule applies to every one.
