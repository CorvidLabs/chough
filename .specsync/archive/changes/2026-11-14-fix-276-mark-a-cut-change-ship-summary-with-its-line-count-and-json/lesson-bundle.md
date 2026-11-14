# Lesson bundle: fix-276-mark-a-cut-change-ship-summary-with-its-line-count-and-json

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix #276: mark a cut change ship summary with its line count and --json
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/change.rs, tests/change.rs, .chough/cases/cli.change-ship.cases.toml, CHANGELOG.md, docs/cli.md, specs/cli/requirements.md
- **Acceptance**: A summary longer than 12 lines ends with a line naming the dropped lines and --json

## Evidence

- Verification commit: `084e606e74ef2863351d7117dd6f69972ffc3776`
- Base commit: `ddc56e96ff77cb0313d7591bee61d5a4a2950f0e`
- Verified by: `specsync check --spec cli`

## From the change's context.md

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

## From the change's testing.md

# Testing

`src/change.rs` unit test
`change::tests::the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines`:
a 20-line summary comes back as exactly 12 lines, the last line is exactly
`… 9 more lines; run with --json for the full report`, the first 11 lines are the
original first 11 in order, and a 12-line summary is returned unchanged.

The existing integration test
`ship_prints_a_short_summary_after_specsync_and_every_report_under_json` in
`tests/change.rs` covers the uncut path (its fixture summary is 9 lines) and the
`--json` report.

Case `TC-cli-change-ship-cut-summary-names-the-cut-and-json` in
`.chough/cases/cli.change-ship.cases.toml` runs the unit test by exact name and
expects `test result: ok. 1 passed`.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-059 | the unit test above, plus the updated acceptance criterion and the case definition |

## Where these lessons go

- `specs/cli/cli.3md`
