---
name: audit-house-style
description: Audit a local repository against the adopted house-style standards and write a HOUSE-STYLE-AUDIT.md report into the target repo. Use when the user asks to audit, review, check, or assess a repo for house-style compliance, especially via /audit-house-style with a repo path, $audit-house-style with a repo path, or "audit this repo against house style".
---

# Audit House Style

Audit one target repository against the installed or checked-out `pacharanero/house-style` standards and write a concrete repo-local improvement report. Do not fix the target repo in this workflow unless the user explicitly asks for fixes after reading the audit.

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

Completion criterion: the audit can distinguish pre-existing worktree state from the audit file it writes.

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

### 4. Write the audit report

Write exactly one file in the target repo unless the user asks otherwise:

```text
HOUSE-STYLE-AUDIT.md
```

If the filesystem blocks writing outside the current workspace, request permission/escalation rather than writing elsewhere.

Use this structure:

```markdown
# House-Style Audit

Audit date: YYYY-MM-DD

Audited against: `pacharanero/house-style` as of this date, especially ...

Scope: lightweight static audit only. No files were changed except this report.

## Summary

<repo type and overall alignment>

Main improvements:

- ...

## Priority Findings

### P1 - <finding>

Evidence:

- `<path>` ...

House style:

- `<standard>.md` says ...

Suggested change:

- ...

## Compliant / Good Patterns

- ...

## Not Applicable

- ...

## Suggested First PR

1. ...

## Suggested Second PR

1. ...
```

Priority scale:

- `P1` - security/supply-chain/release/CI issue, or a gap that blocks safe maintenance.
- `P2` - important workflow or standards gap that should be fixed soon.
- `P3` - polish, consistency, or worthwhile future enforcement.

Completion criterion: the report is actionable without reading the conversation that produced it.

### 5. Validate

Run a whitespace check on the audit file:

```sh
git -C <target> diff --check -- HOUSE-STYLE-AUDIT.md
```

Then check target status and mention any pre-existing dirty/untracked files separately from the audit file.

Completion criterion: the audit file passes `git diff --check`, and the final response names the file written plus the main findings.

## Guardrails

- Do not make compliance fixes during the audit.
- Do not overwrite an existing `HOUSE-STYLE-AUDIT.md` without reading it first and preserving useful previous content.
- Do not revert or clean unrelated target-repo changes.
- Do not treat house style as one-size-fits-all; content repos, clinical repos, CLI repos, and presentation repos have different applicable standards.
- Prefer concrete evidence over generic advice.
- Keep "Not applicable" explicit so future agents do not waste time applying irrelevant standards.
