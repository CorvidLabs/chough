---
change: keep-stage-backfill-s-git-and-library-snapshot-s-save-under-the-flow-14-stop-watch-issue-75-review-5762745045-on-79
artifact: tasks
---

# Tasks

- [x] src/stage.rs: `git_output`, used by `stage backfill` to resolve its commit and check ancestry, runs Git through `records::run_owned`.
- [x] src/library.rs: the `library snapshot` handler holds `records::stop_watch()`; `write_snapshot` reads the HEAD first, calls `records::stop_if_noted()`, writes the copies and the manifest into a hidden `snapshots/.staging-<id>` directory and renames it into place, removing the staging directory on any error.
