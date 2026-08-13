# Examples

Copy these examples into a repo when there is no closer local pattern. They are deliberately complete enough for agents to reuse rather than inventing a new wrapper from scratch.

- [`s/docs-zensical`](s/docs-zensical) - serve a Zensical docs site from a project venv, preflight common Linux inotify failures, pick the first free docs port, and pass caller arguments through.
- [`s/docs-docker-compose`](s/docs-docker-compose) - start a containerised docs stack, wait for it to become reachable, and open the browser.
- [`s/install-hooks`](s/install-hooks) - opt in to tracked `.githooks/` by setting `core.hooksPath`.
- [`s/version++`](s/version++) - branch-protection-ready Rust release bump script for the CI auto-tag cascade.
- [`rust-cli/`](rust-cli/) - minimal `main.rs` / `lib.rs` split for a clap-derived Rust CLI.
- [`architecture/`](architecture/) - optional lightweight ADR, risk-register, and traceability templates, for repos that genuinely need governance artefacts.
- [`new-repo-template/`](new-repo-template/) - optional contribution, security-reporting, code-of-conduct, and GitHub issue/PR templates for collaborative or public repositories.
