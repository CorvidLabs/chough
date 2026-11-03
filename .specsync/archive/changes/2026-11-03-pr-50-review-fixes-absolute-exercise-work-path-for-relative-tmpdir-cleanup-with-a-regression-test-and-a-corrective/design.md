---
change: pr-50-review-fixes-absolute-exercise-work-path-for-relative-tmpdir-cleanup-with-a-regression-test-and-a-corrective
artifact: design
---

# Design

- **Script.** Only when the script creates the directory, it runs `work=$(cd "$work" && pwd)` right after `mktemp`, before any other `cd`. A passed-in path is left as given. The work-directory setup and cleanup trap are wrapped in `# work-dir setup: begin` and `# work-dir setup: end` markers.
- **Regression test.** A unix test in `tests/cli.rs` reads the script, takes the exact source between the markers, adds `cd "$work"` and an exit status, and runs it with `sh` from a temp directory with `TMPDIR=.`. It checks four cases:
  - a passing full run removes the directory;
  - a failed full run keeps it and prints `Kept`;
  - `prepare-cold-start` keeps it;
  - a passed-in relative directory is kept.

  It also checks that the absolute path is in place: without that fix, the passing-run case fails.
- **Evidence.** The existing human-observation record is left untouched. A new `product-test` observation with method `agent-assisted` restates the original check. Its limitation names the record it corrects and says no human witnessed it. The new regression test is bound as a case, and its run adds an automated observation.
