---
id: keep-stage-backfill-s-git-and-library-snapshot-s-save-under-the-flow-14-stop-watch-issue-75-review-5762745045-on-79
state: implementing
type: bug_fix
base_commit: a2e3bc073970d75e2399f217c9efb122f2c06126
---

# Keep stage backfill's Git and library snapshot's save under the FLOW-14 stop watch (issue 75, review 5762745045 on #79)

## Intent

Keep stage backfill's Git and library snapshot's save under the FLOW-14 stop watch (issue 75, review 5762745045 on #79)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- stopping_a_backfill_stops_its_git_and_records_nothing passes: a stop while stage backfill checks its commit stops that Git and leaves .chough byte-identical,stopping_a_library_snapshot_stops_git_and_records_nothing passes: a stop while library snapshot reads the HEAD stops that Git and leaves .chough and the library byte-identical,Every other stage and library test passes

## No-spec Rationale

Call-site wiring for REQ-cli-011 as stated in stop-what-chough-started-when-chough-is-stopped-issue-75: stage.rs runs backfill's Git through records::run_owned, and library.rs reads the HEAD under the stop watch before staging a snapshot under a hidden name and renaming it whole. The requirement text lives in that change.
