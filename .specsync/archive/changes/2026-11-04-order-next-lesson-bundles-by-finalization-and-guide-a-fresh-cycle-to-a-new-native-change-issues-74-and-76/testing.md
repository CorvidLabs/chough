---
change: order-next-lesson-bundles-by-finalization-and-guide-a-fresh-cycle-to-a-new-native-change-issues-74-and-76
artifact: testing
---

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
