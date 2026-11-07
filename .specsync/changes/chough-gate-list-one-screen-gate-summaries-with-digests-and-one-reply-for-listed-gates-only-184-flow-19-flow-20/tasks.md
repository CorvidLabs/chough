---
change: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
artifact: tasks
---

# Tasks

- [ ] Add the REQ-cli-061 and REQ-history-014 tests in `tests/gates.rs`, and the ship-with-list test in `tests/change.rs`. They fail first.
- [ ] Add `src/gates.rs` and the `gate` command; the relay fields and their validation; the close-with-digest path; and ship's `--list` and `--gate`.
- [ ] Update `docs/cli.md`, the README row, the accept and lifecycle references (both copies) and the CHANGELOG, naming the oldest reader.
- [ ] Add REQ-cli-061 and REQ-history-014, and modify REQ-cli-059 (materialized from the deltas).

After `specsync change check` materializes the deltas, not task boxes:
- record the cases;
- run the mutation round;
- run `fledge run linux-verify`.
