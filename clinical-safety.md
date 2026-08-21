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

For DCB0129-style clinical safety management files, use the [`pacharanero/SAFETY.md`](https://github.com/pacharanero/SAFETY.md) pattern: Markdown source, version-controlled safety artefacts, and optional documentation-site or PDF output for organisations that still require a single offline document.

The template is a starting point, not a completed safety case. Generated clinical safety text must be reviewed and rewritten for the actual product, team, deployment context, and hazards.

## Medical-device applicability assessment

At project inception, record an assessment of whether the product may fall within medical-device regulation. Keep it in `SAFETY.md` for a small project or in a linked `MEDICAL-DEVICE-ASSESSMENT.md` for a larger safety file set. This is a governance record, not regulatory advice or a self-classification.

The record must state:

- The product's intended purpose, including intended users, patients or populations, use setting, and clinical claims.
- The decision-maker's name and responsible role.
- Decision date and review date.
- Evidence considered, including dated links to applicable MHRA guidance, product requirements, user-facing claims, and professional or legal advice where obtained.
- The conclusion, including whether the team considers medical-device or IVD obligations potentially applicable, not applicable, or requiring further advice.
- Consequent actions and owner, such as obtaining specialist advice, preserving technical documentation, checking market and registration obligations, or recording why no further action is currently needed.
- Reassessment triggers.

Reassess the record when intended purpose, clinical claims, target users or population, clinical decision support, inputs or outputs, deployment context, integration with a medical device, or market changes. Update it before releasing a change that could alter the conclusion, rather than retrospectively at audit or registration time.

Medical-device applicability and clinical risk management answer different questions. A conclusion that the product is not a medical device does not remove the need to identify and manage clinical hazards. Conversely, a device conclusion does not satisfy the DCB0129 or DCB0160 clinical safety process. Where either is applicable, link the assessment, safety case, hazard log, and resulting obligations from `SAFETY.md`.

The UK framework changes frequently. Check current primary guidance for every assessment, including MHRA guidance on [borderline products](https://www.gov.uk/guidance/borderline-products-how-to-tell-if-your-product-is-a-medical-device) and [regulating medical devices in the UK](https://www.gov.uk/guidance/regulating-medical-devices-in-the-uk), and record the access date. For NHS deployment, also review current NHS England guidance on [DCB0129 and DCB0160 applicability](https://digital.nhs.uk/services/clinical-safety/applicability-of-dcb-0129-and-dcb-0160/step-by-step-guidance).

### Minimal record

```markdown
## Medical-device applicability

- Intended purpose:
- Decision-maker and role:
- Decision date:
- Next review date:
- Evidence considered:
- Conclusion:
- Actions and owner:
- Reassessment triggers:
```

## Safety and implementation stay linked

When a code change affects clinical behaviour, update the safety docs in the same PR or explicitly explain why no safety doc change is needed. Tests for safety-relevant behaviour should cite the safety hazard or clinical source they protect where practical.

Do not let safety docs become a release-time paperwork exercise. They are part of the product.
