---
change: fix-276-mark-a-cut-change-ship-summary-with-its-line-count-and-json
artifact: context
---

# Context

GitHub issue #276: `chough change ship` prints SpecSync's output and then a short
text summary of at most 12 lines (`SUMMARY_LINES` in `src/change.rs`). When the
summary is longer than 12 lines it was cut at 12 lines with no marker, so an agent
could not tell that the report continued. The expected behaviour is that the cut
summary's last line says so and names `--json`, for example
`… N more lines; run with --json for the full report`.

How it was noticed: the summary is built by `print_summary`, which used
`lines.iter().take(SUMMARY_LINES)` and wrote nothing about the dropped lines.
`tests/gates.rs` already marks a similar cut with `… (cut)` (`src/gates.rs`,
`SUMMARY_LINES = 24`), so the same convention was applied here.

Constraints and prior attempts:

- The real ship fixture in `tests/change.rs`
  (`ship_prints_a_short_summary_after_specsync_and_every_report_under_json`) is
  only 9 lines long, so it never reaches the cut; the cut path is covered by a
  unit test of the line-building function instead.
- This worktree has no open Chough session and no human authority, so no
  approval, review or ship gate could be recorded here. Nothing was invented.
- `main` was already an ancestor of `HEAD`; no merge was needed.