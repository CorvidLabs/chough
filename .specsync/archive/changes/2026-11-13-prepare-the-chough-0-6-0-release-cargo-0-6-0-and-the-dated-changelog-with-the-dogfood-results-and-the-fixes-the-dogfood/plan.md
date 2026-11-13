---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: plan
---

# Plan

## Approach

Drive this change with the 0.6.0 build only, as the dogfood cycle the 0.6.0 plan
names: approval and review run as Leif through Chough's gates (#236 for real), Chough
records written after `change check --commit` leave verification current (the #12
path), and the move into done keeps every earlier story byte-identical (#165).

## Out of scope

Tagging, releasing or publishing 0.6.0, and merging #238: each waits for Leif's go.
Contract changes, each chosen by Leif: D2 and the scope-gate summary (REQ-cli-061), the
preview warning and the receipt's `none` (REQ-records-009), D1's pointer and receipts in
time order (REQ-history-006), the journal (REQ-records-011), temporary paths in case
definitions (REQ-records-006), `next` at plan (REQ-cli-021), a bug's trigger and Fix
(REQ-cli-012) and labelled Timeline fields (REQ-history-002). Any other the dogfood finds goes
back to Leif as a send-back.

## Steps

1. D1: a red test (a migrated project whose local pointer names a 0.5.x key), then
   the fix in `src/sessions.rs` and `src/doctor.rs`; a bug record red then green.
2. Rename the test helper `session_id` in `tests/migrate_evidence.rs` to
   `session_folder`.
3. KNOW gaps: link KNOW-3, KNOW-6 and KNOW-24 from the active cases that test them,
   run the cases of KNOW-3, -4, -6, -15 and -24 with `chough cases run`, and recompute
   the gap list from `hi/knowledge.md` and the records until it is empty.
4. #148: run its regression test against the code before #157 (red), link it, run it
   on this tree (green), and mark the bug verified.
5. The four flaky-test bugs: reproduce each under load, fix and verify what has a
   cause, and leave open with its diagnostics, named to Leif, what never reproduces.
6. D2 (REQ-cli-061 delta): a red test (a second change on one branch counts the
   earlier change's files), then the base selection in `src/gates.rs`; #190's test
   stays green. docs/cli.md says the same, and its "planned after 0.6.0" note goes.
7. The migrate preview warning: a red test (a preview with an open native change gives
   no warning), then the warning in `src/migrate.rs`; the bug record red then green.
8. The journal (REQ-records-011): red tests, then `checkpoint --title/--summary`, the
   derived fallback and `journal add --record` (`src/checkpoint.rs`,
   `src/session_md.rs`, `src/journal.rs`); the workflow skill tells the agent to set
   the title and summary before the move into done; docs/cli.md says the same.
9. Temporary paths in case definitions (REQ-records-006): a red test, then doctor's
   scan in `src/doctor.rs`; the bug record red then green.
10. The backfill: each of the 68 earlier archived stories gets its post through
    `journal add --record`, with a title and summary written from that story; 0.6.0's
    post is removed with Git and written again the same way; `library index`
    regenerates the index and the feed, and `--check` passes.
11. Lessons: a few cross-project lessons from this release in `knowledge/lessons/`,
    each sourced to recorded evidence or a story.
12. The fresh-eyes fixes, each red then green with its bug record: the cases check
    crash, the context walk, `next` at plan, the scope-gate summary, the Timeline
    fields, a bug's trigger and Fix, receipt order, the migration receipt's `none`,
    `gate record`'s output and note, the authority line, the plain check's entry, and
    `status` and the stale next action; the new-project docs (README, docs/cli.md and the
    skill: `specsync change adopt` and the delta format); the CHANGELOG and docs
    corrections (drift checks red then green); and jackdaw's finding that a migrated
    `session.md` can end with a blank line, red then green; and t38b's stop survivor,
    a child forked as a stop arrives, fixed in `src/records.rs` and rerun red then green
    under load.
13a. The second fresh-eyes round (t38k's F1 to F13, t38h's (1), (2) and (6)), each red
    then green with its bug record where it is a bug: every gate close names a
    configured human (REQ-records-008); `next` after a close, at accept and with long
    IDs (REQ-cli-021); the gate list's reply line (REQ-cli-061); bugs found in a session
    linked to it (REQ-cli-012); `evidence add` with delta requirements and optional
    links (REQ-cli-005); `chough change check --no-commit` and direct writes recorded
    in order (REQ-cli-057, REQ-cli-060); the story's verified list and front matter
    (REQ-history-006); a `start` entry (REQ-history-002); `merge_driver_outdated`
    (REQ-history-012); the migrate preview on a migrated tree (REQ-records-009); the
    output polish and the docs gaps. Helpers t37f, t38b and t38c write the red and fix
    commits on their own branches; t382 runs and records red and green.
13b. The third fresh-eyes round (t38m's F1 to F12), each red then green with its bug
    record where it is a bug: `next` at accept never passes on SpecSync's review command,
    and at test leaves superseded runs out (REQ-cli-021); `session close` records the
    gates it withdraws, and the story puts its receipts last and names its bugs
    (REQ-history-006); gate summaries name the intent and the delta (REQ-cli-061); a
    failed run's body says why (REQ-cli-017); stage moves print a short summary, the
    full JSON behind `--json` (REQ-cli-008); the migrate preview is text with `--json`
    (REQ-records-009); and the docs and polish items. Helpers t37f, t38b and t38c write
    the red and fix commits; t382 runs and records red and green, then a full local run.
13c. The fourth round's four real findings, each red then green with its bug record: a
    stage move replaces a gate close's next action, and at accept `next` names the
    stale cases' rerun before the review gate (REQ-cli-021); the scope summary counts
    the working tree (REQ-cli-061); the migration's follow-ups name `chough skill
    update` for an outdated skill copy (REQ-records-009); accept.md writes lessons before
    the case rerun. F5 to F11 and the host-name rule recorded as deferred bugs. Then the
    full local suite.
13. Cargo 0.6.0 (Cargo.toml and Cargo.lock), and the CHANGELOG's dated [0.6.0]
    section with the dogfood results in its upgrade notes.
14. This session's post title and summary, then `change check --commit`, Leif's review as him through `chough change ship
   --close-gate`, and the move into done with its two-step closeout.

## Risks

- A flaky test may not reproduce: it stays open, named to Leif, and blocks the tag,
  not this change.
- #148's red needs the test applied to the code before #157; if that code cannot run
  the test, the record says so and the bug goes to Leif.

- The backfill's summaries are written by agents from the stories: each says only what
  its story records, and the story stays the full record.

## Constraints consulted

AGENTS.md (SpecSync owns the lifecycle, Chough only its records); the cli lessons (a
delta is frozen at approval; `change check` refuses unticked tasks, so human steps
are plain text); the 0.6.0 plan's dogfood plan and ticket ledger.
