# Publicity

Work that nobody hears about has less impact than it should. Agents should notice when a piece of work is worth telling people about, and make that cheap: a drafted post and a link that opens the composer. A human always decides what gets published.

Strategy, audiences, account inventories, and voice analysis are kept in private notes, not in this public standard (see [Private publicity notes](#private-publicity-notes)).

## When to suggest an update

Suggest a post when a milestone lands, not after every commit. Good triggers:

- a release, or a user-visible feature reaching `main`;
- a new project, repository, or docs site going public;
- a talk, paper, demo, or event;
- a result worth sharing, such as a benchmark or an adoption by a new user.

Offer the suggestion at the end of the work, in one short block. Do not post, schedule, reply, or repost without explicit approval for that specific post.

## Per-project channels

Each project records which channels it publicises through, so agents do not have to re-derive this every time. Put a short `## Publicity` section in the repository's agent instructions (see [agents.md](agents.md)):

```markdown
## Publicity

- Owner: <organisation or person, matching the SPDX holder in licensing.md>
- Channels: <public handles or pages, e.g. a Bluesky handle, a LinkedIn page, a forum announcement topic>
- Announcement conventions: <tags, mentions, or a standing announcement topic, if any>
```

Choose channels from the project's owner and audience. The [affiliations table](licensing.md#affiliations) is the starting point: a Koloki project announces through Koloki and personal channels, an RCPCH project through RCPCH's channels as RCPCH directs, and so on. List only public handles and pages here. Reasons, audience analysis, and plans belong in the private notes.

## Drafting posts

- Write in the maintainer's voice, not a generic "professional" register. Where a voice reference is available in the private notes, sample it before drafting.
- One short post per channel, sized to the channel's limit. Lead with what the work does for the reader, then link to it. Prefer a link to the release notes, docs, or repo over a long description.
- Follow [the Markdown and prose rules](AGENTS.md#cross-cutting-essentials): hyphens, not emdashes, and no inflated claims.

## Posting by compose link

Hand the human a link that opens each platform's own composer, pre-filled. No API, no credentials, and the human clicks Post:

| Platform | Compose URL |
| --- | --- |
| X | `https://x.com/intent/post?text=<url-encoded text>` |
| Bluesky | `https://bsky.app/intent/compose?text=<url-encoded text>` |
| Mastodon | `https://<instance>/share?text=<url-encoded text>` |
| LinkedIn | No pre-fill link accepts custom text. Give the plain text to copy and paste. |

Encode the whole text, including line breaks and any link. Check each draft against the platform's character limit before building the link.

## Work journal

A maintainer may keep a private work journal of milestones across all their work, including business and code updates that are not for public channels. Where the private notes name one:

- Keep at most one journal entry per day. If today's entry exists, append to it; otherwise start a new one.
- Write a short line per milestone: what was done, with a link to the commit, PR, or release.
- Treat the journal as private. Never copy journal content into a public post, issue, or commit.
- Propose the entry and post it once the human agrees, unless they have already authorised journal updates for the session.

## Private publicity notes

The maintainer's private notes live at `.private/publicity.md` in their local house-style checkout (see [private-workspace.md](private-workspace.md)). That file holds the journal location and commands, the account inventory, the voice reference, and any posting tools. If an agent cannot read it, still suggest posts and build compose links, but skip the journal and say so.
