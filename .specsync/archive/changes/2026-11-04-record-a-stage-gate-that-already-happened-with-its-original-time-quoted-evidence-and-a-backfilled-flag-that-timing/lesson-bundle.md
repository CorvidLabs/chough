# Lesson bundle: record-a-stage-gate-that-already-happened-with-its-original-time-quoted-evidence-and-a-backfilled-flag-that-timing

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Record a stage gate that already happened with its original time, quoted evidence and a backfilled flag that timing reports separately (DRIVE-5)
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: src/stage.rs, src/timing.rs, src/history.rs, docs/cli.md, CHANGELOG.md, skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/lifecycle.md, tests/stages.rs, src/skill.rs, tests/workflow_authority.rs, tests/fixtures/workflow-at-edbc3c1-lifecycle.md, hi/drive.md
- **Acceptance**: Source: Leif's brief t811u item 5, "Want: `chough stage backfill` (or similar) that records a past gate with its original time, the approval evidence (commit, quoted choice), and a backfilled flag that `timing` reports separately.", quoted as DRIVE-5 and confirmed as his criteria at 5:24 PM MST (card t811s9271); the parent relayed at 6:03 PM MST that the hard rule holds: it records a past gate's original time, the quoted evidence and a backfilled flag that timing reports separately, never creates a human approval or review record, and refuses if the native state doesn't support the gate. Outcomes: (1) chough stage backfill --at <UTC time or unix seconds> --commit <rev> --quote <text> [--note <text>] makes the move advance would make now, to the next main stage, and records it as an advance transition whose time is the given past time and which carries backfill {recorded_at_unix_seconds, commit (full SHA), quote}; (2) it applies every refusal advance applies (inactive or side stage, open awaiting-human gate, blocking bugs and unlinked failures leaving test, and the destination's native prerequisite read now), so native state still decides the gate; (3) it refuses with nothing recorded a time in the future, a time before the latest recorded transition, an unreadable latest transition, a commit that does not resolve or is not an ancestor of HEAD, a blank or oversized quote, and the release to done move; (4) it records no human approval, review, consent or awaiting-human gate, and its output says the quote is evidence only; (5) timing lists backfilled moves separately with their recorded time, commit and quote, marks segments a backfilled move bounds, reports their backfilled time per stage, and leaves them out of estimate samples; (6) records without backfill keep their exact bytes, and the legacy log and history events both carry it; (7) docs, CHANGELOG and the lifecycle skill reference describe it, and `chough skill update` replaces an unedited v0.2.0 lifecycle reference with it; (8) the new tests fail before the change and fledge lanes run verify passes

## Evidence

- Verification commit: `eddcc6ace5d405f434fc92c0e5740444fe226d00`
- Base commit: `d673efc9fadd00953cf9e42dad96e08f3c76103c`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

On Raven on 2026-11-03, stages ran outside Chough (for example in subagents). The only
way to reflect them afterwards was to open and close gates now, so the transition log
showed the time of recording with "recorded after the fact" in the reason, and `timing`
reported the wrong durations. Leif's brief t811u item 5 asks for `chough stage
backfill` (or similar) that records a past gate with its original time, the approval
evidence (commit, quoted choice) and a backfilled flag that `timing` reports
separately (DRIVE-5).

The hard rule holds (parent, 6:03 PM MST): backfill never creates a human approval or
review record, and it refuses if native state does not support the gate. The change adds two
requirements: ADDED REQ-cli-036 in the cli spec (the command, its refusals and the
timing report) and ADDED REQ-history-005 in the history spec (the optional backfill
field on the transition record). REQ-cli-036 is kept short as a matter of style; specs
have no size cap (Leif, relayed 6:43 PM MST), so a later cli split is guidance only.
Main's v0.2.0 lifecycle reference becomes a known earlier bundle file, so `chough
skill update` delivers the backfill guidance over an unedited copy.

## From the change's design.md

# Design

- `chough stage backfill --at <time> --commit <rev> --quote <text> [--note <text>]`.
  `--at` takes `YYYY-MM-DDTHH:MM:SSZ` or Unix seconds.
- It runs `move_forward(kind = "advance")` with every existing refusal, so the
  destination's native prerequisite is read now. A gate passed earlier but no longer
  supported is refused, and backfill never asserts a past native state.
- The transition is an ordinary `advance` with `at_unix_seconds` set to the given time
  and `backfill: {recorded_at_unix_seconds, commit, quote}`. The field is omitted when
  absent, so other records keep their bytes. `commit` is the full SHA of a commit that
  is an ancestor of HEAD. `quote` is the human's or the record's words, kept verbatim
  as evidence only.
- Ordering: `--at` must be at least the latest recorded transition's time and at most
  now. An unreadable latest record refuses.
- `release` to `done` cannot be backfilled.
- `timing` reads the `backfill` field. Segments a backfilled move starts or ends carry
  `backfilled: true`, each cycle's stage totals add `backfilled`, a top-level
  `backfilled` list names every backfilled move with its times, commit and quote, and
  stage estimate samples skip stage totals with any backfilled segment.

## From the change's testing.md

# Testing

`tests/stages.rs` with the SpecSync stub: a backfilled plan to build move records the
given time, the full commit and the quote, and makes one native read; `timing --json`
lists it under `backfilled`, marks the bounded segments and leaves them out of
estimates; with history adopted the event carries the same field. Refusals with
nothing recorded: a draft change (native state does not support build), an open
awaiting-human gate, a future time, a time before the latest transition, an unknown or
unreachable commit, a blank quote, a side stage, and the release to done move
(`backfill_refuses_release_to_done`, which fails when that check is removed).
`tests/workflow_authority.rs`: an unedited v0.2.0 lifecycle reference is
known-outdated and `skill update` replaces it with the backfill guidance. No approval, review, consent or
gate record appears. The tests fail before the change. `fledge lanes run verify` runs
the full suite.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-036 | `tests/stages.rs`: `a_backfilled_move_keeps_its_original_time_and_quoted_evidence` and `backfill_refuses_what_advance_or_its_evidence_does_not_support`, `backfill_refuses_release_to_done` and `tests/workflow_authority.rs`: `lifecycle_guidance_from_v0_2_0_updates_to_the_backfill_guidance`. |
| REQ-history-005 | `tests/stages.rs`: `a_backfilled_move_keeps_its_original_time_and_quoted_evidence` (legacy log bytes) and `a_backfilled_move_is_carried_by_history_events`. |

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
