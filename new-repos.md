# New Repos

New repos should start from a small, consistent scaffold and then delete what they do not need. The goal is not maximal boilerplate; it is avoiding the same setup decisions on every project.

## Naming and remote

- Repo name matches the local directory name exactly.
- Lowercase, hyphen-separated names.
- One repo, one purpose.
- Default branch is `main`.
- Add a short GitHub description when creating the repo.
- Prefer private until there is a reason to publish.

## Minimum files

Every new repo should have:

- `README.md` - project purpose, status, setup, contribution note, and licensing.
- `AGENTS.md` or thin pointer to `agent-instructions.md` - see [agents.md](agents.md).
- `LICENSE` or `LICENCE` - chosen deliberately, not left ambiguous.
- `SECURITY.md` - vulnerability reporting policy for public repos.
- `.gitignore` - pruned for the stack, never copied blindly.
- `.editorconfig` - indentation, line endings, final newline.
- `s/` - repeated-process scripts once there is anything worth automating.
- `.github/dependabot.yml` - with cooldown and grouping as in [ci.md](ci.md).
- `.github/workflows/ci.yml` - once the repo has code to test.

Add `CONTRIBUTING.md`, issue templates, PR templates, `CODE_OF_CONDUCT.md`, markdownlint, pre-commit, and docs workflows when the project is collaborative or public. Copy and adapt the optional [community-health templates](examples/new-repo-template/) rather than writing them from scratch.

Add the root `/.private` ignore convention when the project needs retained local-only decks, reports, captures, or notes. Do not create the directory or boilerplate subfolders until they are useful. See [private-workspace.md](private-workspace.md).

## README minimum

Replace template README text before publishing. A project README should answer:

- What is this?
- What is it not?
- Who is it for?
- How do I install or run it?
- How do I test or contribute?
- What is the licence?

Use badges only when they carry real information. CI, licence, release version, OpenSSF Scorecard, and package registry badges can be useful. Stars, forks, "PRs welcome", and decorative badges are noise.

## Bootstrap runbook

1. Create an empty repository, then copy only the relevant standards and examples from house style.
2. Replace the README.
3. Choose and add the licence.
4. Prune `.gitignore` and language guide files.
5. Add `s/test`, `s/lint`, `s/docs`, or `s/up` only as they become real repeated processes.
6. Add CI once there is a meaningful check.
7. Add Dependabot for the ecosystems actually present.
8. Set the Solo `main` protection baseline: require CI, block force pushes and deletion, and document any deliberate exception. Select a higher tier from [ci.md](ci.md) before accepting collaborators, deploying production workloads, or handling high-consequence code.
9. For clinical or health-informatics software that can affect care, create `SAFETY.md` and record a medical-device applicability assessment at inception. See [clinical-safety.md](clinical-safety.md).
10. Link agent instructions to the adopted standards, for example <https://github.com/pacharanero/house-style/blob/main/AGENTS.md>.

## Do not over-scaffold

Do not add a docs site, release pipeline, clinical safety file set, or architecture templates to a repo that does not need them yet. The house style is a menu of standards to adopt when relevant, not an instruction to make every repo look the same size.
