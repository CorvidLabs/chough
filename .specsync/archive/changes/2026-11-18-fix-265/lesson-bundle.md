# Lesson bundle: fix-265

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 265
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/records.rs, src/doctor.rs, src/migrate.rs, tests/records.rs, tests/doctor.rs, tests/migrate.rs, docs/cli.md, .chough/cases/records.conflict-markers.cases.toml, changelog.d/2026-11-18-fix-265.md
- **Acceptance**: A record write whose body holds a Git conflict marker, a line starting <<<<<<<, a line starting >>>>>>>, or a ======= line between them, is refused as refused:conflict-markers naming the file, the line and the marker, with every file unchanged; a ======= line outside such a block, as a Markdown heading underline, is not a marker. chough doctor reports each committed record (not other tracked files) whose body holds one as a conflict_markers finding naming the file, the line and the marker, with no fix, and exits 1. chough migrate checks every file it would write before writing any and refuses, writing nothing, when one would hold a conflict marker.

## Evidence

- Verification commit: `343ca8c4181f37f6b6668f57e1d820c46a568e81`
- Base commit: `0b7ac77e7acf916d40a464035b1fa22c0b503d29`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

GitHub #265, from the 0.6.0 swarm report (s4). Conflict markers in a record's body (`<<<<<<<`, `=======`, `>>>>>>>`) are accepted and committed silently: every reader and `chough doctor` treat a half-merged record as healthy.

Leif approved the scope on 2026-11-18, relayed by hc t4c ("Approve the scope"):
- a record body may not hold git conflict markers;
- writes are refused naming the file, line and marker;
- `chough doctor` reports committed records that hold them;
- a new sentence in REQ-records-001.

This change is new, on main. It builds on two earlier attempts, read but not reused as records:
- the local team's in `chough-nemo-265` (018a059, d9012c6, 74defb4): correct code and tests, but wrong records, per Leif's decision on chough#220;
- an older fix on branch `agent/265-conflict-markers` in `chough-265`.

Ripple's minor findings on the local team's attempt are addressed:
- the doctor scan is limited to records;
- the cases' expectations don't overlap;
- `src/migrate.rs` is declared, with its guard tested.

Two choices go beyond those attempts:
- A `=======` line counts only between a `<<<<<<<` and a `>>>>>>>` line, so a Markdown setext heading underline is not refused. Both earlier attempts refused any line of seven `=`.
- The migration checks every file it would write before writing any, so its refusal writes nothing.

`src/records.rs` and `src/doctor.rs` belong to `cli`, so REQ-cli-055 (the doctor) names the new finding beside REQ-history-012's.

## From the change's design.md

# Design

- `records::conflict_marker(text)` returns the first conflict line of a record's body, with its 1-based file line and its text. The body is what follows the front matter's closing `---`.
  - A line starting `<<<<<<<` or `>>>>>>>` is a marker.
  - A line of `=======` is one only while a `<<<<<<<` line is open.
- `records::write_new_bounded` and `write_replace_bounded`, through which every record write passes, refuse a record, a Markdown file under `.chough/` or a journal post, whose body holds one (a generated knowledge note or a skill file may quote a marker), as `refused:conflict-markers`, before anything is written.
- `chough migrate --apply` checks every planned text file the same way before it writes any.
- `chough doctor` adds `conflict_markers` findings for committed records only, the files `bound_of` classifies as records. Each finding gives the file, the line and the marker. There is no fix, and `--fix` reports it `skipped`.

## From the change's testing.md

# Testing

Each test fails on main:
- `tests/records.rs::a_record_body_with_conflict_markers_is_refused_and_unchanged`: a session whose body holds a conflict block refuses `checkpoint --decision` as `refused:conflict-markers`, naming the file, the line and `<<<<<<< HEAD`, with every file unchanged.
- `tests/records.rs::a_heading_underline_is_not_a_conflict_marker`: a `=======` heading underline in a session's body is written and saved. This passes on main too, and guards the narrower rule.
- `tests/doctor.rs::doctor_reports_conflict_markers_in_committed_records_only`: a committed story with a conflict block gives three `conflict_markers` findings with lines and markers, and `--fix` reports them `skipped`. A committed case definition and README holding marker lines give none.
- `tests/migrate.rs::migrate_refuses_a_plan_whose_files_would_hold_conflict_markers`: a 0.5.x inbox entry whose text holds a conflict block makes `migrate --apply` refuse as `refused:conflict-markers` with nothing written. Its text becomes the 0.6 entry's body as written; the bug, session and objective fields are flattened to one line, so they cannot carry a marker line.

Cases `TC-records-a-conflict-marker-write-is-refused`, `TC-records-doctor-reports-conflict-markers-in-records` and `TC-records-migrate-refuses-conflict-markers` each bind one test, so their expectations don't overlap.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
