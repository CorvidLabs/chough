# Lesson bundle: show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Show open questions in next and status, report an archived session as archived, and warn when an inbox observation has no record (issue 84)
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/main.rs, src/sessions.rs, tests/next.rs, tests/sessions.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: chough next shows the checkpoint open questions in text and JSON bounded to 10 with their total and chough status reports them,A question folded from a session inbox appears in next,session list reports an archived session with status archived,inbox add of an observation entry whose ID has no evidence record still adds it and warns on stderr naming the ID and inbox list marks it recorded false,Each new test fails on 290cce3 and fledge lanes run verify passes on Linux

## Evidence

- Verification commit: `f0b2668f22393708dec498fe92514ba78b8c198e`
- Base commit: `290cce3a024425cfd35600a43b8e458beaa89436`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/84 comes from a field test on
2026-11-03 against #77 head 2c53f44 (SpecSync 6.1.2):

- A subagent's `inbox add --kind question` folds into the session's
  `open_questions`, but `next`, `next --json` and `status` never show open questions,
  so the question is silently parked. The same holds for `checkpoint --question` on
  main: the checkpoint stores open questions and only `context` shows them.
- a. `session list` shows an archived session as `"archived": true, "stage": "done",
  "status": "active"`.
- b. `inbox add --kind observation --text OBS-bogus` exits 0 and folds; only a later
  `next` says the ID is unavailable.

This branch is stacked on #77 (`corvid-agent/feat/sessions-per-objective` at 290cce3),
because items a and b live in its sessions code; it retargets to main when #77 merges.

Criteria: LIFE-3 ("I can resume work with focused context, decisions, open questions,
and the next action") and DRIVE-3 already cover this; no new HI wording is needed.

Item b: on 2026-11-04 Leif chose "Warn, don't refuse" from offered options: `inbox
add` still adds an observation entry whose ID has no record, warns on stderr, and
`inbox list` marks it, because subagents may report before the parent tree has the
record and `checkpoint --observation` accepts unknown IDs too. Refusing was offered
and not chosen.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes, given in
a Claude Code session, and his brief the same morning to fix issues 81 to 88 one PR
each.

## From the change's design.md

# Design

- src/main.rs: the next view adds `open_questions` as `{"total", "items"}` through the
  existing `listed` helper (at most 10 entries, each cut to 512 bytes), it joins the
  lists `fit` may shorten, and the text form prints `Open questions: none` or
  `Open questions: N open` with one `  - <question>` line each, like blockers.
  `status` adds `open_questions` (every one) to its checkpoint section.
- src/sessions.rs: `session list` reports `status: "archived"` for an archived
  session. `inbox add` warns on stderr, naming the ID, when an observation entry's ID has no
  `.chough/evidence/<id>.json` record (`evidence::is_present`), and still writes the
  entry; `inbox list` adds `recorded: true|false` to each observation entry. Fold is
  unchanged.

## From the change's testing.md

# Testing

- tests/next.rs `open_questions_appear_in_next_and_status`: 12 questions from
  `checkpoint --question` show 10 with total 12 and "2 more" in `next` text and JSON,
  and all 12 in `status`; `--resolve-question` removes one from both.
- tests/sessions.rs `a_folded_inbox_question_appears_in_next`: the issue's repro.
- tests/sessions.rs `session_list_reports_an_archived_session_as_archived`.
- tests/sessions.rs `an_unrecorded_inbox_observation_is_added_with_a_warning`:
  `inbox add` with an unknown ID succeeds, warns on stderr naming it, and `inbox list`
  marks it `recorded: false`; a recorded one gets no warning and `recorded: true`.
- Each new test fails at 290cce3; `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
