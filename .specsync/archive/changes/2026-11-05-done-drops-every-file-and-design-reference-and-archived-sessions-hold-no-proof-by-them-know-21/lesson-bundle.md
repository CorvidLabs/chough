# Lesson bundle: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Done drops every file and design reference, and archived sessions hold no proof by them (KNOW-21)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/stage.rs, src/evidence.rs, tests/reduce.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: stage advance into done drops every file and design reference from the checkpoint and reports them as dropped_files and dropped_design, keeps current-cycle observation references under the KNOW-16 rule, and a cold loose folder a dropped reference held is then trimmed; retention ignores an archived session file and design references without rewriting it. Both new tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `a5625b8cb991b81fdf4a3be91a771d07d7517635`
- Base commit: `9991d09cb153ac409a43c5071ad5371ddd53fe49`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

## KNOW-21

"When my change reaches done, the checkpoint also drops its file and design references from closed cycles, so the old folders they name stop holding proof." Leif captured it as written on 2026-11-05.

## Leif's choice: "Drop all at done"

2026-11-05, 16:29 MST, via the orc. File and design references record no cycle or time, so they can't be split by cycle the way KNOW-16 splits observations. When a change reaches done its cycle closes, so done drops every file and design reference from that checkpoint. Retention ignores the file and design references of archived sessions, which are all done. The native archive and the vault keep what the change touched.

Two options were offered and not chosen:
- record the cycle with each reference from now on;
- drop only references to `.chough/` paths.

## What this repository holds today

- Only the adopted legacy session, now archived, holds file references: 58 of them.
- They protect about 48 old proof items, such as `archived session adopt-event-…` among the librarian's protections.
- Per-change sessions hold none.

## Out of scope

- Closed bugs releasing their observations (KNOW-22, its own change).
- Ledger rotation (KNOW-23).
- The wider ownership rules (KNOW-24).

## From the change's design.md

# Design

## Approach

1. **Drop at done.** `drop_closed_cycle_references` in src/stage.rs keeps its observation rule. It now also takes every file and design reference.
   - It saves once and reports the dropped `dropped_files` and `dropped_design` lists.
   - A missing reset time skips only the observation part.
   - An unreadable record still refuses the whole step, so nothing is dropped.
   - A failed save restores all three lists.
2. **Ignore archived sessions in retention.** `file_references` in src/evidence.rs skips checkpoints labeled `archived session <key>`, matching the label `crate::sessions::every_checkpoint` gives an archived session. Archived session files are never rewritten.

## Risks

- Display: the checkpoint at done no longer lists file and design references. The native archive keeps the change's paths, and the history keeps its transitions.
- Tests: the #109 folder-reference protection test used the done trim. Since the done move now drops the reference first, it is exercised by `library archive --apply` before done, with every assertion kept.

## Constraints consulted

- REQ-cli-041, REQ-cli-045, REQ-cli-046.
- KNOW-13, KNOW-16, KNOW-18.
- The specs/cli/context.md lessons on retention and the #108 review: an unreadable record refuses.

## From the change's testing.md

# Testing

## tests/reduce.rs

- `done_drops_every_file_and_design_reference`:
  - no reset is recorded;
  - the checkpoint names a cold loose folder file, a source file and a design file, and observation u3;
  - done reports `dropped`, with both files and the design file;
  - it empties both lists and keeps u3;
  - the cold folder is trimmed.

  It fails first: the step reports `skipped`.
- `an_archived_session_file_reference_protects_nothing`:
  - in sessions mode, an archived session names a cold loose folder by file and design reference;
  - the done trim removes the folder;
  - the archived session file is unchanged.

  It fails first: the folder is protected by `archived session old`.
- Existing tests:
  - `done_drops_checkpoint_references_from_closed_cycles` and `an_unreadable_record_keeps_its_reference_and_refuses_the_drop` still pass;
  - `a_checkpoint_folder_reference_protects_loose_files_and_observation_sets` now runs through `library archive --apply`, with every assertion kept.

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
