---
change: flag-direct-specsync-lifecycle-writes-in-history-183-drive-21
artifact: tasks
---

# Tasks

- [ ] Add the REQ-cli-060 and REQ-history-013 tests in `tests/direct_writes.rs`, and change the slice-1 write-nothing test to the modified REQ-cli-057. They fail first.
- [ ] Add `src/direct_writes.rs`, the `direct-native-write` kind and `direct_write` field (`src/stage.rs`, `src/history.rs`), the change-step hooks (`src/change.rs`), the stage-move recording (`src/stage.rs`) and the next and status views.
- [ ] Update `docs/cli.md`, the lifecycle reference (both copies) and the CHANGELOG, naming the oldest Chough that reads the event.
- [ ] Add REQ-cli-060 and REQ-history-013 and modify REQ-cli-057 (materialized from the deltas).

After `specsync change check` materializes the deltas, not task boxes:
- record the cases' failing-first and passing observations;
- run the mutation round and record its logs;
- run `fledge run linux-verify`.
