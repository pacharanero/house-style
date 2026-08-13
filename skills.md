# Skills

Reference: Matt Pocock's `writing-great-skills` skill (<https://github.com/mattpocock/skills/tree/main/skills/productivity/writing-great-skills>) and the documentation for the target agent platform.

Skills are small, load-bearing instruction packages for agents. A good skill makes agent behaviour more predictable: not identical output every run, but the same process, checks, and stopping conditions when faced with the same kind of task.

## What belongs in a skill

Create a skill when the agent needs procedural knowledge that is not obvious from general coding ability:

- A repeated workflow with sharp sequencing.
- Domain rules or vocabulary the model will not reliably infer.
- Tool integration details.
- Validation steps that are easy to forget.
- Reusable scripts, references, or assets that stop agents rewriting the same thing badly.

Do not create a skill for generic advice, a one-off task, or instructions that belong in `AGENTS.md`, `CONTEXT.md`, `spec/`, or this house-style repo.

## Shape

Use the standard skill layout:

```text
skill-name/
|-- SKILL.md
|-- agents/
|   `-- openai.yaml
|-- references/
|-- scripts/
`-- assets/
```

Only `SKILL.md` is mandatory. Add the other directories only when they earn their keep.

`SKILL.md` contains YAML frontmatter and a short Markdown body. The frontmatter must include:

- `name` - lowercase, hyphen-separated, matching the folder name.
- `description` - for model-invoked skills, a trigger description; for user-invoked skills, a human-facing summary if the platform still requires it.

Prefer names that are short, verb-led, and concrete: `gh-address-comments`, `fix-ci`, `write-safety-case`. Avoid vague nouns like `workflow-helper`.

## Invocation

Decide deliberately whether the skill should be model-invoked or user-invoked.

Use a **model-invoked** skill when the agent should discover and apply it autonomously, or when another skill should be able to reach it. The description is always in context, so it must earn that permanent context load.

Use a **user-invoked** skill when the human will explicitly ask for it and autonomous triggering would add noise. This spends human memory instead of context. If user-invoked skills multiply, add one router skill that names them and when to use each.

For high-stakes domains such as clinical safety, regulatory, legal, security-sensitive, or assurance workflows, prefer **user-invoked** by default unless autonomous invocation is clearly necessary. The human should stay in control of when the workflow runs; a router skill is safer than broad trigger descriptions that may misfire.

For a model-invoked description:

- State what the skill does.
- Name the distinct branches that should trigger it.
- Use the leading words people actually type in prompts, docs, and code.
- Cut synonyms that describe the same branch twice.
- Do not put body content, setup notes, or background exposition in the description.

The description is the most expensive part of a model-invoked skill because it is always loaded. Prune it harder than the body.

## Body

The body of `SKILL.md` is for the material the agent needs immediately after the skill fires.

Write in this order:

1. Purpose and trigger boundary.
2. Required read-first material.
3. Steps, if the skill is procedural.
4. Completion criteria for each step or for the skill as a whole.
5. Pointers to references, scripts, or assets.
6. Failure modes and how to recover, only where they are specific to this skill.

Every step should end with a checkable completion criterion. "Understand the repo" is weak; "identify the command that fails and the first failing assertion" is checkable. Strong completion criteria are the main defence against premature completion.

Keep `SKILL.md` short. If it approaches 500 lines, it is almost certainly carrying reference material that should be disclosed behind pointers or split by branch.

## Progressive disclosure

Use progressive disclosure to protect the top of the skill:

- Put common, always-needed steps in `SKILL.md`.
- Put branch-specific details in `references/<branch>.md`.
- Put deterministic operations in `scripts/`.
- Put output resources in `assets/`.

A pointer must say when to read the file, not just that the file exists:

```markdown
- For OAuth provider setup, read `references/oauth.md`.
- For SAML provider setup, read `references/saml.md`.
```

Keep references one level from `SKILL.md`. Avoid chains where `SKILL.md` points to a file that points to another file that contains the actual rule.

## Steps vs reference

Separate ordered work from reference facts.

Use **steps** when order matters or when a later step can distract the agent from finishing the current one. Use **reference** for rules, definitions, examples, or checklists that can be applied in any order.

If a long sequence causes the agent to rush early steps, split the sequence at a real boundary. Hiding later work can improve legwork on the current step.

If a skill is mostly a checklist, it can be all reference. In that case, give the checklist an explicit demand: "Apply every rule below to the changed files" rather than "consider these rules".

## Scripts, references, and assets

Use `scripts/` for deterministic repeatable work: validators, converters, generators, API wrappers, or fragile command sequences. Prefer scripts when the same code is being rewritten repeatedly.

Use `references/` for detailed documentation the agent may need but should not always load: schemas, API docs, domain rules, examples, policy text, or framework variants.

Use `assets/` for files the skill uses as output material: templates, images, fonts, starter projects, sample documents.

Do not add README files, changelogs, installation guides, or meta-documentation inside a skill unless the skill explicitly needs the agent to read them. Skills are not mini repos; they are instruction packages.

## Leading words

A leading word is a compact concept the model already knows and can use as a behavioural anchor: "triage", "red-green-refactor", "smoke test", "router", "golden", "hazard log".

Use leading words to collapse repeated explanation into one stable concept. Prefer existing words over coined terms. A coined term costs definition tokens before it can steer behaviour.

A good leading word appears in the description, the body, and the user's natural prompt vocabulary. That shared language improves invocation and execution.

## Pruning

Skills rot by accumulation. Review them with these tests:

- **Relevance**: does this line still affect what the skill does?
- **No-op**: would the agent already do this without being told?
- **Duplication**: does this meaning live somewhere else too?
- **Sprawl**: is this live material, but too much for the top-level body?
- **Sediment**: is this leftover process history rather than current instruction?

Delete no-ops rather than polishing them. Move sprawl into references. Collapse duplication into one source of truth.

## Validation

Validate a skill against realistic prompts before treating it as done:

- Does the description trigger when it should?
- Does it stay quiet when it should?
- Does the agent read the right references?
- Does it perform enough legwork before claiming completion?
- Are scripts executable and documented by their invocation?
- Can another agent follow the skill without the conversation that produced it?

For complex skills, run a forward test with a fresh agent or subagent and inspect the resulting artefacts, diffs, or logs. Do not leak the intended answer into the validation prompt unless that is the thing being tested.

## House style checklist

Before committing a skill:

- Folder name is lowercase hyphen-case and matches `name`.
- Description is trigger-focused and concise.
- `SKILL.md` contains only immediate instructions and pointers.
- Branch-specific material is in `references/`.
- Deterministic repeated work is in `scripts/`.
- Assets are in `assets/`, not described as prose.
- Steps have checkable completion criteria.
- There is one source of truth for each rule.
- Obvious no-ops and sediment are removed.
- Validation has been run or the reason for skipping it is explicit.
