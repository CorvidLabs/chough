# Lesson bundle: raise-the-checkpoint-session-byte-limit-to-131072-and-the-observation-pointer-limit-to-400

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Raise the checkpoint session byte limit to 131072 and the observation pointer limit to 400
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/checkpoint.rs, src/records.rs, src/main.rs, tests/, specs/cli/, docs/cli.md, CHANGELOG.md, hi/, INTENT.md, .chough/cases/
- **Acceptance**: The checkpoint (.chough/session.json) bound is 131072 bytes for reading, validating, writing, init and checkpoint --merge inputs and output; a checkpoint at exactly 131072 bytes is accepted and one over it is refused with a message naming 131072, leaving records byte-identical
- **Acceptance**: The checkpoint observations list accepts 400 pointers and refuses 401; every other checkpoint list keeps its 200-item bound and every other record keeps its 65536-byte bound
- **Acceptance**: Existing refusal tests keep their assertions with only the limit value updated; fledge lanes run verify passes; hosted CI is reported only as observed

## Evidence

- Verification commit: `ac57e4c2338a793b85c3eaad8ea3154446ac296f`
- Base commit: `747a12cc6fc396f0e8646fee1f2801e02b7d3035`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR #50 (issue #49 cleanups) could not take main 747a12c: a correct three-way `chough checkpoint --merge` produced a 68,737-byte checkpoint, over the 65,536-byte bound, with the observation pointer list already at its 200-item cap. Leif's agent posted a compaction plan on PR #50 (issuecomment-6595491835) and said its safe-only candidate leaves 110 bytes and no slots.

Human decision (Leif, 0xLeif, chat, 2026-11-02 9:40 PM MT): "Do your agent's cleanup and raise the progress file and test-result limits." Captured as HI LIFE-11. No history pruning. Approvals are recorded as corvid-agent under Leif's 2026-11-02 9:14 AM MT instruction "Use corvid-agent for all approvals for all things!! You are corvid-agent".

## From the change's design.md

# Design

Add `SESSION_LIMIT = 131_072` and `MAX_OBSERVATIONS = 400` in `src/checkpoint.rs`. Add bounded variants `read_bounded_record(path, limit)` and `write_new_bounded` / `write_replace_bounded(..., limit)` in `src/records.rs`; the existing functions call them with `RECORD_LIMIT`, so every other record keeps 65,536 bytes. Checkpoint reads, validation, init, saves, migration and merge use `SESSION_LIMIT`. `ensure_ids` takes its limit, and only `observations` passes `MAX_OBSERVATIONS`.

128 KiB is twice the old bound and keeps the checkpoint well under the 1 MiB receipt bound that history protection already uses to read it. 400 pointers at the current ~90-byte observation IDs use about 36 KiB, within the new byte bound.

## From the change's testing.md

# Testing

- `tests/cli.rs`: an exactly-131072-byte valid checkpoint is read by `context`; 131,073 bytes is refused naming 131072 (was 65,537 / 65536). Init over the bound still refuses before writing.
- `tests/stages.rs`: merge operand over 131,072 bytes refused in each position, naming 131072; merged result over the bound refused with inputs byte-identical.
- `tests/cli.rs`: 400 observation pointers validate, 401 refuse; 201 decisions still refuse; an evidence or bug record over 65,536 bytes still refuses.
- Full `fledge lanes run verify` with SpecSync 6.1.2.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-032, TC-cli-checkpoint-byte-bound-is-131072 | `tests/cli.rs::checkpoint_byte_bound_is_exactly_131072`: a valid checkpoint padded to exactly 131,072 bytes is read by `context`; 131,073 bytes is refused naming 131072 with the file byte-identical; a checkpoint whose content exceeds 131,072 bytes is refused. `oversized_records_fail_explicitly` (131,073 bytes, names 131072) and `init_rejects_unreadable_session_sizes_before_writing` (objective serialized over 131,072 bytes) keep their assertions with only the bound updated. |
| REQ-cli-032, TC-cli-observation-pointer-bound-is-400 | `tests/cli.rs::observation_pointer_bound_is_400_and_other_lists_stay_at_200`: 400 observation pointers validate, 401 are refused, and 201 `decisions`, `completed` or `bugs` entries are still refused. |
| REQ-cli-032 | `tests/cli.rs::records_other_than_the_checkpoint_keep_the_65536_byte_bound`: a preferences record over 65,536 bytes is still refused naming 65536. |
| REQ-cli-032, REQ-cli-007, TC-cli-checkpoint-merge-enforces-the-raised-bounds | `tests/stages.rs::checkpoint_merge_enforces_input_collection_and_output_byte_bounds`: an input over 131,072 bytes is refused in each position naming 131072; an input of exactly 131,072 bytes merges in each position; a merged result over 131,072 bytes from two in-bound inputs is refused with inputs byte-identical; 200 + 200 added observations merge to 400 and 200 + 201 are refused; the 300-decision and 201-consent refusals are unchanged. |

Each new or updated test fails with the bounds set back to 65,536 bytes and 200 observations (checked locally).

## Where these lessons go

- `specs/cli/context.md`
