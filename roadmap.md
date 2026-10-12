# House-style roadmap

Proposed additions to the shared standards. These are work items, not adopted requirements; changes to the normative pages need their own review and worked examples.

Legend: `[ ]` proposed, `[~]` in progress, `[x]` complete. `R` identifiers are stable and are not reused.

## Lessons from ods

Recorded 2026-10-01 after reviewing Oli Evans's [`ods`](https://github.com/olizilla/ods/tree/7f95000). Credit the project when incorporating these ideas. The existing registry-policy, conformance and test-isolation guidance remains the starting point.

- [ ] **R1 - Named behavioural guarantees and hostile-environment verification.** Extend [testing.md](testing.md) and, where appropriate, [specs.md](specs.md) with a compact guarantee-to-test index patterned on `ods/docs/tests.md`. Link authoritative specs rather than copying requirements, exercise each promised format/surface, and prove representative assertions fail. Add worked guidance for temporary HOME/XDG/config/cwd, narrow/non-terminal output, unavailable endpoints and unexpected-write detection. Prefer child-process environments to global mutation and distinguish intentional build outputs from leaks. Reuse `sct`'s isolated-data-home and `dsc`'s command-tree policy patterns; record concrete adoption evidence before making any new universal requirement.

- [ ] **R2 - Reproducible artefact identity distinct from build events.** Document when a data-producing CLI needs separate source, schema/transformation and tool identities, and how timestamps/local paths can remain auditable without becoming content identity. Use `ods`'s independent rebuilds and tool/dataset versions, `sct`'s canonical/companion fingerprints, and `anarchie`'s canonical JSON/content-addressed knowledge packages as worked examples. Specify what each hash proves and how encoding-dependency changes affect promised bytes. Recommend cross-platform checks only for a precisely defined reproducibility claim, not blanket byte-identical databases or mandatory OCI distribution.

- [ ] **R3 - Evidence-backed citations and explanations as optional CLI patterns.** Capture source/derived-artefact/software citation separation, recorded-versus-verified evidence, honest unknowns, path redaction, offline export and upstream attribution in [rust-cli.md](rust-cli.md) and [licensing.md](licensing.md) once the `sct cite` implementation provides a worked example. Pair explanatory surfaces with equivalence checks: `ods find --sql`, shared-engine calculator workings, query plans and mock-verified mutation plans. Keep these domain-driven patterns rather than requiring every CLI to add `cite` or `--explain`.
