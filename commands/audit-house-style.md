# Audit House Style

Use the canonical `audit-house-style` skill from this repository to audit the target repo path supplied by the user.

Expected invocation:

```text
/audit-house-style <target-repo-path>
```

Follow `skills/audit-house-style/SKILL.md` exactly:

- Read the adopted [house-style index](https://github.com/pacharanero/house-style/blob/main/AGENTS.md) and relevant standards.
- Inspect the target repo.
- Add actionable findings as stable `HS-*` checkbox items in the target repo's active roadmap, under its own House-Style Audit section.
- Ask where findings should go if no roadmap exists or several roadmaps have ambiguous ownership.
- Do not create a standalone audit report.
- Do not fix the repo during the audit.

If no target repo path is supplied, ask for one.
