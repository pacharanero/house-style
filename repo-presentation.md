# Repository Presentation

Reference: [Standard Readme](https://github.com/RichardLitt/standard-readme), [awesome-readme](https://github.com/matiassingers/awesome-readme), [Readme Driven Development](https://tom.preston-werner.com/2010/08/23/readme-driven-development.html), and the README conventions of well-presented open source projects such as [HTTPie](https://github.com/httpie/httpie), [Doom Emacs](https://github.com/doomemacs/doomemacs), [Sniffnet](https://github.com/GyulyVGC/sniffnet), and [Tauri](https://github.com/tauri-apps/tauri).

A repository should be exciting to arrive at and fast to act on. A reader who lands on the GitHub page should understand what the project is, whether it is for them, and how to start using it - within the first screenful. Installation should be the easiest section to find, not buried below philosophy or architecture.

This standard covers the README and the repo's first impression: its description, topics, social preview, release notes, and linked docs site. For documentation site conventions see [docs.md](docs.md). For interface design see [ui.md](ui.md). For new-repo file minimums see [new-repos.md](new-repos.md).

## Principles

- **Cut to the chase.** The reader's need comes first. Lead with what the thing is and how to get it running. Earn the right to explain philosophy, architecture, and roadmap later.
- **One screenful to value.** A first-time visitor should be able to read the opening, see a visual, find install instructions, and run a minimal example without scrolling past essays.
- **Show, do not tell.** A screenshot, GIF, or terminal recording communicates more per line than prose. Use a real visual of the actual product, not a stock illustration.
- **Signal, not decoration.** Badges, icons, and visual elements must carry real information. Stars, forks, "PRs welcome", and "made with heart" badges are noise. CI, licence, release, and package-registry badges are signal.
- **Be honest about status.** State whether the project is production-ready, experimental, or archived. Do not imply maturity that does not exist.
- **Keep it maintained.** A README with stale badges, broken demo links, or outdated install steps is worse than a shorter one. If a section has rotted, fix it or remove it.

## Above the fold

The first screenful is the most valuable real estate in the repository. Put these elements at the top, in this order:

1. **Project name and one-line description.** The GitHub repo description field should carry the same one-liner. The reader should know what the project is before they finish the heading.
2. **Badges.** A small row of informative badges directly under the title. CI status, licence, latest release, and package-registry version (if published) are the core set. OpenSSF Scorecard is useful for public repos. Keep it to one row; wrap to a second only when there is genuine signal to show.
3. **Hero visual.** One screenshot, GIF, or terminal recording that demonstrates the primary workflow. For a CLI, show the terminal output. For a GUI, show the main screen. For a library, show a code snippet and its result. No hero visual is better than a generic stock image.
4. **Quick start.** The shortest path from zero to working software. Prerequisites, install command, and a minimal usage example - copy-pasteable, tested, and labelled with the OS or environment where it applies. Link to a fuller guide for advanced setup.
5. **What it is.** One or two sentences expanding the one-liner: what problem it solves and who it is for. State what it is *not* if that prevents common misunderstandings.

## Quick start rules

Installation is the section readers scan for first. Make it prominent, concrete, and current.

- Use the actual install command for the primary platform. `cargo install <crate>`, `pip install <package>`, `brew install <formula>`, or a one-line `curl|sh` installer are all fine. Show the real command, not a placeholder.
- Label prerequisites: language version, OS, or external runtime the reader must have first.
- Show a minimal runnable example immediately after install. One command, one line of output, done.
- Separate user installation from contributor setup. A user needs to install and run; a contributor needs to clone, build, test, and link to [CONTRIBUTING.md](examples/new-repo-template/CONTRIBUTING.md). Do not mix them in one block.
- Test the quick start in a clean environment. If it does not work from scratch, it is not a quick start.
- For CLI tools, follow the CLI shape in [rust-cli.md](rust-cli.md): a bare invocation must be helpful, `--help` exits 0, and `--version` works.

## Sections after the fold

Use only the sections the project needs. A small library does not need an architecture diagram or a roadmap; a clinical platform does.

- **Features.** Three to five concrete benefits, not abstract adjectives. "Calculates eGFR using CKD-EPI 2021" is a feature. "Powerful and modern" is not.
- **Configuration.** A compact table of environment variables, flags, permissions, or defaults when the project has them. Link to a fuller reference page rather than reproducing every option.
- **Usage.** A few real examples that go beyond the quick start. Show the common cases, not every flag.
- **Architecture.** A short section or a separate `ARCHITECTURE.md` for projects with non-obvious structure. A source-code map, a diagram, and the key design decisions. See [specs.md](specs.md) for where deeper design lives.
- **Documentation.** One link to the docs site or `docs/` directory. Do not reproduce the docs in the README.
- **Roadmap or changelog.** Link to `ROADMAP.md` or `CHANGELOG.md` rather than maintaining a duplicate list in the README. Spotlight one or two near-term items only if they help a reader decide whether to adopt now.
- **Contributing.** One sentence plus a link to `CONTRIBUTING.md`. Do not reproduce the full contribution guide.
- **Community.** Link only to channels that actually exist and are monitored. A dead Discord link is worse than no community section.
- **Licence.** State the SPDX identifier, the copyright holder, and link to the full `LICENSE` file. See [licensing.md](licensing.md). A one-line statement is enough; the file is authoritative.

## Table of contents

Add a table of contents when the README is long enough that a reader might search for a specific section. A short README with five sections above the fold does not need one. A long README with architecture, configuration, multiple usage examples, and a roadmap does.

Use a flat Markdown list with anchor links. GitHub auto-generates heading anchors from heading text. Keep the TOC to one level of depth unless the README is genuinely large.

## Badges

Use [shields.io](https://shields.io/) badges or equivalent. Keep them in one row directly under the title.

Informative badges:

- CI build status
- Licence
- Latest release / package version
- OpenSSF Scorecard (public repos)
- Coverage (only if coverage is actually tracked in CI)

Noise badges - do not use:

- Stars, forks, download counts
- "PRs welcome", "made with heart"
- Code-style badges (Prettier, Black, Ruff) - redundant if CI enforces them
- Last-commit or last-update badges - a reader can see the commit history

## Visuals

- Use real screenshots or recordings of the actual product, not stock images or generic illustrations.
- Keep GIFs short (a few seconds) and small enough to load fast on a mobile connection.
- For terminal recordings, use a tool like [vhs](https://github.com/charmbracelet/vhs) or [asciinema](https://asciinema.org/) to produce clean, reproducible output.
- Store screenshots under `docs/img/` or a top-level `img/` directory. Use descriptive filenames.
- Optimise image file sizes. A 5 MB PNG in a README is a bad experience on mobile and slow connections.
- Add an `alt` description to every image for accessibility. A screenshot of a CLI output should have alt text that summarises the output, not just "screenshot".

## Repo metadata

The README is one surface. The repository's GitHub presentation should match it.

- **Description.** Set the one-line GitHub description to match the README one-liner. Do not leave it blank.
- **Topics.** Add 3-8 relevant topics for discoverability. Use standard terms (`rust`, `cli`, `healthcare`, `fhir`) rather than invented tags.
- **Social preview.** Set a custom social preview image (1200x630 PNG) so shared links look intentional rather than auto-generated.
- **Releases.** Write release notes that a user can act on. Link the changelog, summarise notable changes, and call out breaking changes. See [distribution.md](distribution.md).
- **Linked docs site.** If the project has a Zensical docs site ([docs.md](docs.md)), link it in the README, the GitHub repo description, and the Pages settings. The docs site and README should agree on what the project is.

## What to avoid

- A wall of text before the quick start. Readers leave.
- A README that duplicates the documentation site. Link instead of copying.
- Stale badges that show a failing build or a release that no longer exists. Fix or remove.
- A "Built with" section listing every framework in the stack. That belongs in `CONTRIBUTING.md` or the docs, not above install.
- Decorative emoji headings. They render inconsistently and add visual noise. Use plain headings.
- An empty `## Roadmap` or `## FAQ` section. Remove sections that have no content rather than leaving placeholders.
- A README that has not been updated since the first commit. The README is a living document; update it when the project changes.

## Template

A starting point for a project README. Cut sections that do not apply.

````markdown
# Project Name

One-line description that matches the GitHub repo description.

[![CI](https://github.com/<owner>/<repo>/actions/workflows/ci.yml/badge.svg)](https://github.com/<owner>/<repo>/actions/workflows/ci.yml)
[![Licence: AGPL-3.0](https://img.shields.io/badge/Licence-AGPL--3.0-blue.svg)](LICENSE)
[![Latest release](https://img.shields.io/github/v/release/<owner>/<repo>)](https://github.com/<owner>/<repo>/releases)

![Screenshot or GIF](docs/img/screenshot.png)

## Quick start

```sh
<install command>
```

```sh
<minimal usage example>
```

Prerequisites: <language version, OS, or runtime>.

## What it is

One or two sentences expanding the one-liner. What problem it solves and who it is for.

## Features

- Concrete benefit one.
- Concrete benefit two.
- Concrete benefit three.

## Configuration

| Variable | Default | Description |
|---|---|---|
| `FOO` | `bar` | What it controls. |

## Documentation

Full documentation: <https://<owner>.github.io/<repo>>

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Licence

<chosen licence>. See [LICENSE](LICENSE).
````

## Browser extensions and packaged apps

Some surfaces have their own first impression beyond the GitHub README. Apply the same principles there.

- **Browser-store listing.** Treat it as a deployment surface with its own release checklist, screenshots, privacy disclosure, and fresh-profile QA. Keep it in sync with the repo README.
- **Trust boundary.** Document where credentials are stored, which hosts receive data, which permissions are optional, and whether any intermediary service exists. Readers and users need this before installing.
- **Generated bundles.** If the store package needs committed bundles, identify them as generated, forbid hand-editing in agent guidance, and rebuild them during validation.
- **Extension testing.** Test extension-specific behaviour, not only pure functions: mock `chrome.storage`, exercise settings migrations, test permission flows, and verify popup state transitions.
- **Version sync.** Keep `manifest.json` and package metadata in sync with an automated version check.
- **Scoped permissions.** Prefer scoped user API keys and optional host permissions. Document the minimum permissions needed for each feature.
