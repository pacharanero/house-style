# Presentations

Reference: <https://github.com/pacharanero/reveal-js-cdn-template>. The Reveal.js agent skill at <https://github.com/ryanbbrown/revealjs-skill> is useful for agent-assisted deck creation. URL-per-slide presentation tools are experimental and not yet the default.

Presentations are usually web-native slide decks. The default is **Reveal.js from a CDN**, edited as a small static folder and served locally for testing.

## Default stack

Use Reveal.js unless there is a strong reason not to.

- Keep the deck as static HTML/CSS/assets.
- Load Reveal.js and plugins from a CDN.
- Do not add Reveal.js as a git submodule.
- Do not vendor a full local copy of Reveal.js into the presentation repo.
- Pin the CDN version when reproducibility matters. The current verified stable release is `6.0.1`, for example `https://cdn.jsdelivr.net/npm/reveal.js@6.0.1/dist/reveal.css`.

The starter is <https://github.com/pacharanero/reveal-js-cdn-template>. Copy it for new decks rather than starting from scratch.

## Folder naming

Presentation folders use a year prefix and slug-case title:

```text
YYYY-talk-title
```

Examples:

- `2026-gitehr`
- `2026-openehr`
- `2025-lets-talk-digital-clinical-safety`

Use the year the talk is given or created. If no year is specified, use the current year. Use lowercase words separated by hyphens; no spaces or underscores.

## Deck structure

Prefer one `index.html` plus local assets:

```text
2026-talk-title/
|-- index.html
|-- custom.css
|-- README.md
|-- AGENTS.md
`-- img/
```

Markdown-in-HTML Reveal decks are fine. Keep the separator regexes conventional:

- `-----` for horizontal slides.
- `---` for vertical slides.
- `Note:` for speaker notes.

## Two-dimensional Reveal flow

Use Reveal's two-dimensional navigation deliberately.

- Horizontal slides (`-----`) are the main storyline.
- Vertical slides (`---`) are optional depth: examples, evidence, caveats, backup material, or alternate detail under one topic.
- Do not flatten everything into a single long horizontal sequence when a topic naturally has a drill-down path.
- Keep the main horizontal path coherent if the presenter skips the vertical stacks.

This gives a talk a strong spine while keeping supporting material close to the slide that needs it.

## Slide style

Favour fewer words and more images.

- One idea per slide.
- Short, punchy headings.
- Strong statements, rhetorical questions, and concrete examples.
- Fragments for pacing, not for dumping long bullet lists.
- Prefer screenshots, diagrams, photos, and source-linked images over dense prose.
- Use speaker notes for cue words, timings, links, or details the audience should not have to read.

Avoid long paragraphs, crowded slides, decorative gimmicks, and multiple unrelated ideas on one slide.

Use hyphens, not emdashes, in visible text.

## Images and assets

Put local images under `img/` or `assets/`. Use descriptive filenames. Prefer real screenshots or relevant source images over generic stock-like visuals.

When using external images:

- Link the image or nearby text to its source where practical.
- Keep enough provenance in comments, notes, or README for later attribution.
- Optimise large images enough that local serving and browser navigation stay fast.

## Local serving

Test slides through a local HTTP server, not by opening `index.html` directly, because browsers apply different rules to local files.

The preferred lightweight server is the Rust `http-server` crate when available:

```sh
cargo install http-server
http-server .
```

If the repo has a script, use `s/serve` or `s/present` as the stable entry point and have it wrap the server. The script should print the local URL and pass through server arguments where useful.

Other simple static servers are acceptable when already present, but do not introduce a Node dependency just to serve a static Reveal deck if `http-server` is available.

## Agent-assisted deck creation

For agent work, the Ryan Brown Reveal.js skill is a useful accelerator: <https://github.com/ryanbbrown/revealjs-skill>. Use it for planning, scaffold generation, visual checks, overflow checks, screenshot review, and PDF/export support where helpful.

The skill is a tool, not the house style. Generated decks still need to conform to this standard:

- Reveal.js loaded from CDN unless there is a clear exception.
- No vendored Reveal.js tree or submodule.
- Minimal, image-led slides.
- Two-dimensional Reveal structure where useful.
- Local browser testing before considering the deck ready.

If the skill's default scaffold differs from the local CDN template, prefer the local template and apply the useful generated layout/content patterns into it.

## Optional work digests

For a substantial feature, refactor, investigation, or milestone, a short local Reveal.js **work digest** is an effective optional mechanism for reducing context-recovery cost. Suggest one when the user is moving across several repositories, the diff is difficult to understand from a written summary, or the work has commands, architecture, evidence, and scope boundaries worth seeing together.

Work digests are self-paced technical explanations, not polished conference talks. Keep them under the repository's gitignored [`.private/`](private-workspace.md) workspace unless the user explicitly wants to publish or commit them:

```text
.private/decks/feature-name/
|-- index.html
|-- styles.css
|-- demo.cast
`-- view
```

Aim for two to five minutes and roughly seven or eight horizontal slides:

1. **What changed** - one-sentence outcome.
2. **Why it matters** - the previous problem and practical benefit.
3. **Before and after** - concrete source state versus resolved state.
4. **How it works** - one architecture or data-flow diagram.
5. **Demo** - real commands and representative output.
6. **Safety and behaviour** - guardrails and failure cases.
7. **Evidence** - tests, corpus results, limitations, and independent checks.
8. **What next** - the implemented boundary and remaining work.

This is a starting structure, not a required template. Delete slides that add no value. Use vertical slides only for optional drill-down evidence; the horizontal path should remain a complete rapid briefing.

### Personal visual identity

Work digests should be recognisably Marcus's rather than looking like generic generated decks.

- Use Reveal's pinned **Solarized** theme as the starting point. The current verified stable release is Reveal.js `6.0.1`: `https://cdn.jsdelivr.net/npm/reveal.js@6.0.1/dist/theme/solarized.css`.
- Use **Oswald** for headings and short display text, with a restrained sans-serif body font such as Arial or Helvetica. Load Oswald from Google Fonts when network access is acceptable, and provide `Arial Narrow, Arial, sans-serif` fallbacks.
- Preserve Solarized's warm paper background, ink-like text, blue accents, and muted secondary colours. Extend it sparingly rather than replacing it with generic dark cards, neon gradients, glassmorphism, or interchangeable SaaS styling.
- Prefer diagrams, terminal output, screenshots, strong typography, and whitespace over grids of decorative cards.
- A project with an established visual identity may override these defaults deliberately. Record the exception in the deck rather than drifting accidentally.

Minimal setup:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reveal.js@6.0.1/dist/reveal.css">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reveal.js@6.0.1/dist/theme/solarized.css">
<link rel="stylesheet" href="styles.css">
```

```css
@import url("https://fonts.googleapis.com/css2?family=Oswald:wght@400;500;600;700&display=swap");

.reveal h1,
.reveal h2,
.reveal h3 {
  font-family: "Oswald", "Arial Narrow", Arial, sans-serif;
  font-weight: 600;
  text-transform: none;
}
```

Check upstream releases before creating a new long-lived deck and update pins deliberately rather than using `latest` CDN URLs.

### Reveal.js creation workflow

1. Ensure `/.private` is present in `.gitignore` and verify the target with `git check-ignore -v`.
2. Create `.private/decks/<feature-name>/` and scaffold static `index.html` plus `styles.css`. The Reveal.js agent skill may generate the initial structure.
3. Use the pinned Reveal.js CDN core and Solarized theme. Add Oswald headings in the custom CSS.
4. Write the rapid horizontal storyline first. Add optional vertical evidence only where it helps.
5. Use real repository commands and measured results. A digest is evidence-backed documentation, not an invented product narrative.
6. Serve over HTTP and inspect the rendered deck at its 1280 x 720 design size and in phone landscape. A portrait phone may show a rotate hint rather than reflowing a fixed presentation.
7. Check every slide for overflow, clipping, wrapping, contrast, missing icons/assets, and transition-state artefacts. Capture and visually inspect every stable slide, not only a sample.
8. Keep temporary screenshots and browser-automation output inside `.private/` or delete them after review.

The Ryan Brown skill's scaffold script, overflow checker, screenshot/PDF workflow, and browser text editor are useful implementation tools. Follow the skill's visual-review requirements while applying this house visual identity.

### Asciinema terminal demos

Use an embedded Asciinema recording when the dynamic sequence adds understanding beyond a static command block. Keep it short, deterministic, synthetic-data-only, and free of credentials, usernames, machine-specific secrets, or distracting setup.

Preferred recording workflow when the `asciinema` CLI is available:

```sh
asciinema rec -c './demo-command' demo.cast
```

Choose a new filename or confirm an intentional overwrite if the cast already exists. For multi-command demos, put deterministic commands in a local helper script under `.private/` and record that script. Do not add Asciinema as an application dependency.

Embed the local cast using a pinned Asciinema Player CDN release:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/asciinema-player@3.17.0/dist/bundle/asciinema-player.css">
<script src="https://cdn.jsdelivr.net/npm/asciinema-player@3.17.0/dist/bundle/asciinema-player.min.js"></script>
<div id="terminal-demo"></div>
<script>
  AsciinemaPlayer.create("demo.cast", document.getElementById("terminal-demo"), {
    autoPlay: true,
    loop: true,
    fit: "width",
    speed: 1.15
  });
</script>
```

If the CLI is unavailable but the output is deterministic, an agent may create an asciicast v2 `.cast` file from the exact commands and captured output. State that it is a generated playback, not a literal keystroke recording. The format is newline-delimited JSON: one header followed by timestamped output events. Preserve realistic pacing, terminal dimensions, and command wrapping.

Always include a readable static `<pre><code>` fallback containing the same essential output. Hide it only after the player loads successfully. This keeps the slide useful offline, under restrictive content policies, or when the CDN cannot load.

If custom CSS gives the fallback an explicit `display` value, preserve the HTML `hidden` contract:

```css
.terminal-fallback[hidden] {
  display: none !important;
}
```

The current verified stable Asciinema Player release is `3.17.0`. Check the recorder's local `asciinema rec --help` because overwrite and long-option spelling differ between CLI generations. Record output only by default; do not enable input capture, which can record passwords and other typed secrets.

### Local viewing helper

For a private digest, a tiny untracked `.private/decks/<feature>/view` helper is enough:

```sh
#!/usr/bin/env bash
set -euo pipefail

deck_dir="$(cd "$(dirname "$0")" && pwd)"
port="${1:-8765}"

printf 'Open http://127.0.0.1:%s/index.html\n' "$port"
exec python3 -m http.server "$port" --directory "$deck_dir"
```

Using Python here is acceptable because this is a local-only convenience helper, not a committed project runtime or canonical repository script. If the project already has `http-server` or an `s/present` convention, use that instead.

### Review example

A good work digest can combine a static Reveal.js deck, an embedded generated asciicast with a static fallback, a local viewing helper, and desktop/mobile/overflow screenshot review. Retain a non-public digest through the root `/.private` ignore convention when it should not be committed.

## Publishing

For a standalone talk repo:

- Keep the repository name the same as the folder slug.
- Public repos are normal for public talks.
- Include a short `README.md` with title, one-sentence description, event/date if known, and slides URL.
- License presentation content as CC-BY-SA-4.0 unless there is a specific reason not to.
- Include a final slide with the slides URL, source repo, and relevant further-reading links.

For a deck inside a docs site, link it from the docs nav or a presentation page. A static Reveal deck can live under `docs/presentation/` when it is part of a project documentation site.

## Alternatives

Use alternatives only for a clear reason:

- PDF export when a venue requires a static handout or upload.
- A native app or browser-tab presenter for a specific demo-heavy talk.
- `tabsentation` / `Present` style URL-per-slide workflows when the point of the talk is live web material.

URL-per-slide presentation tools are promising but not yet house style. Mention them as experimental, not as the default pattern.
