# Rust CLIs

How command-line tools are shaped. The cross-cutting essentials in [AGENTS.md](AGENTS.md) state the basics; this expands them and adds the two rules that make a CLI composable and agent-friendly.

## Shape

- **Thin `main.rs`, reusable `lib.rs`.** `main.rs` parses arguments and delegates; all behaviour lives in the library so it can be embedded. A host CLI reuses the library verbatim rather than reimplementing it - [`gitehr/gitehr`](https://github.com/gitehr/gitehr)'s `gitehr calc` calls [`pacharanero/clincalc`](https://github.com/pacharanero/clincalc)'s `clincalc::cli::run`, so there is one implementation behind both the standalone `clincalc` binary and the subcommand.
- **`clap` derive, never the builder.** Subcommand enums with `#[derive(Subcommand)]`, args with `#[derive(Parser)]`.
- **`anyhow::Result` throughout** application code; reserve typed error enums for library crates that callers match on.
- **One module per command family**, each exposing a `run(...)`. The top-level `match` dispatches to `commands::<x>::run`.
- **A bare invocation must be helpful.** Running `<tool>` with no arguments or subcommand should never just error. It should do the most useful safe thing for that CLI: usually print a compact command list / usage summary, or for catalogue-style tools run the discoverability command, as [`pacharanero/clincalc`](https://github.com/pacharanero/clincalc) does for calculators. The same applies to command families: `<tool> documents` should print the `documents` subcommand help, not a bare "missing subcommand" error. Exit successfully unless there is a real runtime failure.
- **Every command has a short description.** Top-level commands and subcommands must have a concise `about`/doc-comment line so `--help`, completions, generated docs, and LLM/tool callers can discover the surface without blank rows or guesswork.
- **Command order is deliberate.** When listing commands in help, docs, or a bare invocation, choose exactly one ordering strategy and keep it consistent: either simple alphabetical order for easy lookup, or logical groups ordered by workflow with the most common / most important commands first. Do not let enum declaration drift become the user-facing order. If using logical groups, separate them with headings where the surface is large enough to need scanning.
- **Reset SIGPIPE on Unix** at startup so output pipes cleanly into `head`/`less` instead of panicking on a closed pipe.

## Output: data on stdout, everything else on stderr

The single most important rule for a composable CLI: **stdout carries the result; stderr carries hints, progress, and errors.** A user piping `tool x | jq` must get clean data on stdout, with any "did you mean..." guidance going to stderr where it cannot corrupt the stream. [`pacharanero/clincalc`](https://github.com/pacharanero/clincalc)'s `clincalc <name>` prints the JSON input template to stdout and the fill-it-in hint to stderr; `s/size` prints the table to stdout and tips to stderr. Get this wrong and the tool is unscriptable.

- A global `--format text|json[|yaml]` enum, honoured by every command. Text by default for humans; `json` for scripts and agents.
- Predictable, documented exit codes; a non-zero exit with a clear stderr message on any error.

## Progress on long-running work

Any command that can run for more than a second or two - building an index, streaming a large file, embedding a corpus - should show progress, because silence reads as "hung". Progress is a stderr concern (the [output rule](#output-data-on-stdout-everything-else-on-stderr) above), so it never touches the result stream, and it must vanish when no one is watching.

- **Draw to stderr, and auto-hide when it is not an interactive terminal.** A pipe, a redirect, a CI job, or an unset / `dumb` `TERM` must get clean output with no bar frames or carriage-return spam - the same run that animates a bar in a terminal is silent in a log file. `indicatif`'s default stderr draw target does this for you; the mistake is forcing a draw target that overrides it.
- **Show an ETA whenever a total is knowable.** Prefer a real bar over a bare spinner: a count bar (`pos/len`, per-second rate, ETA) for a loop over a known number of items, and a byte bar (bytes/total, throughput, ETA) for streaming a file of known size - drive the byte bar straight off the reader (`indicatif`'s `wrap_read`) so it advances with no per-loop bookkeeping. Fall back to a spinner only for a genuinely indeterminate phase (reading from stdin, an opaque library call).
- **Centralise the widget styling in one module.** A single helper that hands back a styled spinner / byte bar / count bar keeps every command visually consistent and stops four copies of the same `ProgressStyle` drifting apart. [`pacharanero/sct`](https://github.com/pacharanero/sct)'s `progress` module is the worked example, applied across `ndjson`, `sqlite`, `fst build`, `tct`, and `embed`.
- **Never let progress fight other stderr writes.** While a bar is live, route any interim message through the bar's own print method (`indicatif`'s `println` / `suspend`) so it lands cleanly above the bar instead of overwriting it; and where a lower layer already emits its own line-by-line breadcrumbs, leave that phase to them rather than double-reporting.

## Path and file arguments

Path arguments are the input half of the same contract, and they hide a sharp, easily-missed edge: **the shell does not always expand `~` for you.** A leading `~/` is expanded only in an unquoted, space-separated argument (`--db ~/x`); it is passed through *literally* in the `--db=~/x` (equals) form and inside quotes (`"~/x"`) - exactly the forms scripts and careful users reach for. Hand that literal `~/x` to `open()` and the tool fails with a baffling "no such file `~/...`".

- **Expand `~` yourself, on every path argument.** Wire one clap `value_parser` - a `fn(&str) -> Result<PathBuf, _>` that rewrites a leading `~/` (and a bare `~`) to `$HOME` - onto *every* `PathBuf` / `Option<PathBuf>` / `Vec<PathBuf>` argument, flags and positionals alike, so `~` works in every shell form everywhere and not just the one the shell happened to expand. [`pacharanero/sct`](https://github.com/pacharanero/sct)'s `paths::tilde_pathbuf` is applied to all ~75 of its path arguments this way. Do not scatter the expansion into individual `run()` bodies, where the next command forgets it and the surface drifts inconsistent; the value_parser makes it uniform and impossible to miss. Non-argument paths (values read from env vars, config files, or file contents) go through the same shared helper at their point of use.
- **Hint the type so completions help.** Give each path argument a `value_hint` (`ValueHint::FilePath` or `ValueHint::DirPath`) so the generated [shell completions](#shell-completions) offer files or directories for it. It composes with the value_parser: one advertises the shape, the other normalises it.
- **Name the flag after the artefact, not a generic role.** `--ndjson <FILE>` tells the user and the completion what kind of file is wanted; `--input <FILE>` does not. When you rename a flag for clarity, keep the old name as a `visible_alias` so existing scripts and muscle memory keep working.

## Configuration discovery

Configuration selection must be deterministic and diagnosable. Prefer one selected file over implicit layering unless the product has a concrete need for merged configuration and can explain every field's merge semantics.

Use this first-match-wins hierarchy for a tool named `<tool>`:

1. Global `--config <PATH>`.
2. Explicit `$TOOL_CONFIG` file path.
3. A documented project file in PWD, when it exists.
4. `$TOOL_CONFIG_HOME/<filename>`, falling back to `$XDG_CONFIG_HOME/<tool>/<filename>` and then `$HOME/.config/<tool>/<filename>`.
5. Optional XDG system directories and `/etc` locations only for tools with a real machine-wide administration use case.

There is no ancestor-directory traversal by default. A CLI or environment selector is explicit: expand `~`, require the selected file to exist and parse, fail with a clear error when it does not, and never fall through to a lower-priority source. Missing discovered candidates are skipped. Relative selectors are PWD-relative; relative asset and include paths stored inside a config file are relative to that config file's directory so a configuration bundle remains portable when invoked elsewhere.

Configuration is an input contract, not a best-effort suggestion. Parse and validate the selected configuration fully at load time. Reject malformed syntax, invalid values, incompatible types, unknown keys, and unknown sections with an error that names the file and offending field. Never silently ignore an unrecognised setting or quietly fall back to a broader default: a typo in a safety, security, or strictness setting must not disable the setting it meant to enable. In Rust, use typed deserialisation and `#[serde(deny_unknown_fields)]` on the root configuration type and every nested section type.

Test malformed syntax, an invalid value, an unknown key, and an unknown section. If a configuration-writing or migration command must preserve arbitrary third-party or future fields outside the part it edits, parse that narrow edit as a loose value or table, but keep the application's own load path strict. A deliberately extensible configuration surface is an exception: document its extension namespace and validation rules rather than accepting arbitrary unknown data.

Expose a safe bare `<tool> config` diagnostic command that prints the selected source, relevant environment variables, and discovery candidates without printing secrets. Honour the global machine-readable output format. Include normalised absolute paths in structured output, but preserve enough source information to explain why a path won. Test every precedence edge directly, including a missing explicit selector, PWD versus user config, XDG fallback, tilde expansion, and no-config behaviour.

## Declarative resource management (pull → edit → push → diff)

For a CLI that manages **stateful remote resources** (site settings, theme files, config objects, DNS records), the most powerful and least surprising shape is a declarative sync loop, not a pile of imperative setters.

- **`pull` snapshots to a local file** you can read, diff, and version-control. YAML by default, JSON when the path ends `.json`. Put a small header on the file - a schema `version` integer plus provenance (`pulled_at`, the source's identity/version) - so `push` can reject an incompatible schema and a human can see where it came from. Expand awkward server encodings into editable form (a JSON-array-inside-a-string becomes a real YAML list) and collapse it back on push.
- **`push` is idempotent and computes a plan.** Read the current server state, diff it against the file, and send **only what actually differs**. An unchanged push is a clean no-op, not a storm of redundant writes. Report the plan (changed / unchanged / unknown-on-server) rather than a bare "done".
- **Compare semantically, not byte-for-byte.** The file's serialisation (compact, sorted keys) rarely matches the server's (spaced, insertion-ordered). Comparing raw strings makes every field look "changed" and defeats idempotency - the single most common bug in this pattern. Parse both sides and compare the values.
- **`diff` is the read-only sibling of `push --dry-run`** - compare a snapshot (or one live instance) against another and print the delta with no option to apply. Drives drift detection across instances.

## `--dry-run` on every mutating command

Every command that writes remote state honours a global `-n` / `--dry-run`: it prints the exact plan it *would* execute - resolved names, the diff, the requests - and touches nothing. This is the review gate before acting on production, so the dry-run output must be *complete*: a provisioning command prints the full resource plan and settings diff, not a one-line summary. Read-only commands accept and ignore the flag so scripts can pass it uniformly.

## Guard destructive and irreversible actions

- **Refuse the dangerous case with a message that names the safe alternative**, rather than doing it. [`koloki-co/dsc`](https://github.com/koloki-co/dsc)'s `theme delete` refuses the site-default theme ("set another default first"); `theme field push` refuses a git-backed component ("the repo owns these; edit upstream and `theme update`, or `duplicate` it first"). A good refusal teaches the correct workflow.
- **Never print secrets.** Redact credentials embedded in URLs (`https://user:TOKEN@host` → `https://***@host`); never echo a minted API secret beyond the single unavoidable creation line.
- **Order multi-step provisioning so the enabling/irreversible step is last** ("enable last"): set up the dependencies, then flip the switch that activates them at the very end, so a mid-sequence failure leaves a disabled-but-clean state rather than a half-enabled broken one.

## Every capability is reachable

If the client library has a method that performs a distinct remote operation, there is a command that reaches it. An API wrapper wired to no command is dead capability - a `delete` you built but no one can call. Audit periodically: for each public client method, confirm a command references it (directly or transitively). A `pub fn` in a library crate suppresses the dead-code lint, so the compiler will not catch this for you.

## Version reporting

Version reporting is a command first, and a conventional flag second.

- **`<tool> version` is the canonical documented interface.** It is a command that reports data about the tool, so it should behave like the rest of the command surface and honour the global `--format` flag: `tool version --format json`.
- **`<tool> --version` and `<tool> -V` must still work.** They are the standard quick-check interface expected by humans, package managers, shell completions, and CI smoke tests. They should return a short human-readable string and exit immediately without config loading, project discovery, network access, or other side effects.
- **Any obvious attempt to get the version should be helpful.** `-V`/`--version` are the canonical flags, but a user reaching for the version will also try lowercase `-v` and the mistyped `-version`. Unless a short `-v` is already bound to something else (e.g. a global `--verbose`), map these bare forms to the same version output rather than letting them fall through to an "unexpected argument" error. clap only wires `-V`/`--version` from `#[command(version)]`; a tiny pre-parse check in `main()` that catches a lone `-v`/`-version` and prints the version covers the rest.
- **JSON version output should be structured.** At minimum include `name` and `version`; add `git_sha`, `target`, build profile, feature flags, or dependency versions where they are genuinely useful for debugging.
- **Docs and examples should prefer `<tool> version`.** Use `--version` / `-V` only for install checks and compatibility examples.

## Shell completions

Every shipped CLI supports shell completions. Good completions are part of the interface, not an optional flourish.

- Use `clap_complete` from the live `clap::Command`, so completions cannot drift from the actual command surface. Do not hand-maintain completion scripts.
- Support at least `bash`, `zsh`, `fish`, and `powershell`; add `elvish` where the tool already supports it.
- **`<tool> completions <shell>` prints the completion script to stdout.** This is the stable interface for package managers, installer scripts, and unusual setups.
- **`<tool> completions --dir <path> <shell>` writes the correctly named file into a target directory.** Use the shell's normal file names: `<tool>` for bash, `_<tool>` for zsh, `<tool>.fish` for fish, and `<tool>.ps1` for PowerShell.
- **`<tool> completions install [--shell <shell>] [--dir <path>]` is the human interface.** With no shell, detect the current shell from `$SHELL` / PowerShell environment where possible, create the standard user completion directory, write the file, and print any one-time shell configuration the user still needs. It must never silently edit `.bashrc`, `.zshrc`, PowerShell profiles, or other startup files.
- **Default Zsh user installs to `~/.zfunc`.** This is the house convention across CLIs. Tell the user to add `fpath=(~/.zfunc $fpath)` before `compinit` when it is not already configured; retain `--dir` for custom setups.
- Document `completions install` first. The redirect form is a fallback for advanced users and packaging, not the main install instruction.
- Add a focused test that completion generation succeeds for every supported shell and includes a representative top-level command. Where completions include dynamic values, test the injection point rather than relying on brittle full-script snapshots.

## Make the surface machine-discoverable

CLIs are now driven by scripts and LLMs as much as by humans. Design the surface so a caller can discover what to do without scraping `--help`:

- **`--schema` / `--print-schema`** emits a JSON Schema of a command's inputs - the authoritative input contract.
- A **fillable template**: `tool <name>` with no input prints a JSON object whose keys are the inputs and whose placeholder values describe them, so the command's output is the shape of its own input. [`pacharanero/clincalc`](https://github.com/pacharanero/clincalc)'s surface is the worked example: `list -> <name> (template) -> --schema -> --input` is a clean round-trip.
- Feed the *same* schema to other surfaces (an MCP tool's `inputSchema`), so humans, scripts, and models share one contract rather than three drifting copies.

## Registry-driven dispatch

Where a CLI exposes many similar things (calculators, generators, checks), keep a single registry (`all()` / `get(name)`) and drive every surface from it. Adding one entry to the registry surfaces it in the CLI, its schema, and any MCP or host integration with no per-item wiring. This is what keeps a fifty-command surface maintainable, and it pairs with the leaf-crate discipline in [library-extraction.md](library-extraction.md): the registry lives in the engine crate, and every consumer enumerates it.

## Enforce CLI contracts by walking the command tree

Most rules on this page are contracts that *every* command must satisfy: `--dry-run` never mutates, list commands offer `text|json|yaml`, path arguments carry a `ValueHint`, each command has an `Examples:` block. Enforcing those in code review means remembering them for every new command, and the failure is silent when someone forgets.

The clap command tree is itself a registry - each leaf is a member - so apply the registry/policy pattern from [testing.md](testing.md) to it. Walk `Command::get_subcommands()` recursively and assert the contract for every leaf. A new command that forgets a contract then fails the suite rather than review. Two useful shapes:

- **Introspection tests** are nearly free: read `get_possible_values()`, `get_value_hint()`, `get_after_help()` off the tree. No process spawning, no network.
- **Behavioural tests** run each mutating command against a local mock server and assert the invariant holds - [`koloki-co/dsc`](https://github.com/koloki-co/dsc) proves `--dry-run` issues no `POST`/`PUT`/`DELETE` across ~70 commands in about two seconds, offline, so it runs in ordinary CI rather than behind a live-test gate.

What separates a useful policy test from bureaucracy:

- **Name the offender; never assert a count.** `assert_eq!(count_path_hints(&cmd), 46)` cannot say *which* argument is wrong, and the obvious fix is to bump the number. Worse, counting the arguments that *have* hints means adding one *without* a hint leaves the count unchanged, so it passes - exactly how [`koloki-co/dsc`](https://github.com/koloki-co/dsc) shipped a `backup push` path argument with no completion hint. Collect violations and print them.
- **Pair every allowlist with a stale-entry assertion.** An exception for a command or argument that has since been renamed hides a real gap; assert each entry still exists in the tree.
- **Make the failure message state the fix**, including which list to add to and when each is appropriate. That is what stops the next person appending blindly to get back to green.
- **Reserve it for invariants that are universal and whose violation is silent.** A safety property like `--dry-run` earns the tax on every new command; a cosmetic one usually does not.

Where the classification needs judgement, prefer expressing it in source next to the command, as [`koloki-co/dsc`](https://github.com/koloki-co/dsc)'s `dry_run_refusal_reason` does, and having the test assert the registry is *total* over the tree, rather than keeping a parallel list in `tests/`.

Write behavioural and policy tests that exercise the invariant without contacting live services.
