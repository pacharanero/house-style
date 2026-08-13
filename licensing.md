# Licensing

## Company affiliation and copyright holder

Choose the copyright holder and any company affiliation from the project's domain and use it consistently in SPDX headers, REUSE annotations, package metadata, notices, documentation, and public project branding:

- A project may be personal, affiliated with an organisation such as Koloki Ltd or Baw Medical Ltd, or have another deliberate arrangement.
- For mixed-domain or unclear work, confirm the intended holder rather than guessing. Existing project ownership and licensing history takes precedence; never silently relicense a repository while changing its affiliation.

## Choose a licence deliberately

This repository's prose is licensed under CC-BY-SA-4.0. Source code needs a software licence chosen for that project, not an assumed house default. The project owner should record the choice in a root licence file before accepting contributions or publishing code.

## SPDX header on every source file

Every source file carries a two-line SPDX header at the very top:

```
// SPDX-FileCopyrightText: <year> <copyright holder>
// SPDX-License-Identifier: <chosen SPDX identifier>
```

Use the comment syntax of the language (`//` for Rust, `#` for shell/TOML/YAML/Python). The header sits above everything, including module doc comments. When adding it to an existing tree, a one-shot script that prepends it where `SPDX-License-Identifier` is absent is the way (idempotent, skips files that already have one).

## REUSE for files that can't carry a header

Files that cannot take an inline header (Markdown, JSON, lockfiles, generated assets, test fixtures) can be covered in bulk by a root `REUSE.toml`: a `version = 1` file with `[[annotations]]` blocks giving `path` globs, `SPDX-FileCopyrightText`, and `SPDX-License-Identifier`. Run `reuse lint` locally; enforce it in CI with `fsfe/reuse-action` (see [ci.md](ci.md)).

## Content: CC-BY-SA-4.0

Written and clinical content - documentation, source references, the input-definition registry - is **CC-BY-SA-4.0**. Note this in `mkdocs.yml` copyright and the README licensing section.

## Third-party provenance

When an algorithm or dataset comes from elsewhere, record it and honour its licence:

- Carry the upstream licence and any required disclaimer (e.g. QRISK3 / QFracture are ported from ClinRisk's LGPL-3.0 source and must carry the ClinRisk disclaimer).
- Third-party content keeps its own copyright (e.g. SNOMED CT terms remain SNOMED International's, subject to its licence) - document it in-file and in the README.
- Where each item can record its own distribution licence, as in [`pacharanero/clincalc`](https://github.com/pacharanero/clincalc), make that a required, tested field so nothing ships without its provenance on record.

## README licensing section

State, briefly: code licence, content licence, and any notable third-party provenance.
