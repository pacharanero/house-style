---
name: audit-house-style
description: Audit a local repository against the adopted house-style standards and add actionable findings to the repository's active roadmap. Use when the user asks to audit, review, check, or assess a repo for house-style compliance, especially via /audit-house-style with a repo path, $audit-house-style with a repo path, or "audit this repo against house style".
---

# Audit House Style

Audit one target repository against the installed or checked-out `pacharanero/house-style` standards and add concrete improvement tasks to the repository's active roadmap. Do not fix the target repo in this workflow unless the user explicitly asks for fixes after reading the audit.

## Inputs

Require a target repo path. If the user omits it, ask for it before auditing.

Expand `~` and relative paths. Verify the target is a Git repo or a plausible project directory before proceeding.

## Read First

1. Read the adopted house-style `AGENTS.md`, locating it from the installed skill or its checked-out `pacharanero/house-style` repository.
2. Read only the house-style standards relevant to the target repo.
3. Read the target repo's existing guidance first: `AGENTS.md`, `CLAUDE.md`, `agent-instructions.md`, `README.md`, `CONTEXT.md`, `spec/README.md`, `ROADMAP.md`, or equivalents where present.

Completion criterion: know what kind of repo this is and which standards apply.

## Workflow

### 1. Record repository state

Check:

- `git status --short --branch`
- top-level files and directories, excluding generated/cache/vendor directories
- remotes, if relevant

Note existing dirty or untracked files. Do not modify them.

Completion criterion: the audit can distinguish pre-existing worktree state from the roadmap changes it writes.

### 2. Inspect applicable standards

Choose standards by repo shape:

- All repos: `AGENTS.md`, `new-repos.md`, `agents.md`, `security.md`, `licensing.md`, `scripts.md`, `repo-presentation.md`.
- Docs/content sites: `docs.md`, `specs.md`, `ci.md`.
- Rust CLIs/libraries: `rust-cli.md`, `testing.md`, `distribution.md`, `commits.md`, `library-extraction.md`.
- Clinical/health software: `clinical-safety.md`.
- Presentation repos: `presentations.md`.
- Skill repos: `skills.md`.
- Web and frontend apps: `ui.md`.
- Tauri apps: `tauri-gui.md`, `ui.md`.

Do not force irrelevant standards onto the repo. Mark them "Not applicable" instead.

Completion criterion: every finding is tied to a specific applicable standard or explicitly marked as a pragmatic recommendation.

### 3. Gather evidence from the target repo

Inspect concrete files before writing findings. Useful checks include:

- agent instructions: `AGENTS.md`, `CLAUDE.md`, `agent-instructions.md`
- scripts: `s/`, `scripts/`, `Makefile`, package scripts
- CI: `.github/workflows/*`, action pinning, permissions, Pages deploy
- dependency automation: `.github/dependabot.yml`
- docs: `mkdocs.yml`, `docs/`, `requirements.txt`, Pages workflow
  - for Zensical sites specifically, check for half-migrated MkDocs/Material leftovers: active `theme.name: material`, `material.extensions.emoji.*`, active `mkdocs`/`mkdocs-material`/unsupported `mkdocs-*` requirements, `mkdocs serve`/`mkdocs build`/`mkdocs gh-deploy` commands, and unsupported plugins under `plugins:` (see `docs.md`)
- licensing: `LICENSE`, `LICENCE`, `REUSE.toml`, SPDX headers where relevant
- security: `SECURITY.md`, secrets patterns, `.gitignore`
- specs/domain docs: `spec/`, `CONTEXT.md`, `ROADMAP.md`
- release model: `s/version++`, `cliff.toml`, cargo-dist workflows
- content policy tests: scripts or tests that enforce repeated-item contracts
- frontend quality: responsive layouts, realistic loading/empty/error states, keyboard focus, target sizes, reduced motion, relevant assets, and evidence of rendered mobile/tablet/desktop verification
- repo presentation: README structure, quick start prominence, informative badges, hero visual, honest status, maintained content, and repo metadata (description, topics, social preview)

Use fast tools: `find`, `rg`, `git ls-files`, `sed`. Avoid crawling huge generated directories such as `target/`, `node_modules/`, `site/`, `.cache/`, `.venv/`, `dist/`, and `.git/`.

Completion criterion: each proposed improvement cites target repo evidence with file paths.

### 4. Find the active roadmap

Look for the roadmap named by the target repository's agent instructions, README, spec index, or contribution guidance. Common locations are `spec/roadmap.md`, `ROADMAP.md`, `roadmap.md`, and `docs/roadmap.md`.

- If guidance identifies one roadmap, use it.
- If there is exactly one plausible roadmap, use it.
- If several roadmaps exist, determine which owns repository improvement work from their scope and references. Ask the user if ownership remains ambiguous.
- If no roadmap exists, ask the user where findings should go. Offer `spec/roadmap.md` when the repository already uses `spec/`, otherwise `ROADMAP.md`.

Do not create `HOUSE-STYLE-AUDIT.md` or another standalone audit report.

Completion criterion: one existing or user-approved roadmap is identified as the destination.

### 5. Add audit findings to the roadmap

Modify exactly one roadmap file unless the user asks otherwise. Preserve its existing structure and tasks. Add or update a dedicated `## House-Style Audit` section containing:

- Audit date and the applicable standards reviewed.
- One short paragraph noting the repository's strongest compliant patterns.
- Priority subsections where useful: `P1`, `P2`, and `P3`.
- One standard roadmap checkbox per actionable finding, with a stable `HS-<number>` identifier.
- A short non-checkbox list of deliberate exceptions and non-applicable standards where that prevents future churn.

Use this task shape:

```markdown
Legend: [x] done, [~] in progress or partially done, [ ] not started

## House-Style Audit

Audit date: YYYY-MM-DD

### P1 - Safety, Supply Chain, Release, And CI

- [ ] **HS-1 - <actionable title>.** <Concrete repository evidence>. <What the applicable house-style standard requires>. Done when <observable completion criteria>.
```

Roadmap rules:

- Add the standard checkbox legend near the top of the roadmap if it does not already have one.
- Keep each finding actionable without requiring the original conversation or a separate report.
- Cite concrete paths and relevant external settings in the task text.
- Use `[ ]` for unresolved work and `[~]` only when evidence shows meaningful partial completion.
- Preserve existing `HS-*` identifiers and assign the next unused number. Never renumber or reuse identifiers.
- Do not reset a completed `[x]` item during a later audit unless the completed control demonstrably regressed; create a new item that references the regression instead.
- Update an existing open task when the new finding is the same underlying gap; do not append duplicates on every audit.
- Keep compliant patterns concise and do not turn them into tasks.

Priority scale:

- `P1` - security/supply-chain/release/CI issue, or a gap that blocks safe maintenance.
- `P2` - important workflow or standards gap that should be fixed soon.
- `P3` - polish, consistency, or worthwhile future enforcement.

Completion criterion: every unresolved finding is represented by one stable, checkable roadmap item and no standalone audit file is created.

### 6. Validate

Run a whitespace check on the roadmap file:

```sh
git -C <target> diff --check -- <roadmap-path>
```

Run the repository's Markdown lint and spelling checks against the roadmap when available. Then check target status and mention pre-existing dirty or untracked files separately from the roadmap update.

Completion criterion: the roadmap passes available checks, and the final response names the roadmap updated plus the main findings.

## Guardrails

- Do not make compliance fixes during the audit.
- If a legacy `HOUSE-STYLE-AUDIT.md` exists, read it and migrate still-relevant open findings into the roadmap. Remove the legacy file only when the user requests cleanup or its useful content is demonstrably preserved.
- Do not revert or clean unrelated target-repo changes.
- Do not treat house style as one-size-fits-all; content repos, clinical repos, CLI repos, and presentation repos have different applicable standards.
- Prefer concrete evidence over generic advice.
- Keep "Not applicable" explicit so future agents do not waste time applying irrelevant standards.
