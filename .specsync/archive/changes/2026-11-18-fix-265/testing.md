---
change: fix-265
artifact: testing
---

# Testing

Each test fails on main:
- `tests/records.rs::a_record_body_with_conflict_markers_is_refused_and_unchanged`: a session whose body holds a conflict block refuses `checkpoint --decision` as `refused:conflict-markers`, naming the file, the line and `<<<<<<< HEAD`, with every file unchanged.
- `tests/records.rs::a_heading_underline_is_not_a_conflict_marker`: a `=======` heading underline in a session's body is written and saved. This passes on main too, and guards the narrower rule.
- `tests/doctor.rs::doctor_reports_conflict_markers_in_committed_records_only`: a committed story with a conflict block gives three `conflict_markers` findings with lines and markers, and `--fix` reports them `skipped`. A committed case definition and README holding marker lines give none.
- `tests/migrate.rs::migrate_refuses_a_plan_whose_files_would_hold_conflict_markers`: a 0.5.x inbox entry whose text holds a conflict block makes `migrate --apply` refuse as `refused:conflict-markers` with nothing written. Its text becomes the 0.6 entry's body as written; the bug, session and objective fields are flattened to one line, so they cannot carry a marker line.

Cases `TC-records-a-conflict-marker-write-is-refused`, `TC-records-doctor-reports-conflict-markers-in-records` and `TC-records-migrate-refuses-conflict-markers` each bind one test, so their expectations don't overlap.
