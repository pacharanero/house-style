# Security

Security work should be explicit, reproducible, and scoped. Do not rely on ad-hoc terminal history for checks that may need to be repeated or reviewed.

## Scope and authorisation

Security scripts and playbooks must state their authorised scope. For personal tooling this usually means assets owned or operated by the maintainer or organisation.

Never run recon, scanning, exploitation, or credential testing against third-party systems unless written authorisation is already clear. If a script can touch a network target, its README or header should say what targets are allowed.

## Secrets hygiene

- Never commit `.env`, credentials, tokens, private keys, or generated session memory.
- If a secret is committed, treat it as compromised and rotate it.
- Do not print secrets in logs or CLI output.
- Redact credentials embedded in URLs.
- Prefer environment variables or local ignored files over checked-in config.

Public repos should have a `SECURITY.md` vulnerability reporting policy. Private repos should still document where sensitive findings belong.

## Reproducible security workflows

Recurring security work belongs in scripts or playbooks:

- `s/scan-code` for static checks.
- `s/ids-check` for intrusion-detection checks.
- `s/recon` for authorised asset discovery.
- `s/report` for turning findings into a shareable report.

Names can vary by repo, but the principle does not: make the workflow repeatable, reviewable, and diffable.

## Dependency and supply-chain checks

Use [dependencies.md](dependencies.md) before admitting a dependency and [ci.md](ci.md) for automation: pinned GitHub Actions, Dependabot cooldown/grouping, `cargo audit` for Rust, and package-manager equivalents where appropriate.

For generated lockfile changes, review what changed. A routine grouped Dependabot PR is still code entering the project.

## Findings handling

Treat security findings as sensitive until triaged. Do not paste exploit details or credentials into public issues. Use private notes, private repos, or responsible disclosure channels as appropriate.

When a finding is fixed, record enough evidence to make the fix auditable: commit, PR, test, dependency bump, config change, or release note.

## Security and clinical safety

For healthcare software, security and clinical safety overlap. A security weakness that can affect care, patient data, or clinical workflow may also be a safety hazard. Link it from the safety case or hazard log where appropriate.
