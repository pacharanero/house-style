# Writing

This applies to all prose in a repository and around it: docs, READMEs, specs, roadmap items, commit and PR bodies, issue comments, release notes, posts, and agent hand-offs to the user.

Write for a busy reader who knows the domain. Say the specific thing, once, and stop.

## Defaults

- Lead with the point. The first sentence should tell the reader what changed, what to do, or what the page is for.
- Prefer concrete detail to description: a command, a number, a file path, an example. "Lookups take 6 ms" beats "dramatically faster".
- Use plain words and short sentences. Cut words that do no work.
- Claim only what the evidence supports. If something is untested or uncertain, say so plainly.
- Match the length to the job. A one-line change gets a one-line commit body, or none.
- British English spelling, unless the repository already uses another.

## Formatting

- Do not hard-wrap Markdown. Write each paragraph as one line and let editors soft-wrap. Line breaks are fine where they carry meaning: list items, headings, code, tables.
- Use hyphens, not emdashes. The emdash is one of the most recognisable marks of machine-drafted text, and a reader who spots it starts skimming. A hyphen with spaces, a comma, brackets, or a full stop does the same job.
- Use sentence case for headings.
- Use bold sparingly, for the one thing a skimming reader must not miss. Bold on every bullet means nothing stands out.
- No emoji in technical writing, unless the repository already uses them deliberately.

## Patterns to avoid

Machine-drafted text tends to have these, and so does a lot of corporate writing. Before finishing any prose, check it against this list and rewrite what matches:

1. Inflated significance: "a pivotal moment", "plays a crucial role", "a testament to", "marks a significant step". Say what it does.
2. Promotional adjectives: "seamless", "robust", "powerful", "cutting-edge", "comprehensive", "game-changing". Replace with the fact that made you reach for the adjective.
3. "Not just X, but Y" and "It's not X, it's Y". Just say Y.
4. Lists of three by reflex: "fast, reliable, and secure". Keep three only when there are really three things.
5. Vague attribution: "experts say", "it is widely recognised", "many users find". Name the source or drop the claim.
6. Hollow "-ing" clauses tacked onto a sentence: "..., highlighting the importance of testing". Delete them or make them a real sentence.
7. Swapping "is" for "serves as", "stands as", "represents", or "boasts".
8. Synonym cycling: calling the same thing three different names to avoid repetition. Use one term, the canonical one (see [specs.md](specs.md#ubiquitous-language)).
9. Openers and closers that say nothing: "Great question", "Certainly!", "In summary", "In conclusion", "Overall", "I hope this helps". End when the content ends.
10. Hedging stacked on hedging: "may potentially help to some extent". Commit to the claim or state the uncertainty once.
11. Narrating the process instead of reporting the result: "I then carefully reviewed...". Say what was found.

## Sources

These patterns come from Wikipedia's [Signs of AI writing](https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing) and the [humanizer skill](https://github.com/blader/humanizer) built on it. Read them for the full catalogue and examples; the list above is the subset agents fall into most often.

## Related standards

- PR descriptions: [commits.md](commits.md#pull-request-descriptions).
- Public posts: [publicity.md](publicity.md).
- Interface copy: [ui.md](ui.md).
