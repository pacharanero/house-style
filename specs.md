# Specs And Conformance

Use `spec/` for product and domain decisions that need to outlive a chat, issue, or PR. A good spec folder is the place an agent or new contributor can read the shape of the project before touching code.

## `spec/` layout

For projects with meaningful domain or architecture decisions, use:

```text
spec/
|-- README.md
|-- standard.md
|-- roadmap.md
|-- queries.md
`-- ubiquitous-language.md
```

Use only the files the project needs. Small repos do not need ceremony.

- `README.md` is the index and reading order.
- `standard.md` or domain-specific files record the actual design/specification.
- `roadmap.md` tracks domain milestones and gaps between spec and implementation.
- `queries.md` holds open questions that need human decisions.
- `ubiquitous-language.md` records canonical terms, aliases, and terms to avoid.

Engineering task lists can live in a root `ROADMAP.md`; domain backlog can live under `spec/`. Be explicit about which file owns which queue.

## Status markers in roadmaps

Mark every roadmap item with a task checkbox so status is visible at a glance and the done/remaining split needs no prose:

- `[x]` done
- `[~]` in progress or partially done
- `[ ]` not started

Put a one-line legend near the top of the file. GitHub renders `[ ]` and `[x]` as real checkboxes and rolls them up in the UI; `[~]` is a house extension for partial progress and shows literally, which is fine. Keep standing principles (guiding constraints, invariants) as plain bullets - they are not tasks, so they carry no box.

## Stable roadmap item codes

Give every actionable roadmap item a short, stable identifier so a human or agent can be told `complete R12`, a commit can say `implements R12`, and later discussion still points to the same work.

Use `R<number>` by default:

```markdown
Legend: [x] done, [~] in progress, [ ] not started

- [x] **R1 - Establish the initial schema**
- [~] **R2 - Add conformance fixtures**
- [ ] **R3 - Publish the reference implementation**
```

Rules:

- Assign the next unused number when an item is created.
- Never renumber items when priorities change, an item moves, or an earlier item is removed. Gaps are harmless; changing identity is not.
- Never reuse the code of a deleted, superseded, or completed item for different work.
- Put the code before the title so it remains visible in long roadmaps and search results.
- Keep the code when changing the checkbox. The code identifies the work; the checkbox records its current status.
- Refer to the code exactly in instructions, commits, PRs, queries, and decision notes: `complete R12`, `blocks R7`, `supersedes R4`.
- If the roadmap has genuinely independent tracks, use a short stable prefix such as `CLI-12`, `DOCS-7`, or `SAFE-3`. Pick one scheme per track and document it in the legend.
- Avoid bare `#12` as the canonical written form in GitHub-hosted Markdown because GitHub may interpret it as issue 12. Where a roadmap item also has a GitHub issue, write both explicitly: `R12 (GitHub #84)`.

## Ubiquitous language

When a project has its own domain vocabulary, write it down. Follow the `gitehr` / `greenbook` style:

- Canonical term.
- Plain definition.
- Examples where helpful.
- Aliases or terms to avoid.

Use the canonical terms in CLI commands, docs, code identifiers, UI labels, tests, and agent instructions. Do not let three words for the same thing drift across surfaces.

## Conformance suites

When behaviour must be shared across implementations, define it as data:

```text
conformance/
|-- README.md
|-- cases.json
|-- fixtures/
`-- expected/
```

or, where it is purely test-owned:

```text
tests/conformance/
|-- README.md
|-- fixtures/
`-- expected/
```

The conformance suite should explain:

- What each case names.
- Where fixtures come from.
- What output is expected.
- Which implementation generates goldens, if any.
- How every implementation runs the same cases.

When behaviour changes intentionally, regenerate goldens and review the expected-output diff in the same change. A golden update without a rationale is a smell.

## Reference implementation

For multi-implementation projects, name the reference implementation. Other implementations should validate against the same conformance data rather than copying tests by hand.

Keep the test harness thin and boring: load case manifest, load fixture, run implementation, compare expected output semantically. Avoid brittle byte comparisons where object key order is irrelevant.

## Specs before code, but not instead of code

Use specs to make decisions clear, not to create a second stale product. If the code intentionally diverges from the spec, update the spec in the same PR or open a named follow-up in `spec/queries.md` or the roadmap.

## Optional governance artefacts

Some projects need lightweight architecture or governance artefacts: ADRs, risk registers, traceability matrices, or review packs. Do not add these to every repo by default.

When they are useful, start from the curated examples:

- [examples/architecture/adr-template.md](examples/architecture/adr-template.md)
- [examples/architecture/risk-register-template.md](examples/architecture/risk-register-template.md)
- [examples/architecture/traceability-matrix-template.md](examples/architecture/traceability-matrix-template.md)

ArcKit has a much larger template catalogue. House style deliberately keeps only a small optional subset here.
