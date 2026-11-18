---
change: fix-339
artifact: testing
---

# Testing

Each test fails on main:
- `tests/records.rs::a_fenced_example_and_a_lone_separator_both_save`: backtick and tilde fences quoting a conflict save, and so does a lone `=======` before text.
- `tests/doctor.rs::doctor_warns_of_a_lone_conflict_separator_without_failing`: a lone `=======` in the committed session gives one advisory finding naming its line, the one inside a fence gives none, and doctor's exit status is unchanged.
- `tests/doctor.rs::doctor_reports_conflict_markers_in_committed_records_only`, updated: the committed case definition now gives three findings, while `.chough/README.md` gives none and the heading underline is not a marker.
- `tests/cases.rs::cases_check_names_a_conflict_marker_in_a_definition`.
- `tests/migrate.rs::migrate_refuses_a_plan_whose_files_would_hold_conflict_markers`, extended: the JSON and text previews name the refusal and the 0.5.x `INB-….json` source, and the apply refusal names it first.

Cases `TC-records-a-fenced-example-and-a-lone-separator-both-save` and `TC-records-doctor-warns-of-a-lone-conflict-separator` bind the first two.
