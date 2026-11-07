---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: plan
---

# Plan

## Approach

Add `src/direct_writes.rs`, which:
- fingerprints the six lifecycle records of the active session's native change;
- keeps the baseline and pending findings in `.chough/local/native-writes/<id>.json`;
- renders findings for `next` and `status`.

The `chough change` steps update the baseline around their SpecSync step. Stage moves record findings as `direct-native-write` events in the same save as the move. `src/history.rs` and `src/stage.rs` accept the new kind and field.

## Out of scope

- Other changes.
- Authored artifacts.
- Refusing on a finding.
- Recording from `next` or `status`.
- Gate summaries (slice 4) and native gate records (slice 5).

## Steps

1. Add the tests in `tests/direct_writes.rs`, and change the slice-1 write-nothing test to the modified REQ-cli-057. They fail first.
2. Add `src/direct_writes.rs`. Add the event kind and the `direct_write` field (`src/stage.rs`, `src/history.rs`). Hook the change steps (`src/change.rs`), the stage moves (`src/stage.rs`) and the next and status views (`src/main.rs`, and `src/checkpoint.rs` if the view lives there).
3. Update `docs/cli.md`, the lifecycle reference (both copies) and the CHANGELOG, with the oldest reader named.
4. Materialize, record the cases, run a mutation round, and run `fledge run linux-verify`.

## Risks

- After a history holds the event, every older Chough refuses it, so mixed versions in one repository break. The change log says so.
- Approve and review stay direct until slice 5, so most changes will record one or two findings for them. That is the need DRIVE-21 wants seen.
- The baseline is local. A worktree removed before its next stage move loses its pending findings, and a fresh clone starts from first sight.

## Constraints consulted

- DRIVE-21 and DRIVE-4.
- DRIVE-2: no native read is added; the records are read as files.
- DRIVE-12: events are append-only.
- FLOW-14.
- Leif's rule: no refusal is added, so no recovery is needed.
