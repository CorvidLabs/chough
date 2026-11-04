# Lesson bundle: order-next-lesson-bundles-by-finalization-and-guide-a-fresh-cycle-to-a-new-native-change-issues-74-and-76

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Order next lesson bundles by finalization and guide a fresh cycle to a new native change (issues 74 and 76)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/main.rs, src/library.rs, src/stage.rs, tests/next.rs, tests/stages.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: chough next names at most 3 archived lesson bundles newest first by finalization timestamp then archive directory so they match the hot set of chough library index and an archive without a readable finalization timestamp sorts as oldest,With SpecSync configured and no native change recorded the command for a refused no-native-change gate is specsync change new instead of a bare specsync change status,After a stage reset while the recorded native change is archived with authenticated terminal evidence and was finalized before that reset next shows the new native change guidance with the specsync change new command at every main stage before done and never the archived-change refusal or another reset,A recorded change finalized after the latest reset keeps the existing archived-change guidance,docs/cli.md states the order and the guidance and each new test fails on main edbc3c1 and fledge lanes run verify passes

## Evidence

- Verification commit: `9e36a233b34f95f229de5f99185aebd58d4caee7`
- Base commit: `edbc3c1adecebf6a1378a731965c3bfe8c8217df`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Kimi's field lab round 3 (2026-11-03, Chough 4daf861, SpecSync 6.1.2) is an agent lab
review, not a human pass. corvid-agent reproduced the findings on v0.2.0 (edbc3c1) and
filed them as https://github.com/CorvidLabs/chough/issues/74 (N1),
https://github.com/CorvidLabs/chough/issues/75 (N2) and
https://github.com/CorvidLabs/chough/issues/76 (N3).

- Issue 74: `archived_bundles` in src/main.rs sorted archive directory names and
  reversed them, at most 5. SpecSync names archives `<UTC date>-<slug>`, so within a
  day the order is alphabetical by slug. `chough library index` orders by
  `(finalization.timestamp, dir)` and marks the newest 3 hot (KNOW-8), so `next`
  disagreed with the librarian and could drop a hot archive.
- Issue 76 item 2: at plan with no native change recorded, the no-native-change gate's
  command was a bare `specsync change status`, which cannot help.
- Issue 76 item 5: after `stage reset --to plan` from done, the archived old change hit
  the native-archived gate refusal (command: status of the old change), hiding the
  reset's next action. After `--to intent` then advance to plan, `next` treated the old
  archive as this cycle's and recommended another reset, a loop.

Authority: Leif's 2026-11-03 7:49 AM MT standing Chough maintainer authority for bounded
fixes to Chough bug tickets, and his 6:52 PM MT answer "File all three as Chough issues
and fix the clear bugs as small PRs".

Out of scope: issue 75 (no confirmed criterion covers interruption yet), issue 76 items
1, 3 and 4 (need Leif's wording; item 3 waits on DRIVE-6), and the open questions in
issue 74 (stale index note, automatic index). One change covers both issues because both
modify REQ-cli-021 and two open deltas of the same requirement would conflict.

## From the change's design.md

# Design

- src/library.rs: `HOT_ARCHIVES` becomes `pub(crate)`. New `finalized(root, dir)` reads
  an archive's `finalization.json` within the native record bound and returns its
  `change_id` and `timestamp` when the directory name ends with that ID (the same check
  `archives` uses). New `finalized_at(root, change)` returns the timestamp of the
  archive whose `change_id` is exactly `change`.
- src/main.rs `archived_bundles`: sort the directories that hold a `lesson-bundle.md`
  by `(timestamp or 0, name)` descending and show `HOT_ARCHIVES` of them. The JSON shape
  is unchanged (`total`, `items` of paths), so only the order and the count change.
- src/stage.rs `assess`: `previous_cycle` is true when the latest stage transition is a
  reset (the old rule) or when the newest reset's `at_unix_seconds` is not earlier than
  the recorded change's finalization timestamp. An archived, authenticated change that
  is the previous cycle's skips `archived_action`; when the next main gate is refused,
  the action is `Continue <stage>: <checkpoint next action>` with the command
  `specsync change new ...`. A refused `refused:no-native-change` gate also names
  `specsync change new ...`. Gates are unchanged, and no native read is added (the
  finalization file is local).

## From the change's testing.md

# Testing

- `tests/next.rs` `next_lists_the_newest_three_bundles_by_finalization_like_the_librarian`:
  six same-day archives whose slug order differs from their finalization order; `next`
  JSON and text list the newest 3 by finalization, total 6, and match the hot archives
  `chough library index` writes to `knowledge/INDEX.md`. An archive with no
  `finalization.json` sorts oldest.
- `tests/stages.rs` `plan_without_a_native_change_names_change_new`: SpecSync configured,
  no change recorded, at plan: the command is `specsync change new ...`.
- `tests/stages.rs` `reset_after_an_archived_change_guides_to_a_new_change`: done with an
  archived change finalized before the reset; `reset --to plan` and `reset --to intent`
  then `advance` to plan both give the command `specsync change new ...`, keep the
  reset's next action and never recommend another reset. A change whose finalization is
  after the latest reset keeps the archived-change guidance.
- Existing tests that pinned 5 bundles now pin 3, per the modified REQ-cli-021.
- Each new test fails with `src/` at main edbc3c1.
- `cargo test --locked` and `fledge lanes run verify` pass.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-021 | `tests/next.rs`: `next_lists_the_newest_three_bundles_by_finalization_like_the_librarian`; `tests/stages.rs`: `plan_without_a_native_change_names_change_new`, `reset_after_an_archived_change_guides_to_a_new_change`. |

## Where these lessons go

- `specs/cli/context.md`
