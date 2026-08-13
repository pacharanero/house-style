# house-style

Engineering standards for how [Marcus Baw](https://github.com/pacharanero) builds software. They are published for projects and coding agents that benefit from explicit, evidence-led defaults.

Start with **[AGENTS.md](AGENTS.md)** (the index and cross-cutting essentials). Copy or link only the standards relevant to a project; local documented decisions take precedence.

- [ci.md](ci.md) - CI workflows and GitHub Actions pinning
- [clinical-safety.md](clinical-safety.md) - clinical safety file sets, hazard logs, DCB0129-style docs
- [commits.md](commits.md) - conventional commits and changelog generation
- [dependencies.md](dependencies.md) - dependency admission, versioning, lockfiles, and lifecycle management
- [distribution.md](distribution.md) - releases, cargo-dist, Homebrew, crates.io
- [docs.md](docs.md) - Zensical documentation sites
- [agents.md](agents.md) - agent instruction files and read-first guidance
- [licensing.md](licensing.md) - SPDX headers, REUSE, and content licensing
- [new-repos.md](new-repos.md) - new repo scaffolding and setup runbook
- [presentations.md](presentations.md) - Reveal.js CDN slide decks, presentation style, local serving
- [private-workspace.md](private-workspace.md) - gitignored local-only decks, reports, captures, and notes
- [repo-presentation.md](repo-presentation.md) - README structure, quick start, badges, visuals, and repo metadata
- [security.md](security.md) - scope, secrets hygiene, and reproducible security workflows
- [scripts.md](scripts.md) - the `s/` directory of repeated-process scripts
- [skills.md](skills.md) - writing predictable, lean, progressively disclosed agent skills
- [specs.md](specs.md) - specs, ubiquitous language, and conformance suites
- [tauri-gui.md](tauri-gui.md) - Tauri desktop GUI shape
- [testing.md](testing.md) - policy tests, goldens, and test isolation
- [ui.md](ui.md) - interface design, accessibility, copy, responsive states, and visual verification

Supporting files:

- [examples/](examples/) - copyable script, Rust CLI, and governance examples for agents and humans
- [skills/](skills/) - canonical reusable agent skills that can be copied into an agent platform's documented skill location
- [commands/](commands/) - slash-command wrappers that point to canonical skills or workflows

## Licence

The standards and prose are licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). Copyable Rust and shell examples are [MIT licensed](LICENSES/MIT.txt).

## Contributing And Security

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before proposing a change. Report vulnerabilities privately as described in [SECURITY.md](SECURITY.md).
