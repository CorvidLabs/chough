---
change: keep-a-case-run-s-save-and-chough-s-own-git-under-the-flow-14-stop-watch-issue-75-review-on-79
artifact: tasks
---

# Tasks

- [x] src/cases.rs: `run_case` holds `records::stop_watch()` from before the binding runs until the observation is saved; it takes the snapshot, calls `records::stop_if_noted()`, then writes the artifact and the observation.
- [x] src/evidence.rs: `git`, `is_ancestor_of_head` and `blob_of` run Git through `records::run_owned`.
