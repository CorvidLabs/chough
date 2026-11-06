---
change: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
artifact: tasks
---

# Tasks

- [ ] Add the REQ-cli-059 tests to `tests/change.rs` with a stand-in that ships and reports archived. They fail first.
- [ ] Extract the `--close-gate` relay check into a shared function in `src/checkpoint.rs`, with no behavior change.
- [ ] Add `chough change ship` to `src/change.rs`.
- [ ] Move the walkthrough, `docs/lifecycle.md`, `docs/cli.md`, the README row, the CHANGELOG and the skill's accept, lifecycle and stages references (both copies) to `chough change new`, `check` and `ship`.
- [ ] Add REQ-cli-059 (materialized from the delta).

After `specsync change check` materializes the delta, not task boxes:
- record the cases' failing-first and passing observations;
- run the mutation round and record its logs;
- run `fledge lanes run verify` on Linux.
