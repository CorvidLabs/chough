# Lesson bundle: size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Size the checkpoint from measured use: a 1 MiB cap with at-cap budgets, session shelve, and sessions that never break each other (DRIVE-14, 125)
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: src/checkpoint.rs, src/sessions.rs, src/evidence.rs, src/main.rs, src/history.rs, tests/cli.rs, tests/next.rs, tests/sessions.rs, tests/stages.rs, tests/common/mod.rs, docs/cli.md, docs/architecture.md, CHANGELOG.md
- **Acceptance**: Every checkpoint is bounded at 1,048,576 bytes; at the cap next, status and context answer within 2 s in the debug test build (250 ms release) with the focused view within 16,384 bytes; one byte over is refused naming the cap and chough session shelve; chough session shelve <key> and --legacy move the oversized checkpoint byte for byte into .chough/sessions/shelved/ so session new and next work; one unreadable session never breaks list, new, use or another session next; retention reads shelved sessions; REQ-cli-021 AC1 holds 40 gates, 40 blockers and 50 decisions of 4 KiB within the cap. The new tests fail first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `bb8154ca295f7c6ba78039df24cd53c0956bbb93`
- Base commit: `ec830cfb001cc5f71f4f83809ca3294d685f42a4`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

## Intent and decisions

DRIVE-13, DRIVE-14 and KNOW-29 were captured on 2026-11-05 in Leif's words, relayed by the orc. Leif's decisions, via the orc:
- "figure out the good size for the checkpoints and resize... maybe one megabyte... we need to self test" (19:56 MST);
- the session-equals-change and keep-five intent (19:57);
- the 1 MiB cap and "Capture as written" (20:08);
- "Add shelve", including per-session robustness (20:12).

C3 (#125, audit D9) folds in here. At 1 MiB, REQ-cli-021 AC1's original shape is legal: 40 gates, 40 blockers and 50 decisions of 4 KiB each, about 380 KB. KNOW-29 (keep five archived sessions) is its own change and PR, after this one.

## Measured use

Measured across all CorvidLabs repositories and worktrees, with files deduplicated by content.

- **Per-change sessions on main:** 17 sessions of 985 to 1,813 bytes, with 1 to 3 decisions and 0 to 4 observations. One adopted legacy session is 73,231 bytes.
- **Legacy checkpoints:** 85 distinct files. The largest is 93,748 bytes (124 decisions, 388 observations, 110 completed). Others: jackdaw 29,308 bytes, raven 2,938 bytes.
- **Growth:** about 19 KB/day on the largest legacy checkpoint. The 200-entry list bound binds in about 8 days, before the byte cap does.
- **Ceiling:** the current list and entry bounds allow at most about 4 MiB.

## Benchmark

Release builds, warm median in ms. Cold times are within about 20% of warm. Every command exited 0.

| Checkpoint | macOS next | macOS status | macOS write | macOS context | Linux next | Linux status | Linux write | Linux context | Peak memory (MiB) |
|---|---|---|---|---|---|---|---|---|---|
| 128 KiB | 64 | 63 | 12 | 9 | 14 | 15 | 7 | 5 | 5–9 |
| 512 KiB | 68 | 68 | 15 | 9 | 15 | 16 | 10 | 7 | 8–9 |
| 1 MiB | 69 | 67 | 17 | 11 | 18 | 19 | 17 | 10 | 11 |
| 2 MiB | 74 | 142 | 23 | 13 | 23 | 24 | 28 | 18 | 13–19 |
| 4 MiB | 102 | 115 | 40 | 29 | 35 | 37 | 48 | 29 | 24–33 |

- **Debug build on Linux at 1 MiB:** next 151 ms, status 167 ms, write 289 to 553 ms.
- **Focused view at 4 MiB:** stays within 16 KiB (`next` text 16,270 bytes, JSON 16,239 bytes).
- **Full status report:** grows with the checkpoint, by design (3.28 MB at 4 MiB).

## The rule for the cap

The cap must be at least 10 times the per-change p99 (73,231 bytes, so at least 732,310) and at least 4 times the largest legacy checkpoint (93,748 bytes, so at least 374,992). 1,048,576 bytes is 14.3 times and 11.2 times those. At the cap, `next` and `status` are within 250 ms in release and within 2 s in debug CI.

## Recovery

Before this change, no command recovered from an over-cap checkpoint. In legacy mode, `next`, `session adopt` and `history adopt` all failed. In sessions mode, one oversized session broke `next`, `session new` and `session list` for every session. Leif chose "Add shelve".

## From the change's design.md

# Design

## Approach

- **The cap:** `SESSION_LIMIT` becomes 1,048,576. The refusal text names the cap and the shelve command for the file that was refused.
- **Per-session reads:** `sessions::keys_in` and readers (`list`, `new`, `use`, the active session's `next`) read each session independently. An unreadable one becomes an entry with `readable: false`, a reason, and "chough session shelve <key>". Only retention (`every_checkpoint`) still refuses on an unreadable session, and it also reads `shelved/` within 16 MiB.
- **Shelve:** `session shelve <key>` checks that the file exists, then moves it with a no-replace rename into `.chough/sessions/shelved/`. It writes `<key>.shelved.json` (size, SHA-256, time, reason), clears `.chough/local/active-session` if it named the key, and holds the checkpoint lock. `--legacy` does the same for `.chough/session.json` as `legacy`, creating `.chough/sessions/` so sessions mode starts.
- **Tests:** the test suite uses one `CHECKPOINT_CAP` constant.

## Risks

- A shelved session no longer drives work, but its bytes stay in place and in Git, and retention still protects what it references.
- Test runtime: the at-cap test writes 1 MiB files. That is fine.

## Constraints consulted

- REQ-cli-032, REQ-cli-021, REQ-history-006.
- DRIVE-14, DRIVE-13.
- The no-replace rule (#109).
- The 16 KiB focused view.

## From the change's testing.md

# Testing

## tests/cli.rs and tests/stages.rs

The checkpoint bound tests use `CHECKPOINT_CAP` (1,048,576): exactly at the cap is read, one byte more is refused naming 1048576, and the merge input and result bounds are checked. They fail first under the 131,072 cap.

## tests/next.rs

- `a_checkpoint_at_the_cap_stays_fast_and_one_byte_more_is_refused`: at exactly 1,048,576 bytes, `next`, `next --json`, `status --json` and `context` succeed in under 2 s, with `next` within 16,384 bytes. One byte more is refused naming 1048576.
- `handoff_blocks_stay_within_the_cap_and_are_byte_identical`: REQ-cli-021 AC1's full shape, with 40 gates, 40 blockers and 50 decisions of 4,096 bytes each, within the cap.

## tests/sessions.rs

- `one_oversized_or_unreadable_session_does_not_break_the_others`
- `shelving_an_oversized_session_lets_work_continue`
- `shelving_an_oversized_legacy_checkpoint_lets_a_session_start`

All fail first.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-032 | tests/next.rs `a_checkpoint_at_the_cap_stays_fast_and_one_byte_more_is_refused`; tests/cli.rs `checkpoint_byte_bound_is_exactly_1048576` |
| REQ-cli-021 | tests/next.rs `handoff_blocks_stay_within_the_cap_and_are_byte_identical` |
| REQ-cli-007 | tests/stages.rs `checkpoint_merge_enforces_input_collection_and_output_byte_bounds` |
| REQ-cli-034 | tests/cli.rs `oversized_records_fail_explicitly` |
| REQ-cli-042 | tests/cli.rs `oversized_records_fail_explicitly` |
| REQ-history-006 | tests/sessions.rs `one_oversized_or_unreadable_session_does_not_break_the_others`, `shelving_an_oversized_session_lets_work_continue`, `shelving_an_oversized_legacy_checkpoint_lets_a_session_start` |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
