---
change: keep-stage-backfill-s-git-and-library-snapshot-s-save-under-the-flow-14-stop-watch-issue-75-review-5762745045-on-79
artifact: testing
---

# Testing

- `tests/stop.rs` `stopping_a_backfill_stops_its_git_and_records_nothing` fails before the fix (the Git stand-in survives) and passes now.
- `tests/stop.rs` `stopping_a_library_snapshot_stops_git_and_records_nothing` fails before the fix (a copied file is left with no manifest) and passes now.
- The library unit test `an_interrupted_snapshot_has_no_completion_manifest` now asserts that a failed snapshot leaves nothing under `snapshots/`, neither the copy nor a staging directory.
- `cargo test --locked` and `fledge lanes run verify` pass.
