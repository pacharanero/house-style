# Scripts (`s/`)

Every repo gets a top-level `s/` directory of tiny shell scripts that name the **repeated processes** of working on the project. Each one is a one-liner-ish wrapper round a real tool. The script is the canonical way to invoke the process, so a fresh clone (human or agent) finds the right entry point by listing one directory.

## Why `s/`

- **Discoverable**: `ls s/` is a project's user-facing verb list. No README hunt, no Makefile to read.
- **Tiny name to type**: `s/docs`, `s/test`. Six keystrokes. That matters when you run it a dozen times an hour.
- **Standard across repos**: the same names mean the same things in every project, so a clinician/engineer/agent moving between repos doesn't relearn.
- **Not `scripts/`**: the directory is `s/` for terseness. This is deliberate.
- **Not Make / npm-run / just**: one canonical wrapper per process, not a task runner layered over them. Hosts that need a task runner (Tauri's `vite`, a Docker compose stack) hide behind a single `s/` script that calls the underlying tool.

## Shape of a script

A good `s/` script is short, readable in one screen, and does one thing.

Copy from [examples/s/docs-zensical](examples/s/docs-zensical), [examples/s/docs-docker-compose](examples/s/docs-docker-compose), [examples/s/install-hooks](examples/s/install-hooks), or [examples/s/version++](examples/s/version++) when a repo needs that wrapper and there is no closer local script. Examples are intentionally complete, including preflights and comments, so agents should adapt them rather than inventing a new shape.

```bash
#!/usr/bin/env bash
#
# One-paragraph description of what this script does and when to run it.
# Note any one-off setup the reader has to do once (venv, toolchain).
#
# Common gotchas (inotify limits, ports already in use, etc) go here so
# the reader doesn't have to find them by failing.

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

# Any preflight checks (binary present, port free, venv installed) with
# clear error messages that tell the reader exactly how to fix them.

exec <underlying-tool> "$@"
```

Rules:

1. **Bash**, `#!/usr/bin/env bash`, `set -euo pipefail`.
2. **Executable** (`chmod +x`) and committed with the executable bit on. Repos using `core.fileMode = false` should still commit them executable.
3. **CD to the repo root** with `cd "$(git rev-parse --show-toplevel)"` so the script works regardless of where it is invoked from. For scripts that must work outside a git checkout, fall back to the `BASH_SOURCE` pattern.
4. **`exec` the underlying tool** so the script transparently replaces itself - signals, exit code, and TTY all pass through.
5. **Forward arguments with `"$@"`** so the underlying tool's flags are reachable without wrapping each one.
6. **Header comment** explaining purpose, one-time setup, and common gotchas. The header is the README for the script.
7. **Preflight, don't just fail**: if the script depends on something the reader might not have installed yet (a venv, a binary, a Docker daemon, an open port), check for it and emit a copy-pasteable fix.
8. **SPDX header optional** on trivial wrappers; required on anything that contains non-trivial logic (the `sct` convention).

## Canonical script names

The same names mean the same things in every repo. Add as needed; don't reach for these names for unrelated jobs.

| Name | Purpose |
|---|---|
| `s/test` | Run the project's full test suite (`cargo test`, `pytest`, whatever). |
| `s/build` | Build the project (typically a release build). |
| `s/docs` | **Serve the documentation site locally and open the browser on the right port.** Single key process for docs development - see below. |
| `s/install` | Install the local build onto the user's `PATH` (often `cargo install --path .`). |
| `s/install-hooks` | Opt in to tracked local Git hooks by setting `core.hooksPath=.githooks`. |
| `s/version++` | Bump the version, regenerate the lockfile and changelog, commit, and land on `main` (direct push, or a release PR when `main` is branch-protected). The landing *is* the release: it triggers the auto-tag cascade (see [distribution.md](distribution.md)). The only command needed to ship. **Does not tag locally** - the workflow tags. |
| `s/up` / `s/down` | Bring a Docker Compose stack up / down (Compose-based projects). |
| `s/lint` | Run the project's linter(s) (separate from `s/test`). |

There is deliberately **no `s/release`**. Bumping the version with `s/version++` and landing the release commit on `main` is the release - the auto-tag workflow does the tagging and shipping. The landing may be a direct push or a release PR, depending on branch protection. A separate release script that also tags/pushes was a source of divergence between repos; [distribution.md](distribution.md) is the canonical model.

Project-specific scripts are fine alongside these (`s/generate` in gitehr makes 10,000 test repos; `s/size` reports disk footprint; `s/spellcheck` in dgc-documentation runs codespell). Name them for the verb.

## `s/docs` - the canonical example

Every project with a docs site ships `s/docs`. It is the single key repeated process during documentation work, so it must:

1. **Serve the docs locally** with hot reload (`zensical serve`, `mkdocs serve`, `docker compose up`, whatever the project uses).
2. **Open the browser** on the right port automatically, so the writer doesn't reach for the URL.
3. **Work first-time from a clean clone**, or fail with a precise instruction on how to set up the dependency it needs (typically the docs venv).
4. **Pass arguments through** so the writer can override (e.g. a different port, or skip the browser open on a remote machine).

Two living shapes:

- **Zensical / mkdocs, no container**: a thin shell wrapper around `zensical serve --open`. The browser-open is done by zensical itself via `--open`. Copy [examples/s/docs-zensical](examples/s/docs-zensical).
- **Docker Compose**: `docker compose up` plus an `xdg-open`/`BROWSER` fallback that waits for the port to be reachable before opening.

Pick the shape that matches the docs stack and copy from the closest living example.

## Documentation quality scripts

Documentation-heavy repos should expose the quality checks writers actually run:

| Name | Purpose |
|---|---|
| `s/linkcheck` | Build the docs and check internal links, with optional remote-link checking where the tool supports it. |
| `s/spellcheck` | Run codespell or the repo's chosen spelling checker over docs/spec prose. |
| `s/markdownlint` | Run the repo's Markdown linter with the checked-in config. |
| `s/build-pdf` | Produce the PDF/offline documentation artifact where the project has that requirement. |

Use these when the repo needs them; do not add them to a small code repo with a three-page docs site.

## `s/README.md`

Once a project has more than three scripts, add `s/README.md` listing them with a one-line summary and the most common invocations.

The shape used in `gitehr/s/README.md` and `rcpch/dgc-documentation/s/README.md` is the template:

```markdown
# `s/`

The `s/` directory contains some simple convenience scripts to speed up and standardise working with this repository.

## `s/<name>`

One-line description.

- `s/<name>` - default invocation
- `s/<name> --flag` - common variant
```

Until there are enough scripts to warrant a README, the header comment in each script is the documentation.

## Local Git hooks

Repos that want local pre-commit checks should commit a tracked `.githooks/` directory and an `s/install-hooks` helper:

```bash
#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

git config core.hooksPath .githooks
echo "Git hooks installed: core.hooksPath=.githooks"
```

The standard `.githooks/pre-commit` should run `s/lint`:

```bash
#!/usr/bin/env bash
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

exec s/lint
```

This keeps hooks portable, reviewable, and opt-in per checkout. Do not write directly into `.git/hooks` in the repo setup docs; Git does not track that directory.

Copy [examples/s/install-hooks](examples/s/install-hooks) for the reusable installer shape.

## Anti-patterns

- **A task runner layered over the scripts** (Make, just, npm run, cargo xtask) used as the day-to-day entry point. The scripts are the entry point. If you reach for a task runner, ask whether the script could do it directly.
- **Scripts that don't `cd` to the repo root**, so they only work from one directory.
- **Long flags hidden behind a script name** (e.g. `s/test-with-coverage`, `s/test-only-integration`). Add the flag to the underlying tool and pass it as an argument: `s/test --coverage`. One verb per script.
- **Scripts that silently install things** (a venv, a Docker image) on first run. Print what is about to happen, or refuse and tell the reader to run the one-off setup explicitly. Surprising side effects are worse than a clear error.
- **`scripts/`, `bin/`, `tools/` directories holding the same things.** Use `s/`.
