---
change: fix-247-the-verify-lane-s-example-waits-past-the-failing-run-s-second-before-its-rerun
artifact: tasks
---

# Tasks

- [x] A failing test in `tests/cli.rs` that runs the script's own rerun-and-verify lines with the failing run's recorded second ahead of the clock, so `bug update --status verified` refuses as in run 43357893936.
- [x] The fix in `examples/feature-bug-loop.sh`: before the rerun, wait until the clock is past the failure's recorded second, read from `chough next --json`; one line in `docs/exercises/feature-bug-loop.md` and an Unreleased CHANGELOG line.
- [x] Red then green recorded through Chough, and the full local suite.

Human gates and delivery, as plain text: Leif's review through `chough change ship`, then CI, then his merge.
