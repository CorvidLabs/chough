---
change: fix-276-mark-a-cut-change-ship-summary-with-its-line-count-and-json
artifact: plan
---

# Plan

## Approach

Add `short_summary(lines: &[String]) -> String` next to `print_summary` in
`src/change.rs`. It returns every line, joined, when the summary is at most
`SUMMARY_LINES` (12) lines; otherwise it keeps `SUMMARY_LINES - 1` lines and
appends `… N more lines; run with --json for the full report`, where `N` is the
number of dropped lines. `print_summary` writes the returned text line by line, so the cut
summary's last line names the cut and points at `--json`, which already prints
every step's full report.

## Out of scope

Nothing else about `chough change ship` changes: the `--json` report, the ship
itself, the gate close and the move into done are untouched, and the summary is
still at most 12 lines.

## Steps

1. Add `short_summary` and use it in `print_summary` (`src/change.rs`).
2. Add the unit test
   `change::tests::the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines`.
3. Add the case `TC-cli-change-ship-cut-summary-names-the-cut-and-json` to
   `.chough/cases/cli.change-ship.cases.toml`.
4. Update `specs/cli/requirements.md` (REQ-cli-059), `docs/cli.md` and
   `CHANGELOG.md`.
5. Run `fledge run fmt`, `fledge run test`, `fledge run lint` and the case check.

## Risks

The cut line's wording must stay stable: the test asserts the exact string
`… 9 more lines; run with --json for the full report` for a 20-line summary, and
the first 11 lines must be preserved in order. A summary of exactly 12 lines is
not cut.

## Constraints consulted

- AGENTS.md: keep changes small, preserve existing files, add a CHANGELOG entry,
  run fmt/test/lint, commit on the branch without pushing or opening a pull
  request.
- `src/gates.rs`'s existing `… (cut)` marker was the precedent for the wording.
- No human decision was recorded: this worktree has no open session, and an
  approval or review in a human's name may only come from that human's decision.