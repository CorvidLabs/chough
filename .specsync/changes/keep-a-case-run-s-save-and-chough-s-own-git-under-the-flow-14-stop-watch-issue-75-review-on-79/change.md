---
id: keep-a-case-run-s-save-and-chough-s-own-git-under-the-flow-14-stop-watch-issue-75-review-on-79
state: implementing
type: bug_fix
base_commit: c22bf0c2e6a6cb60e1453b7adb83414c3f930d6c
---

# Keep a case run's save and Chough's own Git under the FLOW-14 stop watch (issue 75, review on #79)

## Intent

Keep a case run's save and Chough's own Git under the FLOW-14 stop watch (issue 75, review on #79)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- stopping_a_case_run_while_it_saves_stops_git_and_records_nothing passes: a stop while cases run takes its Git snapshot after the command exits stops that Git and leaves .chough byte-identical,Every other cases and evidence test passes unchanged

## No-spec Rationale

Call-site wiring for REQ-cli-011 as widened by stop-what-chough-started-when-chough-is-stopped-issue-75: cases.rs holds the stop watch through the save and evidence.rs runs Git through records::run_owned. The requirement text lives in that change.
