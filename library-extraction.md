# Reusable libraries and extracting them

When a piece of a project is useful beyond it, make it a reusable crate from the start and let the host be its first *consumer*, not its owner. The discipline below keeps the crate embeddable and lets it move into its own repo cleanly when it earns one.

## Leaf-crate discipline

Keep the reusable core a **leaf**: it depends only on what it needs to compute (typically just `serde`/`serde_json`), never on the host application, and never on an async runtime. Dependency arrows point *into* the core; the core points at nothing above it. This is what makes it embeddable in any surface and detachable into its own repo.

- Pure logic: no clock, no I/O, no global state. The host stamps timestamps and does I/O.
- A typed result schema the core owns, shared by every surface.
- A registry (`all()` / `get(name)`) so surfaces enumerate one list (see [rust-cli.md](rust-cli.md)).

> [`pacharanero/clincalc`](https://github.com/pacharanero/clincalc) with `default-features = false` depends only on serde/serde_json - no [`gitehr/gitehr`](https://github.com/gitehr/gitehr), no tokio - so one engine drives the CLI, an MCP server, a GUI, and web tools, and it lifted into its own repo unchanged.

## Dependency progression: path, then git, then crates.io

A consumer depends on the library through three stages as it matures:

1. **Path dependency** (`path = "../lib"`) during co-development in one workspace or side-by-side checkouts. Zero friction, but the consumer cannot build without the sibling present.
2. **Git dependency pinned to a rev** (`git = "...", rev = "..."`) once the library has its own repo. Reproducible (especially where the consumer does not commit `Cargo.lock`), CI-friendly, no sibling checkout needed. Pin the rev rather than tracking a branch.
3. **crates.io** (`lib = "0.x"`) once there is a release/publish pipeline, so downstream users and `cargo install` work without `--git`.

**Private git-dependency gotcha:** a git dep on a *private* repo cannot be fetched anonymously. Use an SSH URL (`ssh://git@github.com/org/repo`) plus a `.cargo/config.toml` with `[net]\ngit-fetch-with-cli = true`, and in CI load a read-only deploy key with `webfactory/ssh-agent`. Making the library repo public removes all of this - prefer it for a genuinely open library.

## Extracting to its own repo, with history

When the library earns its own repo, move it with `git subtree split` so its commit history travels with it rather than landing as one squashed "import" commit:

```bash
# in the monorepo: split the crate's history onto a branch
git subtree split --prefix=lib-core -b export-lib-core

# in the target repo: add it as a subdirectory, history intact
git subtree add --prefix=lib-core ../monorepo export-lib-core
```

Then remove the crate from the monorepo (`git rm -r lib-core`), drop it from the workspace `members`, and repoint the consumer's dependency to stage 2 above. Because the core is a leaf, the consumer's glue code does not change when the source moves from a path to a git dep - that is the payoff of the discipline.
