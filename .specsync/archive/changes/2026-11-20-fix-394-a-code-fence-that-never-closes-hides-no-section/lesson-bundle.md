# Lesson bundle: fix-394-a-code-fence-that-never-closes-hides-no-section

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 394: a code fence that never closes hides no section
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/markdown.rs, src/bugs.rs, tests/records.rs, tests/bugs.rs, docs/cli.md, specs/records/requirements.md, specs/records/records.spec.md, specs/cli/requirements.md, specs/cli/cli.spec.md, changelog.d/, .chough/cases/records.unclosed-fence.cases.toml, .chough/cases/cli.unclosed-fence-bug-merge.cases.toml
- **Acceptance**: A session whose body holds a code fence that never closes saves any number of times with each section once and chough next reads its decisions and questions, a closed fence still hides a heading it quotes, and two branches that each add a Log entry to a bug whose write-up holds an unclosed fence merge with both entries and no hand-resolved conflict.

## Evidence

- Verification commit: `00667016496539c7959f1481aab0b3af616cd13b`
- Base commit: `056cbb8029f6535ff74f866a0523893916f682db`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

#394 (62-F12, the 0.6.2 candidate swarm). A session body holding a three-backtick line
that never closes, as from `session new --objective '```'` or a pasted code block, makes
every save append new `## Decisions and why`, `## Open threads` and `## Timeline`
sections, since the section reader treats the rest of the body as code.

Measured on main 056cbb80 (t4b, 2026-11-20): `session new fence --objective '```'`,
then three `checkpoint --decision dN --question qN?`: 4 `## Decisions and why`, 4
`## Open threads`, 2 `## Timeline`; `chough next` says "Decisions: none" and "Open
questions: none". A `~~~` objective does not break it, since the reader knows only
backtick fences. A decision whose text is a fence opener does not either: it is written
as a list item.

The same toggle sits in the bug merge driver (`parts()` in `src/bugs.rs`). Measured:
two sides each adding a Log entry to a bug whose write-up holds an unclosed fence exit
1 with "both sides changed a paragraph differently; resolve this bug by hand", while the
same merge without the fence keeps both entries. That breaks REQ-cli-047.

REQ-records-001 already settles the rule for conflict markers: "a fence that never
closes hides nothing after it". REQ-records-004 says the sections are the only place
the session keeps decisions and open questions. The fix brings the two readers in line;
it adds no refusal and changes no approved sentence (one criterion added to each).

## From the change's design.md

# Design

Nothing visual changes. One fence rule in `src/markdown.rs`, shared by the section
reader (`section_span`) and the bug merge driver's `parts()`: a line opens a fence as
CommonMark reads one (three or more backticks or tildes, at most three spaces of
indent), and the fence hides the lines up to its closing line; a fence with no closing
line hides nothing after it. A closed fence still hides a heading it quotes, as the
existing `## Log` test pins.

Options considered:
- A (recommended): the rule above, as REQ-records-001 already says for markers.
- B: a fence also ends at the next heading of a section Chough owns. It would also stop
  a later closing line in a person's own section from re-hiding Chough's headings, but
  it breaks quoting a Chough heading inside a closed fence, which a test pins today.
- C: fold the objective when it is a fence line, at write. It covers only the objective
  and changes the text a person gave; A covers every source. Not taken.

## From the change's testing.md

# Testing

Each failing on main first:

- `tests/records.rs`: a session whose Goal is three backticks, saved three times with a
  decision and a question each, keeps one of each section, and `chough next` names the
  decisions and the questions; with `~~~` the same; a closed fence quoting `## Log` still
  hides it (the existing unit test).
- `tests/bugs.rs`: two sides each adding a Log entry to a bug whose write-up holds an
  unclosed fence merge with both entries and exit 0.

Then every verify step, the cases on macOS, and the tester's run on the real
repositories, which must show: every committed session, story and bug reads with the
same sections before and after (a count per record kind, and the list of any record
that differs, each one holding a fence that never closes); a save of a copy of each
record that holds an unclosed fence adds no section; and `chough bug merge` on each
such bug keeps both sides' Log entries.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
