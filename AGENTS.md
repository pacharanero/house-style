# House style - engineering standards

The canonical standards for how Marcus Baw builds software. They are opinionated defaults, published for reuse by projects and coding agents.

## How to use this

When working in a project that adopts these standards, **read the relevant standard before doing that kind of work**. These files are the source of truth unless the project documents a conscious exception.

- They describe the *target* convention. If a repo has not adopted it yet, that is a gap to close, not a reason to do something different.
- If a repo's existing local style genuinely diverges, match the repo and flag the divergence rather than silently mixing styles.
- Prefer current official documentation over stale copied snippets. Version-specific examples, especially GitHub Actions, must be revalidated before use.

## Defaults, evidence, and exceptions

These standards are strong defaults, not laws. They exist to remove repeated decisions, not to replace judgement or evidence.

- **A documented exception beats silent divergence, and beats blind compliance.** Distinguish "has not adopted this yet" (a migration gap to close) from "deliberately different" (a decision to record). Write the decision where the next reader will look - the repo's agent instructions, an ADR, or a spec note - and give the reason, not just the deviation.
- **When options are otherwise equivalent, prefer the maintained one and the reversible one.** A choice that can be undone cheaply is worth more than a marginally better choice that locks the project in. Check that a dependency, tool, or platform is still actively supported before betting on it (see [dependencies.md](dependencies.md)).
- **Settle disputed assumptions with evidence, not argument.** A prototype, a measurement, a test, or real user feedback resolves a disagreement faster and more honestly than debate. Prefer the cheap experiment; write down what it showed.
- **Experiment freely, but not on real patients, real patient data, or production credentials.** Prototypes stay isolated until they pass the gates in [clinical-safety.md](clinical-safety.md) and [security.md](security.md). Speed is a legitimate goal right up to the point where it can harm someone.

Canonical reusable skills live under [skills/](skills/). Slash-command wrappers live under [commands/](commands/) and should point back to the canonical skill rather than duplicating the workflow.

## Standards

- [agents.md](agents.md) - agent instruction files, read-first order, invariants, validation commands.
- [ci.md](ci.md) - CI workflows, what to enforce, GitHub Actions SHA-pinning.
- [clinical-safety.md](clinical-safety.md) - root `SAFETY.md`, safety case, hazard logs, DCB0129-style docs.
- [commits.md](commits.md) - conventional commits, git-cliff changelog, release commit shape.
- [dependencies.md](dependencies.md) - dependency admission, versioning, lockfiles, and lifecycle management.
- [distribution.md](distribution.md) - releases, cargo-dist, Homebrew tap, crates.io, versioning.
- [docs.md](docs.md) - documentation sites with Zensical, GitHub Pages deploy.
- [library-extraction.md](library-extraction.md) - reusable engine crates: leaf discipline, the path/git/crates.io dependency progression, extracting to a repo with history.
- [licensing.md](licensing.md) - AGPL, SPDX headers, REUSE, content licensing.
- [mcp.md](mcp.md) - MCP protocol and SDK policy, including the conditional `rmcp` adoption gate and transport bounds.
- [new-repos.md](new-repos.md) - new repo scaffold, README minimums, default files, setup runbook.
- [presentations.md](presentations.md) - Reveal.js CDN slide decks, presentation style, local serving.
- [private-workspace.md](private-workspace.md) - the gitignored `.private/` convention for retained local-only artefacts.
- [repo-presentation.md](repo-presentation.md) - README structure, quick start, badges, visuals, and repo metadata.
- [rust-cli.md](rust-cli.md) - CLI shape, data-on-stdout/hints-on-stderr, auto-hiding progress bars, path arguments that expand `~` themselves, deterministic configuration discovery, the declarative pull/edit/push/diff loop, `--dry-run` + destructive-action guards, machine-discoverable `--schema`/template surface, registry-driven dispatch.
- [scripts.md](scripts.md) - the `s/` directory of repeated-process scripts.
- [security.md](security.md) - scope/authorisation, secrets hygiene, reproducible security workflows.
- [skills.md](skills.md) - writing predictable, lean, progressively disclosed agent skills.
- [specs.md](specs.md) - spec-led development, ubiquitous language, conformance suites.
- [tauri-gui.md](tauri-gui.md) - Tauri 2 + React + Mantine + Vite desktop GUI shape.
- [testing.md](testing.md) - registry/policy enforcement tests, golden/literature-vector tests, filesystem/git test isolation.
- [ui.md](ui.md) - domain-specific interface design, controls, copy, accessibility, and visual verification.

Deferred: a fuller "one core, many surfaces" engine-surface standard. [library-extraction.md](library-extraction.md) keeps the current leaf-crate guidance for now.

## Cross-cutting essentials

The things that apply to (nearly) every project, in brief. The files above expand on them.

- **Language & shape**: Rust by default. CLIs use `clap` *derive* (never the builder), a thin `main.rs` delegating to a reusable `lib.rs`, `anyhow::Result` throughout, and an output-format enum honoured via `--format` (text / json[ / yaml]). Reset SIGPIPE on Unix so output pipes cleanly into `head`/`less`. Each command family is its own module with a `run()` function. Configuration loads are strict: reject malformed, invalid, and unrecognised fields rather than silently defaulting. See [rust-cli.md](rust-cli.md).
- **Agents**: new repos prefer a vendor-neutral `agent-instructions.md` with thin `AGENTS.md` / `CLAUDE.md` pointers. Mature repos can keep substantive `AGENTS.md`, but must include read-first docs, invariants, validation commands, and the local Git workflow. Agents commit and push each validated coherent parcel by default: direct to `main` / `live` for permitted fast-moving work with one or two contributors; a descriptive branch and PR for mature collaborative or protected repositories. Ask if the appropriate path is unclear. See [agents.md](agents.md) and [commits.md](commits.md).
- **Agent artefacts**: gitignore disposable agent and browser-automation output in every repository. This includes `.playwright-mcp/`, `.playwright/`, `playwright-report/`, `blob-report/`, and `test-results/`; do not ignore browser-test source or `playwright.config.*`. See [agents.md](agents.md).
- **Before every commit**: `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. CI enforces all three; do not commit red.
- **Commits & versions**: conventional commits (`feat(area):`, `fix:`, `docs:`, `chore(deps):`, `ci:`). Version single-sourced in `Cargo.toml`. Releasing is **one action**: `s/version++ [patch|minor|major]` bumps the version, regenerates `CHANGELOG.md` (git-cliff), commits `chore(release): vX.Y.Z`, and lands that release commit on `main`. The target model is the CI auto-tag cascade: after the bump lands, `auto-tag.yml` creates the `vX.Y.Z` tag and invokes the release workflow. There is never a second `s/release`. See [distribution.md](distribution.md) and [commits.md](commits.md).
- **Scripts**: a top-level `s/` directory of tiny bash wrappers - `s/test`, `s/build`, `s/docs`, `s/version++`. Not `scripts/`. Each script is one repeated process, `exec`'s the underlying tool, and forwards `"$@"`. Ship `s/README.md` once there are more than three. See [scripts.md](scripts.md).
- **Licensing and affiliation**: written content is CC-BY-SA-4.0; code uses the project-selected licence. Each project decides its copyright holder and affiliation. See [licensing.md](licensing.md).
- **GitHub Actions**: pin every action to a full commit SHA with a `# vX.Y.Z` comment. See [ci.md](ci.md).
- **Dependencies**: add only reviewed direct dependencies, commit application lockfiles, and treat routine and security updates as code review work. See [dependencies.md](dependencies.md).
- **MCP**: Rust MCP clients and servers converge on the official `rmcp` SDK once a stable release includes a bounded stdio reader; no migration may regress finite input limits. See [mcp.md](mcp.md).
- **Clinical safety**: clinical tools should grow from root `SAFETY.md` to a safety file set as risk and governance needs grow. Keep safety docs in Git and link hazards to evidence. See [clinical-safety.md](clinical-safety.md).
- **Specs and language**: use `spec/` for durable design decisions, `ubiquitous-language.md` where terminology matters, and conformance suites for behaviour shared across implementations. See [specs.md](specs.md).
- **Skills**: write skills as lean instruction packages with precise invocation, progressive disclosure, checkable completion criteria, and aggressive pruning. See [skills.md](skills.md).
- **Presentations**: use Reveal.js from a CDN by default, keep slides image-heavy and low-text, and use vertical Reveal stacks for optional depth. For optional work digests, prefer Oswald headings, Reveal's Solarized theme, and a short embedded Asciinema demo where motion adds value. See [presentations.md](presentations.md).
- **Private workspace**: use a root gitignored `.private/` for intentionally retained local-only decks, reports, captures, and notes. Agents should suggest it when useful local output should survive the session but must not be committed. See [private-workspace.md](private-workspace.md).
- **Repo presentation**: lead with a one-liner, hero visual, and a prominent quick start. Use only informative badges. Keep the README honest, maintained, and linked to the docs site. See [repo-presentation.md](repo-presentation.md).
- **User interfaces**: design from the domain and frequent workflow, use concrete copy and familiar controls, avoid generic SaaS/card-grid defaults, meet WCAG 2.2 AA, and inspect rendered mobile/tablet/desktop states before considering frontend work complete. See [ui.md](ui.md).
- **Prose & files**: Markdown is not hard-wrapped (one long line per paragraph, soft wrap). Use hyphens, not emdashes. Filenames are slug-case-with-hyphens, except recognised conventions (`README.md`, `AGENTS.md`, `Cargo.toml`).

Projects may install or link these standards from their own agent instructions; installation is optional and must not depend on a particular workstation layout.
