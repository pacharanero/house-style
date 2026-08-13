# Tauri GUI

The preferred desktop GUI stack is **Tauri 2 + React + Mantine + Vite**, with Rust owning domain behaviour and the webview owning interaction design. A GUI should be another surface over the same project engine, not a second implementation of the product.

## Project shape

Use this layout unless the repo already has a different settled structure:

```text
repo/
|-- gui/
|   |-- package.json
|   |-- vite.config.ts
|   |-- src/
|   `-- src-tauri/
|       |-- Cargo.toml
|       |-- tauri.conf.json
|       `-- src/
|-- docs/gui/
|-- spec/gui.md
`-- s/gui-dev
```

Keep the GUI in a `gui/` subdirectory so the Rust workspace, CLI, docs, and desktop app each have clear ownership.

## Rust owns behaviour

Domain logic belongs in reusable Rust crates. Tauri commands should call those crates directly and return typed serialisable data to the frontend. Do not reimplement scoring, parsing, validation, version handling, or business rules in TypeScript because the GUI needs them.

The TypeScript layer owns view state, forms, layout, keyboard interaction, copy/paste affordances, and presentation. It should be able to ask Rust for domain operations and render the response.

## Frontend defaults

All presentation and interaction work follows [ui.md](ui.md). This file adds the Tauri-specific architecture and tooling choices rather than defining a separate visual standard.

- React + TypeScript.
- Mantine for UI primitives.
- Vite for development and builds.
- Keep theme configuration in one place, usually `gui/src/theme.ts`.
- Prefer ordinary application screens over marketing-style landing pages. GUI apps open into the useful workflow.
- Use the same terminology as `CONTEXT.md` and docs.

## Scripts

Every GUI repo gets:

- `s/gui-dev` - runs the Tauri development server from any working directory.
- `s/test` - includes Rust tests and any frontend checks the repo has adopted.
- `s/lint` - runs `cargo fmt --check`, clippy, and frontend lint/type checks where configured.

`s/gui-dev` is the canonical entry point. Do not make contributors remember `cd gui && npm run tauri dev`.

## Docs and spec

Keep two kinds of GUI writing separate:

- `spec/gui.md` records the design decisions, workflow model, and deferred questions.
- `docs/gui/` explains how to use the GUI and how to run it locally.

If the GUI has special test setup, put it in `gui/e2e/README.md` or `docs/gui/testing.md` and link from `AGENTS.md`.

## Versioning and distribution

GUI package versions must move with the repo's canonical version. If the Rust workspace, npm package, and Tauri config all carry a version, `s/version++` updates all of them in the same release commit. Do not let the GUI drift onto a separate release clock unless it is genuinely a separate product.

Tauri release workflows still follow [distribution.md](distribution.md): action SHAs pinned, version bump first, tag/release triggered by the standard release model, and no separate `s/release`.

## Testing

At minimum, the Rust commands exposed to Tauri are tested as ordinary Rust functions. GUI smoke tests should focus on the user workflows that prove the Rust/frontend boundary is wired correctly. Avoid brittle visual snapshots unless the project has a real design regression need.

Before considering GUI work complete, perform the responsive, keyboard, state, asset-rendering, and screenshot checks in [ui.md](ui.md). For desktop-only windows, test the supported minimum, default, and large window sizes rather than pretending the surface has phone breakpoints.
