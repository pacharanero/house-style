# Queries

Questions that need a maintainer decision before the first public release. Answer in place; the relevant files can then be completed without reopening the publication boundary.

## Q1 - Code Example Licence

The prose and standards are CC-BY-SA-4.0. Which permissive software licence should apply to the copyable scripts and Rust examples: MIT, Apache-2.0, BSD-2-Clause, or another option? Until decided, these examples are marked as pending and should not be treated as released under a permissive software licence.

Decision: MIT

## Q2 - Code Of Conduct

Should the repository adopt the Contributor Covenant 2.1 before public launch? If yes, should reports go to private GitHub contact with the maintainer, or a dedicated public email address?

Decision: yes let's go with it. I'm not sure how private GitHub contact would work, there's no DM feature on GH. Use contributing@bawmedical.co.uk

We should be clear, since this is a metarepository, that repos using this house-style may need to ascertain their own reporting arrangments depending on whether it's a pacharanero, Baw Medical, Kololi or completely unrelated project

## Q3 - Security Reporting Service Level

Is GitHub private vulnerability reporting plus private GitHub contact sufficient initially, or should `SECURITY.md` name an email address and acknowledgement/response timeframes?

Decision: For most of my work you could use security@bawmedical.co.uk

## Q4 - Standard Distribution

The current candidate supports linking to its public GitHub URLs or copying relevant files. Should the first release also document a preferred install location for Claude Code, Codex, OpenCode, or other agent platforms?

Decision: link to or copy relevant standards and skills. Do not prescribe agent-specific installation locations in the first public release; users should follow their agent platform's documentation.

## Q5 - Publication Scope

The candidate retains clinical-safety, medical-informatics, Discourse, and Tauri/Rust defaults as Marcus's personal standards, but removes private source repositories and workstation links. Is that intended public scope, or should any of those standards move to a later healthcare or organisation-specific variant?

Decision: yes, this is a public house-style, but it's still primarily for my use. I just wanted to publish it for the benefit of others.
