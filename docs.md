# Docs

## Tool: Zensical

Documentation sites use **Zensical** (a Material-for-MkDocs successor), deployed to GitHub Pages. Every project's docs look and build the same way.

Files at repo root:

- `mkdocs.yml` - Zensical-compatible config. Do **not** set `theme.name: material`; for new sites prefer:
  ```yaml
  theme:
    variant: modern
  ```
  or `variant: classic` where a Material-like look is more important than the newer Zensical look. Configure the deployment URL, nav, repository/edit metadata, `extra_css`, logo/favicon, and any explicit `markdown_extensions` the site really needs. Do not carry forward Material-only `palette:` blocks or `theme.features:` (`navigation.*`, `content.action.*`, dark/light toggles, etc.) unless Zensical has gained support and the build has proved it.
- `requirements.txt` - pins `zensical` (use the version a current repo pins; confirm the latest before writing - do not pin from memory). Do not keep `mkdocs`, `mkdocs-material`, or unsupported `mkdocs-*` plugin packages unless the repo has a deliberate non-Zensical test fixture or application dependency.
- `docs/` - the markdown tree, plus `docs/stylesheets/extra.css` for brand colour and `docs/assets/` for logo/favicon.

## Zensical compatibility rules

These rules come from a cross-project migration sweep. They are part of house style because they catch the failures that make a site appear "mostly migrated" while still breaking in CI.

### No MkDocs plugin system

Zensical currently has no general MkDocs plugin system. In `plugins:`, keep only things Zensical supports:

- `search` (built in)
- `mkdocstrings` where needed
- Zensical-native extensions/features such as GLightbox or macros, configured as Zensical documents them

Comment out unsupported MkDocs plugins instead of leaving them active. Preserve their config under comments so the feature can be restored when Zensical supports it:

```yaml
plugins:
  - search
  # Plugins below are suspended pending Zensical module system support.
  # Re-enable each once the corresponding Zensical issue is resolved.
  #
  # - git-committers:       # https://github.com/zensical/backlog/issues/17
  #     repository: org/repo
  #     branch: main
  #
  # - git-revision-date-localized:  # https://github.com/zensical/backlog/issues/18
  #     enable_creation_date: true
  #
  # - with-pdf:             # https://github.com/zensical/backlog/issues/25
  #     cover_title: Project docs
```

Common mappings:

- `git-committers` -> suspend, backlog #17
- `git-revision-date-localized` -> suspend, backlog #18
- `with-pdf` -> suspend, backlog #25
- `llmstxt` -> suspend, backlog #92
- `macros` as a MkDocs plugin -> replace with `zensical.extensions.macros` under `markdown_extensions:`
- `tags`, `awesome-pages`, `include-markdown`, `gen-files`, `literate-nav` -> no house-style default; comment out unless the repo has proved a current Zensical equivalent

If commenting out a plugin would materially change navigation or generated content, write that explicitly into the migration notes rather than silently dropping it.

### Markdown extensions: defaults are wider than MkDocs

A Zensical config with no `markdown_extensions:` gets a wider default Python-Markdown/PyMdown set than plain MkDocs. That is usually what we want, but during migration scan for content that may be transformed unexpectedly:

- `pymdownx.magiclink` auto-links bare URLs, emails, GitHub users/repos
- `pymdownx.smartsymbols` converts `->`, `!=`, `--`, `...`, etc.
- `pymdownx.caret`, `pymdownx.mark`, `pymdownx.tilde` enable `^sup^`, `==mark==`, `~~del~~`, `~sub~`

Do not reflexively set `markdown_extensions: {}`. Keep Zensical defaults unless the scan finds a real conflict. If a repo already has explicit `markdown_extensions:`, those entries are the contract; review them for stale Material paths such as:

```yaml
emoji_index: !!python/name:zensical.extensions.emoji.twemoji
emoji_generator: !!python/name:zensical.extensions.emoji.to_svg
```

### Migration sweep checks

Before calling a migration done, run a grep sweep that proves the site is not half-migrated:

- no active `theme.name: material`
- no `material.extensions.emoji.*`
- no active `mkdocs`, `mkdocs-material`, or unsupported `mkdocs-*` packages in requirements
- no `mkdocs serve`, `mkdocs build`, or `mkdocs gh-deploy` in workflows, scripts, Dockerfiles, or docs commands
- no active unsupported plugins under `plugins:`

Known exceptions must be named explicitly: test fixtures, generated build artifacts (`target/`, `site/`), or application code that deliberately embeds MkDocs as a library.

## Build and deploy

`.github/workflows/docs.yml` (preferred name for new repos; sibling repos may still use the older `deploy-docs-to-ghpages.yml` - keep matching the repo's existing name). Triggered on push to `main`, with **path filters so the workflow only fires when something the site actually depends on has changed**, plus `workflow_dispatch` for manual runs.

### Method: artifact-based, NOT a `gh-pages` branch

Deploy with `actions/upload-pages-artifact` + `actions/deploy-pages`. The built site is handed straight to GitHub's Pages servers as a workflow artifact. **There is no `gh-pages` branch**, no orphan history, nothing in `git log --all` that did not come from a real commit.

Do not use `peaceiris/actions-gh-pages` or `JamesIves/github-pages-deploy-action` - those write to a `gh-pages` branch. Do not use `mkdocs gh-deploy` either: Zensical has no `gh-deploy` command, and branch-pushing deploys are not house style.

### One-off repo prerequisite

**Settings → Pages → Source must be set to "GitHub Actions"** (not "Deploy from a branch"). If it is set to the default branch source, `actions/deploy-pages` fails with `Get Pages site failed`. Set this once per repo; it persists.

### Reference workflow

Use this shape after verifying each action's current release and SHA. The header comment is the documentation:

```yaml
# Builds and deploys the <project> documentation site to GitHub Pages.
#
# Method: artifact-based deploy (actions/upload-pages-artifact +
# actions/deploy-pages). The built site is handed straight to GitHub's Pages
# servers as a workflow artifact - there is no `gh-pages` branch, no orphan
# history, nothing in `git log --all` that did not come from a real commit.
#
# Repo prerequisite (one-off): Settings -> Pages -> Source must be set to
# "GitHub Actions". If it is set to "Deploy from a branch", deploy-pages
# fails with "Get Pages site failed". This is set once on the repo and
# persists.
#
# Action SHAs are copied from the sibling repos' docs workflows (the canonical
# living examples) and carry the matching `# vX.Y.Z` comment per house style.

name: docs

on:
  push:
    branches:
      - main
    # Path filters so the workflow only fires when something the site
    # actually depends on has changed - not on every README tweak or Rust
    # commit. Keep this list in sync with what zensical actually reads:
    # docs/, examples/ (when snippets pull from them), the mkdocs config,
    # the pinned requirements, and this workflow file itself.
    paths:
      - "docs/**"
      - "examples/**"
      - "mkdocs.yml"
      - "requirements.txt"
      - ".github/workflows/docs.yml"
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: pages
  cancel-in-progress: true

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
        with:
          persist-credentials: false
      - uses: actions/setup-python@a309ff8b426b58ec0e2a45f0f869d46889d02405 # v6.2.0
        with:
          python-version: "3.x"
      - uses: actions/configure-pages@45bfe0192ca1faeb007ade9deae92b16b8254a0d # v6.0.0
      - run: echo "cache_id=$(date --utc '+%V')" >> $GITHUB_ENV
      - uses: actions/cache@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0
        with:
          key: zensical-${{ env.cache_id }}
          path: ~/.cache
          restore-keys: |
            zensical-
      - run: pip install -r requirements.txt
      - run: zensical build --clean
      - uses: actions/upload-pages-artifact@fc324d3547104276b827a68afc52ff2a11cc49c9 # v5.0.0
        with:
          path: site

  deploy:
    needs: build
    runs-on: ubuntu-latest
    permissions:
      pages: write
      id-token: write
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    steps:
      - id: deployment
        uses: actions/deploy-pages@cd2ce8fcbc39b97be8ca5fce6e763baed58fa128 # v5.0.0
```

Configure the site URL for the chosen GitHub Pages location; add a `CNAME` file under `docs/` for a custom domain. SHA-pinning rule from [ci.md](ci.md) applies to every action above; bump SHAs only after fetching the action's repo to confirm the current latest tag (do not pin from memory).

## Local preview

`s/docs` runs `zensical serve` (greenbook uses `zensical serve --open`). Keep it dependency-light - no Docker just to preview docs.

**Bind the first free port in a range (8000-8030), not a fixed one.** Working across several repos you often have more than one docs server up, and a hard-coded 8000 means the second `s/docs` dies with "address already in use". `s/docs` should scan the range, serve on the first free port, print the chosen `http://localhost:<port>` so the dynamic port is discoverable, and still honour a caller-supplied `-a`/`--dev-addr` override.

**Detect the free port across both IP families.** zensical binds `localhost`, which on many systems resolves to IPv6 `::1`, so an IPv4-only connect probe (`/dev/tcp/127.0.0.1/<port>`) misses an existing IPv6 listener and hands back a port that is actually taken. Use `ss -ltn` (Linux) or `lsof -iTCP -sTCP:LISTEN` (macOS/BSD) - they see every listener regardless of address or family - and keep the connect probe only as a last-resort fallback.

If the docs site is intentionally containerised, use a Docker Compose wrapper that starts the docs service, waits for the port to become reachable, and opens the browser. Copy [examples/s/docs-docker-compose](examples/s/docs-docker-compose) rather than asking contributors to remember the compose incantation.

## Documentation operations

For documentation-heavy repos, expose quality and export tasks as `s/` scripts:

- `s/check-docs-nav` - fail when a page is missing from the nav, or the nav points at a page that does not exist.
- `s/linkcheck` - build the site and check local links; add remote-link checking only where it is reliable enough for the project.
- `s/spellcheck` - run codespell or the repo's chosen spelling checker over docs and spec prose.
- `s/markdownlint` - run the checked-in Markdown linter config.
- `s/build-pdf` - build an offline/PDF artifact where the audience or governance process needs one.

Do not require every repo to carry all of these. A CLI with a small docs site may only need `s/docs`; a clinical safety or public documentation repo may need the full set.

**Nav orphans need their own check, because the build will not tell you about them.** Zensical and MkDocs both render a page whether or not the nav references it, so an orphaned page builds cleanly, the build reports no issues, and the page ends up reachable only by guessing its URL. Every docs site with an explicit `nav:` should carry this check. Wire it into the *CI* workflow rather than the docs-deploy workflow: deploy usually runs only on `main`, and the check needs to block the pull request that introduces the orphan. Keep it dependency-free - nav entries are the only `*.md` paths in `mkdocs.yml`, so matching those needs no YAML parser and runs in a bare container. Where a page is deliberately unlisted (command aliases that would duplicate a nav entry, historical transcripts), record it as a named exception carrying a reason rather than loosening the check, and fail when an exception names a page that no longer exists, so the exception list cannot rot.

## Content conventions

- Navigation grouped by task, not by source file.
- One page per CLI command/subcommand; for CLI tools include runnable demos (copy-pasteable `clincalc <name> ...` invocations with their output).
- An `edit`/`view` action link back to the file on GitHub (`content.action.edit`).
- A domain glossary where the project has its own language, alongside a `CONTEXT.md` where useful.
- Host the install one-liner proxies (`install.sh`, `install.ps1`) at the site root - see [distribution.md](distribution.md).
- A demo or presentation sub-app can live under `docs/demo/` or `docs/presentation/` as static assets linked from the nav (greenbook).
- For DCB0129-style clinical safety docs, use the Zensical/PDF pattern in [clinical-safety.md](clinical-safety.md).
