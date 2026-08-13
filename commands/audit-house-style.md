# Audit House Style

Use the canonical `audit-house-style` skill from this repository to audit the target repo path supplied by the user.

Expected invocation:

```text
/audit-house-style <target-repo-path>
```

Follow `skills/audit-house-style/SKILL.md` exactly:

- Read the adopted [house-style index](https://github.com/pacharanero/house-style/blob/main/AGENTS.md) and relevant standards.
- Inspect the target repo.
- Write `HOUSE-STYLE-AUDIT.md` in the target repo.
- Do not fix the repo during the audit.

If no target repo path is supplied, ask for one.
