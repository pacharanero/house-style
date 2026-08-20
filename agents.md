# Agent Instructions

Every repo should have a clear entry point for AI coding agents. The purpose is not to restate the whole README; it is to tell an agent what to read first, what invariants must not be broken, and how to prove a change is ready.

## File layout

Use a vendor-neutral `agent-instructions.md` as the source of truth for new repos. Keep tool-specific files thin:

- `AGENTS.md` points to `agent-instructions.md` for tools that follow the AGENTS convention.
- `CLAUDE.md` points to the same file for Claude Code.
- Any other tool-specific instruction file should also point at the same source, not fork the guidance.

For mature repos where `AGENTS.md` is already the living source of truth, it can stay substantive. Do not create a second competing file unless you are prepared to move the content and leave the old files as pointers.

## Required content

Every agent instruction file should include:

- **Project purpose**: one short paragraph saying what the repo is and what it is not.
- **Read first**: links to the README, roadmap/spec docs, domain context, and the adopted house-style standard.
- **Core invariants**: the small number of rules that must not be violated. Examples: a core crate stays a leaf; clinical calculations cite primary literature; generated files are not hand-edited.
- **Local workflow**: the canonical `s/` scripts and validation commands.
- **Before every commit**: the exact checks CI enforces.
- **Git workflow**: whether agents commit and push directly to `main` or must use a branch and PR. In the absence of a local exception, follow [commits.md](commits.md): commit and push each validated coherent parcel.
- **External actions**: what needs explicit approval, such as publishing releases, deleting branches, force-pushing, posting issues, or running scans against third-party systems.
- **Protected paths and environments**: code owners, release or deployment paths, and the branch-protection tier where these matter; see [ci.md](ci.md).

Keep the file short enough that an agent actually reads it. Put detailed domain design in `CONTEXT.md`, `spec/`, or docs pages and link to it.

For reusable agent behaviours packaged as skills, use [skills.md](skills.md). Agent instruction files tell an agent how to work in one repo; skills teach a reusable workflow or domain procedure that should travel between repos.

## Agent assurance

Treat an agent as a capable junior developer: give it a clear, well-scoped task and review its output. An agent can propose and implement a change, but it is not evidence that the change is correct.

- Read the diff and run the relevant validation after agent work. Do not accept a passing test suite without checking what the tests actually prove.
- For clinical logic, safety-critical workflows, authentication, cryptography, authorisation, input validation, or code handling real patient data, validate the behaviour independently against an authoritative source, threat model, or reviewed clinical evidence.
- Agent-generated tests must not be the sole basis for accepting high-consequence behaviour. Review their assumptions and include source-derived vectors or other independent evidence where applicable.
- Record safety-relevant changes, hazards, evidence, and review decisions in the same PR and documentation trail as the implementation.

## Agent Git handoff

After completing a validated coherent parcel, agents should inspect the diff, commit it, and push it using the repository's branching workflow. The remote repository is shared state for scheduled agents and collaborators, so leaving completed work only in a local worktree risks duplicated or conflicting changes.

For a new or fast-moving repository with one or two active contributors, direct commits to `main` or `live` are the default unless local instructions say otherwise. For a mature repository with several contributors, a protected default branch, or a PR convention, agents use a descriptive feature branch and push it for PR review. Ask when the repository state or intended workflow is unclear.

## Agent artefacts

Scratch output from agent tooling is session state, not project content. Gitignore it in every repo, not only new ones.

- Covers browser-automation and MCP server output (`.playwright-mcp/` and similar tool-output directories), agent config directories (`.claude/`), and any transcript, screenshot, or snapshot a tool drops into the working tree.
- Committing them leaks local paths and conversation context, and they trip repo-wide file checks - REUSE/licence linting in particular, which scans the working tree rather than the git index, so an untracked artefact directory is enough to fail `reuse lint` locally.
- Keep them in one commented block in `.gitignore`, so the next tool's directory has an obvious home.

Intentionally retained local work is different from disposable agent output. Use the root gitignored [`.private/` convention](private-workspace.md) for decks, reports, captures, notes, and other useful artefacts that should survive the session but must not be committed. Agents should suggest this when the distinction would help rather than scattering durable local output among tool scratch directories.

After substantial work, consider whether a short optional [Reveal.js work digest](presentations.md#optional-work-digests) would materially reduce the user's context-recovery cost. This is especially useful when the user is working across many repositories or the change is best understood through a command demo, data flow, safety boundary, and evidence summary.

## Read-first order

Use this order unless the repo has a strong reason to differ:

1. `README.md` - project purpose and setup.
2. `AGENTS.md` or `agent-instructions.md` - agent-specific guardrails.
3. `CONTEXT.md` - domain language, if present.
4. `spec/README.md` or `ROADMAP.md` - design intent and current work.
5. The adopted house-style `AGENTS.md` - cross-repo standards.

The local repo wins over generic house style when it has a conscious, documented exception. If the repo only diverges because it has not caught up, treat that as a migration gap.

## Good agent instructions

Good instructions are operational:

- They name the files and scripts an agent should open before changing code.
- They say what not to touch casually.
- They give validation commands in copy-pasteable form.
- They avoid stale dependency versions, action SHAs, and large duplicated snippets.
- They point to living examples instead of freezing long code blocks in prose.

Bad instructions are vague personality rules, duplicated README content, or tool-specific folklore that drifts between `AGENTS.md`, `CLAUDE.md`, and `.github/copilot-instructions.md`.

## Template

````markdown
# Agent Instructions

<one paragraph project purpose>

This file is the entry point for AI coding agents. Read it before changing anything.

## Read First

- [README.md](README.md) - setup and project overview.
- [ROADMAP.md](ROADMAP.md) - planned work.
- [spec/README.md](spec/README.md) - design/spec index, if present.
- [CONTEXT.md](CONTEXT.md) - domain language, if present.
- [pacharanero/house-style](https://github.com/pacharanero/house-style) - cross-repo standards, when adopted.

## Core Invariants

- ...

## Workflow

- `s/test` - run tests.
- `s/lint` - run formatting and lint checks.
- `s/docs` - serve docs locally, if present.

## Before Every Commit

```sh
<validation commands>
```

## Git Workflow

- Commit and push each validated coherent parcel; do not leave completed work only locally.
- Use direct `main` / `live` commits for fast-moving work with one or two contributors when permitted.
- Use a descriptive branch and PR for mature collaborative or protected repositories.
- Ask if the appropriate path is unclear.

## Assurance

- Review the diff and validation results after agent changes.
- For <high-consequence areas>, validate behaviour against <authoritative source or evidence>.

## Approval Required

Ask before publishing releases, deleting branches, force-pushing, changing secrets, bypassing branch protection or deployment approvals, or taking externally visible actions.
````
