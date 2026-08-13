# Private Workspace

Use a root `.private/` directory for local material that has continuing value during work but must never become repository content. This is a cross-cutting convention, not an agent-tool scratch directory and not a substitute for tracked documentation.

## Default convention

Add this commented block to the repository `.gitignore` when `.private/` is useful:

```gitignore
# Local-only private workspace
/.private
```

The leading slash limits the rule to the repository root. Keep the directory untracked. Do not add placeholder files to force its creation.

Useful contents include:

- local work-digest decks and terminal recordings under `.private/decks/`;
- private notes, temporary reports, and review material that should survive one session;
- local fixtures or exports containing sensitive or licensed material that cannot be committed;
- generated comparisons, screenshots, captures, and exploratory results worth retaining locally;
- operator-specific helper scripts or configuration that are inappropriate as project defaults.

Prefer descriptive subdirectories such as `.private/decks/`, `.private/reports/`, `.private/captures/`, and `.private/notes/` rather than accumulating unrelated files at the root.

## Boundary

`.private/` means local-only, not automatically secure.

- Gitignore prevents accidental normal commits; it does not encrypt files, remove them from backups, or protect them from other local users.
- Never rely on `.private/` for production credentials, patient-identifiable data, or secrets that belong in a secret manager.
- Before sharing an archive, support bundle, or filesystem snapshot, check whether it includes `.private/`.
- If material becomes canonical project knowledge, sanitise it and move the durable version into tracked docs, specs, tests, or examples.
- If material is disposable tool session output, use the agent-artifact ignore block described in [agents.md](agents.md) instead. `.private/` is for intentionally retained local work.

## Agent behaviour

Agents should suggest `.private/` when they are about to create useful local artefacts that the user is likely to revisit but would not reasonably commit. Typical prompts include a private work digest, local benchmark report, exploratory capture, unpublished draft, or sensitive comparison.

Do not create `.private/` for every task. Ask or suggest it when the retention and privacy boundary is useful. Once the repository already uses the convention, continue it without inventing another local-output directory.

Agents creating `.private/` material must:

- ensure `/.private` is ignored before writing content;
- verify with `git check-ignore -v .private/<path>`;
- keep generated review artefacts inside `.private/`, not scattered through the worktree;
- state clearly that the content is local-only and will not be committed;
- avoid committing the private material even when the `.gitignore` change itself is committed.

## Recommended structure

```text
.private/
|-- decks/
|   `-- feature-name/
|       |-- index.html
|       |-- styles.css
|       |-- demo.cast
|       `-- view
|-- reports/
|-- captures/
`-- notes/
```

See [presentations.md](presentations.md) for the optional Reveal.js and Asciinema work-digest pattern.
