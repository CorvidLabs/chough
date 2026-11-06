# Lesson bundle: pin-every-history-and-inbox-bound-at-exactly-its-value-154-ch-19

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Pin every history and inbox bound at exactly its value (154, CH-19)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: tests/history.rs, tests/sessions.rs
- **Acceptance**: Each of the five unpinned history and inbox bounds has an end-to-end test at exactly its value and one past it, and a mutation that loosens any of the 11 history and session bounds by one is caught; no product, contract, assertion or bound changes; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `b49e9681fdc36fb550e57f835068d5c777dff26c`
- Base commit: `64aa77e01d2763395d8af7566e62743ede37cc32`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

Issue #154, the audit's CH-19 ("Sessions/history" row, unverified): "Every history capacity, orphan, backfill, prune and inbox path has independent end-to-end evidence."

A mutation check loosened each of the 11 history and session bounds by one and ran the history, sessions, reduce and unit tests. Five mutations survived: `RECENT_BYTES` (512 KiB), `CYCLE_BYTES` (16 MiB), `RECEIPT_BYTES` (1 MiB), `INBOX_LIMIT` (16,384 bytes) and `MAX_INBOX` (200). The other six were already caught. The script is `_worktrees/.prof105/mutate-ch19.sh`.

These are test gaps only: each bound is enforced, but no test pinned its exact value.

## From the change's design.md

# Design

## Approach

The history tests build exact layouts with the padded synthetic event chain. Inbox entries are padded with JSON whitespace. Each test asserts the observable result at the bound and one past it: events read, the refusal text, or listing.

## Risks

The 16 MiB test writes about 16 MiB of event files, which takes about 0.4 s.

## Constraints consulted

REQ-history-003 and REQ-history-009. The rule against changing assertions or bounds.

## From the change's testing.md

# Testing

## tests/history.rs

- `recent_traversal_stops_at_exactly_512_kib`:
  - with 100 events of 8,192 bytes, `status` reads exactly 64 events (524,288 bytes);
  - with the 64th event read one byte larger, it reads 63 (516,096 bytes).
- `a_cycle_of_exactly_16_mib_is_full_and_one_byte_more_is_over_its_bound`:
  - a cycle of exactly 16,777,216 bytes is read whole, and `stage iterate` refuses with "capacity reached";
  - one byte more refuses with "partial history";
  - the checkpoint is unchanged either way.
- `a_receipt_of_exactly_1_mib_is_read_and_one_byte_more_is_refused`:
  - a receipt of exactly 1,048,576 bytes lets an interrupted prune resume;
  - one byte more refuses, and the remaining files stay.

## tests/sessions.rs

- `an_inbox_entry_of_exactly_16_kib_is_read_and_one_byte_more_is_refused`: an entry file of 16,384 bytes is listed; at 16,385 bytes, listing refuses and names 16384.
- `an_inbox_holds_200_unfolded_entries_and_refuses_the_201st`: the 201st add refuses, naming 200 unfolded entries and the fold.

## Mutation check

Each test failed against its loosened bound. After these tests, all 11 mutations are caught.

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/history/context.md`
