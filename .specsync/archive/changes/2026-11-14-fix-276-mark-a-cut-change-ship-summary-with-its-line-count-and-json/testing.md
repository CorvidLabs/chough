---
change: fix-276-mark-a-cut-change-ship-summary-with-its-line-count-and-json
artifact: testing
---

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