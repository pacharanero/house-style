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

## Identity and access

Use MFA for every source-control, cloud, registry, and production-service account. Prefer phishing-resistant passkeys or hardware security keys where the service supports them. Store unique passwords in an approved password manager; never share accounts or credentials.

Protect SSH private keys with a passphrase and keep them in the operating system's secure key store or a hardware-backed agent where practical. Use a separate deploy key or workload identity for automation rather than a maintainer's personal SSH key. Review SSH keys, deploy keys, personal access tokens, OAuth apps, GitHub Apps, and cloud service principals regularly.

Grant the smallest role that lets a person or automation do its job. Avoid standing administrator access to repositories, registries, clouds, and production systems. Make access removal part of offboarding and role changes: revoke service access, tokens, SSH and deploy keys, app installations, recovery factors, and environment approvals promptly; rotate any shared secret. After suspected compromise, revoke active credentials, rotate affected secrets, review audit logs and recent deployments, and record the incident and remediation.

For source-control and deployment requirements, use the tiered `main` and environment baseline in [ci.md](ci.md). Signed commits may provide useful tamper evidence, but they do not establish trusted human identity, independent review, or correctness.

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
