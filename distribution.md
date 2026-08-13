# Distribution

## The release model: one command, `s/version++`

**There is exactly one release action, and it is one command: `s/version++ [patch|minor|major]`** (defaults to `patch`). The maintainer's entire job is choosing patch / minor / major; bump, changelog, tag, multi-platform build, GitHub Release notes, crates.io, and package-manager updates all follow. This is the contract, and it is what repos most often get wrong (some had `s/version++`, others `s/release`, doing overlapping and contradictory work).

### Convergence target: CI auto-tag cascade

For Rust CLI repos expected to become multi-contributor, the target model is the **CI auto-tag cascade**:

1. **Gate**: refuse unless starting from `main` with a clean tree, then run the pre-release checks locally (or trust CI if branch protection gates `main`). A broken or unformatted tree never ships.
2. Bump all version-bearing files (`cargo set-version --bump`, plus project-specific files such as Tauri/npm metadata), regenerate `CHANGELOG.md` (git-cliff, below), and commit `chore(release): vX.Y.Z`.
3. **Land the release commit on `main`**: direct push when `main` allows it, or a release PR (`release/vX.Y.Z`) when `main` is branch-protected.
4. `auto-tag.yml` detects the version bump on `main`, creates the annotated `vX.Y.Z` tag, then invokes `release.yml` via `workflow_call` with that tag. The release workflow builds binaries, creates the GitHub Release, publishes crates / taps, etc.

Why this is now the convergence target:

- **Branch-protection compatible.** The same `s/version++` works before and after `main` is protected; the only change is whether the bump lands by direct push or PR merge.
- **Release authority can live in CI.** When `main` is protected, the tag is minted after the reviewed merge, not on a maintainer's laptop.
- **No PAT.** `auto-tag.yml` uses the default `GITHUB_TOKEN` to create the tag and invokes the release workflow directly via `workflow_call`, avoiding the GitHub anti-recursion rule for tag-triggered workflows.
- **One command, one process.** There is still no separate `s/release`; landing the version bump on `main` is the release.

Use a project-local, reviewed `s/version++` implementation; do not copy an unpublished implementation blindly.

### Local-tag model (legacy / explicit exception)

The older model: `s/version++` commits the bump, tags locally **after** the commit, then pushes `main` and the tag. It remains acceptable for repos that intentionally stay single-maintainer and direct-push-to-main, or for release engines that cannot be invoked via `workflow_call`. Its advantages are that it is synchronous and simple for unprotected `main`; its disadvantage is that it does not survive branch protection cleanly.

For cargo-dist repos, regenerate with a dist version/config that emits `workflow_call` before moving to the cascade. If the generated workflow is tag-push-only, either keep local-tag as a named exception or add a PAT-based tag push from CI (not preferred).

### The hazard every version script must avoid: tag AFTER commit

`cargo set-version` followed by `git tag` *without committing first* tags the **previous** commit - the bump is left uncommitted, so the tag points at code that does not contain the new version. More than one repo shipped this bug. Commit the bump + changelog, *then* tag. (In the local-tag default the script does this directly; in the cascade the workflow tags after the bump lands on `main`.)

### Cascade implementation requirements

The cascade has two required workflow pieces:

1. `auto-tag.yml`: listens to `push: branches: [main]`, detects that the version in `Cargo.toml` changed, creates the matching annotated `vX.Y.Z` tag, then invokes the release workflow.
2. `release.yml`: exposes `workflow_call` with a `tag` input and builds/releases from that tag. It may also keep a `push: tags` trigger so manual tag pushes and the cascade converge on the same pipeline.

Do not rely on the tag-push event from `auto-tag.yml`: a tag pushed with the default `GITHUB_TOKEN` does **not** trigger downstream workflows because of GitHub's anti-recursion guard. Either invoke via `workflow_call` (preferred) or use a PAT to push the tag (not preferred).

Older cargo-dist-generated workflows were tag-driven only; do not move those repos to the cascade until `release.yml` has `workflow_call`. Current cargo-dist workflows can be `workflow_call`-able (as in `gitehr` and `dsc`), so cargo-dist repos can converge on the cascade once their release workflow has the right trigger. `dsc` has completed this migration.

### Branch protection on `main` (the cascade's native habitat)

The cascade is the model that survives branch protection on `main`; local-tag does not. When `main` rejects direct pushes and accepts only PR merges:

- `s/version++` lands the bump via a **release PR**, not a direct push: it branches `release/vX.Y.Z` off `main`, pushes the branch, and `gh pr create`s against `main`. Merging the PR is the `push: branches: [main]` event `auto-tag.yml` listens for, so the tag + release happen exactly as before - no change to the cascade itself.
- The local-tag default breaks here: the tag has to follow the *merge* commit, which is a reviewed, server-side event whose SHA you do not control from a laptop. "Run one command, tag locally, ship" degrades to "open a PR, merge it, then tag the merge commit in a second step." This is the single biggest reason to move a repo to the cascade when you turn on branch protection.
- The script **auto-detects** the mode by probing `gh api repos/<owner>/<repo>/branches/main/protection`: protected -> release-PR mode, unprotected -> direct push. Force either with `--pr` / `--direct`. `gh` does detection and PR creation; install it if absent.
- **Gate**: with branch protection, CI runs on the release PR and gates the merge, so the local fmt/clippy/test gate becomes belt-and-braces rather than load-bearing. The default script keeps it uniform across modes for safety; to trust PR CI instead, drop the local gate (permitted when CI gates `main`).
- **Merge strategy**: prefer *merge-commit* over *squash* for release PRs. A merge commit preserves the `chore(release): vX.Y.Z` commit, so `git-cliff`'s `skip = true` on `^chore\(release\)` keeps the changelog clean; a squash renames that commit to the PR title and it can leak into the changelog's "Other" bucket.

For repos expected to become multi-contributor, treat the cascade + branch-protection-ready `s/version++` as the convergence target. Keep local-tag only as a named exception for repos that intentionally remain direct-push-to-main.

**One script, whichever model.** There is never a second `s/release` that also bumps/tags/pushes - that overlap is exactly what this standard removes.

### Conventional commits + changelog

Commits are [conventional](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`, `chore(deps):`, `ci:`, ...). `CHANGELOG.md` is generated from them by **git-cliff** in [Keep a Changelog](https://keepachangelog.com/) format, regenerated by `s/version++` at bump time and committed in the same commit - so the tag, and therefore the GitHub Release notes, are always in sync. The changelog must be correct *before* the tag is created, which both models guarantee by regenerating it inside the release commit (which is then tagged).

### `Cargo.lock`

Commit it, in every repo, libraries included. Current Cargo guidance is to track `Cargo.lock` everywhere: it makes CI and release builds reproducible, lets a binary repo be consumed reproducibly as a git dependency, and is simply ignored by downstream crate consumers. Do not gitignore it.

## Engine: cargo-dist

Use **cargo-dist** for building and shipping the binaries. It is declarative, ~20 lines of config, and generates a `workflow_call`-able release workflow, so the matrix is manifest-driven rather than hand-maintained YAML. Review its current documentation and generate project-specific configuration rather than copying an unpublished template.

Standard configuration:

- `installers = ["shell", "powershell", "homebrew", "msi"]`
- `targets` = the five standard triples: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`
- Set `tap` and `publish-jobs` only when the project has a maintained Homebrew distribution channel.
- `allow-dirty = ["ci"]` because we hand-edit the generated workflow to pin action SHAs (below)
- `[profile.dist]` inheriting `release` with `lto = "thin"`

Pin the action SHAs in the generated `release.yml` after generation (cargo-dist emits unpinned tags); `allow-dirty = ["ci"]` stops dist erroring on the drift. The release build does **not** run the test suite, so the gate is `ci.yml` on `main`: keep it green and never release a red `main`.

**Caveat before adopting**: confirm cargo-dist is on a maintained line before betting a new repo on it. If it stalls, use a project-specific hand-rolled matrix only after reviewing the maintenance cost and required distribution targets.

## crates.io

Publishable crates (libraries, and CLIs people may `cargo install`) publish on the `vX.Y.Z` tag via **`publish-crates.yml`**: build, test, then `cargo publish` (`--no-verify --allow-dirty` is fine - the index check regenerates `Cargo.lock`). Needs the `CARGO_REGISTRY_TOKEN` secret. Publish leaf crates before their dependents. `dsc`'s workflow is the reference.

Do not use a separate release automation tool to own version bumps, changelog updates, tags, or crates.io publication as the default house style. The standard release path is still `s/version++` -> `auto-tag.yml` -> cargo-dist / publish workflows.

## cargo-binstall

Add `[package.metadata.binstall]` so `cargo binstall <crate>` finds the release archives (pkg-url/pkg-fmt overrides per platform). `sct`'s `Cargo.toml` is the reference.

## Shell completions in packages and installers

Completion scripts must update when the CLI updates. Manual one-off redirects like `tool completions zsh > "${fpath[1]}/_tool"` are a fallback, not the standard user path.

- Generate completions from the released binary or the same `clap::Command` used by that binary, never from copied scripts.
- Homebrew formulae should install completions as part of `brew install` / `brew upgrade` (for example by using Homebrew's completion helpers to generate from `bin/<tool> completions <shell>`). This makes completions auto-update with the formula.
- Debian/RPM/MSI/Scoop packages should include completion files where the channel has a normal location for them. Package upgrades then replace the completion files alongside the binary.
- `install.sh` and `install.ps1` should run `<tool> completions install` after replacing the binary when the shell can be detected, or print the exact command to run when it cannot. Re-running the installer after a version bump must refresh completions without the user remembering shell-specific paths.
- Keep `<tool> completions <shell>` available for packagers and custom installs, but docs should lead with `tool completions install`.

## Install one-liners

Ship `install.sh` and `install.ps1` that fetch the latest release, verify the SHA256, and install to `~/.local/bin` (or `%LOCALAPPDATA%`). Publish a `SHA256SUMS` alongside the release artifacts and make it the **single source of truth** - installers and package updates should read it rather than recomputing hashes.

## deb / rpm / scoop - only if the audience needs them

cargo-dist does **not** produce Debian/RPM packages or a Scoop manifest. Do not add them by default. Add them only when a project's users actually install that way, keeping any supplementary job small and generated from release `SHA256SUMS` where possible.

## Content and docs repos

A documentation or content site with no shippable binary is exempt from this pipeline. Its "release" may be continuous deployment on merge to `main` (see [docs.md](docs.md)). Keep a Keep-a-Changelog `CHANGELOG.md` for notable content changes, but there is no project version to bump and no tag or binary to build. Where the content itself needs versioning, version the data artifacts (filename or a field in the file), not the repo.
