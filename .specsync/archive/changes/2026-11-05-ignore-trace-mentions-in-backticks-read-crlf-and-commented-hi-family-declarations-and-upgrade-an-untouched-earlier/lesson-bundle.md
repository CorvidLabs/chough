# Lesson bundle: ignore-trace-mentions-in-backticks-read-crlf-and-commented-hi-family-declarations-and-upgrade-an-untouched-earlier

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Ignore Trace mentions in backticks, read CRLF and commented HI family declarations, and upgrade an untouched earlier knowledge README (review of 107 at 5326ec4)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/vault.rs, src/library.rs, tests/vault.rs, tests/fixtures/knowledge-readme-at-8fbbbe7.md
- **Acceptance**: A Trace list is read whether an inline backticked Trace mention comes before or after it; HI files with CRLF line endings or a YAML comment after the families list give their family; a knowledge README byte-identical to the template generated at 8fbbbe7 is upgraded to the current template by library index and init, and an edited README is left byte-identical. The new tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `bc61e626dd217d3a1783a87ff7cf96d8ddbbc4a8`
- Base commit: `5326ec48b4ae856543e4d8cabd7c06b29daef0e5`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

corvid-agent's review of #107 at 5326ec4 found three more gaps:
1. Reading the last `Trace:` on a line loses a list that a backticked mention follows
   (`Trace: UX-1, SEND-2. Readers grep the \`Trace:\` lines.`).
2. HI files saved with CRLF lost their family, because the front matter parser expected
   `\n` (Windows is a supported CI target).
3. A `knowledge/README.md` that an earlier Chough generated (#106, sha256 5365c2f9...)
   never gained the new front matter, because the README is create-only. This was
   raised in review 5779587370 by Leif's review agent, which asked to migrate only the
   exact old template and keep edited READMEs.

A YAML comment after the families list (`families: [SEND] # send flows`), noted as
non-blocking, is handled too.

## From the change's testing.md

# Testing

Two new tests in tests/vault.rs, both failing first:
- `inline_trace_mentions_crlf_and_comments_do_not_hide_criteria_or_families`: a
  backticked `Trace:` after the real list, a CRLF HI file, and a commented families list.
- `an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept`: the README from main
  8fbbbe7 (tests/fixtures/knowledge-readme-at-8fbbbe7.md) is upgraded by `library
  index`; an edited copy is left byte-identical by `index` and `init`.

The earlier inline-mention-before-list test still passes. `fledge lanes run verify` runs
on Linux.

## Where these lessons go

- `specs/cli/context.md`
