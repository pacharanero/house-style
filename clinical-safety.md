# Clinical Safety

Clinical safety documentation belongs beside the product and evolves with it. Prefer Markdown in Git over detached Word documents and spreadsheets, so safety decisions, hazards, mitigations, and code changes share one audit trail.

This standard does not replace the need for a Clinical Safety Officer or a proper clinical safety process. It sets the repo shape that makes that process easier to maintain.

## Tier 1: root `SAFETY.md`

Every clinical or health-informatics repo that can affect care should have a root `SAFETY.md` once it moves beyond a toy. For low-risk tools this can be the whole safety file.

It should cover:

- What the product does and does not do.
- Intended users and use environment.
- Clinical safety owner or responsible role.
- Current safety status.
- Known hazards or a link to the hazard log.
- Links to safety case, safety plan, and external clinical governance where present.

## Tier 2: safety file set

As the safety argument grows, add a directory such as `clinical-safety/`, `safety/`, or `docs/safety/`. Keep the root `SAFETY.md` as the entry point and link from it.

Recommended files:

- `SAFETY.md` - overview and navigation.
- `SAFETY-CASE.md` - the clinical safety argument and evidence.
- `SAFETY-PLAN.md` - process, responsibilities, review cadence, and release safety gates.
- `HAZARD-LOG.md` - hazard list for small projects.

For larger projects, split hazards into one file per hazard under `hazards/` with required frontmatter, and generate a summary table from them.

## Hazard logs

Hazards need stable IDs and reviewable lifecycle state. At minimum record:

- ID and title.
- Hazard description.
- Cause.
- Clinical consequence.
- Initial severity/likelihood/risk.
- Mitigations.
- Residual severity/likelihood/risk.
- Owner.
- Status.
- Evidence links to commits, PRs, tests, issues, or docs.

Use Git history and PR discussion as part of the evidence trail. Do not hide the real safety argument in a binary spreadsheet if Markdown can carry it.

## DCB0129 / DCB0160 documentation

For DCB0129-style clinical safety management files, use the `turva/dcb0129-template` pattern: Markdown source, Zensical site, GitHub Pages deploy, and optional PDF output for organisations that still require a single offline document.

The template is a starting point, not a completed safety case. Generated clinical safety text must be reviewed and rewritten for the actual product, team, deployment context, and hazards.

## Safety and implementation stay linked

When a code change affects clinical behaviour, update the safety docs in the same PR or explicitly explain why no safety doc change is needed. Tests for safety-relevant behaviour should cite the safety hazard or clinical source they protect where practical.

Do not let safety docs become a release-time paperwork exercise. They are part of the product.
