# Lesson bundle: fix-339

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 339
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/records.rs, src/doctor.rs, src/cases.rs, src/migrate.rs, tests/records.rs, tests/doctor.rs, tests/cases.rs, tests/migrate.rs, docs/cli.md, .chough/cases/records.conflict-marker-follow-ups.cases.toml, changelog.d/2026-11-18-fix-339.md
- **Acceptance**: A conflict quoted inside a fenced code block in a record is not a marker, so the record saves. A lone ======= with no <<<<<<< or >>>>>>> around it is never refused; chough doctor reports it as an advisory possible_conflict_separator finding, 'possible leftover conflict separator at line N', that does not change its exit status. A committed case definition holding markers gives doctor conflict_markers findings, and cases check names the file, line and marker. The migration preview says the apply refuses and the apply refuses, each naming the 0.5.x source file first, with nothing written.

## Evidence

- Verification commit: `ed54cb4c20f3b67c4c012b3ceee957f250f5fd3b`
- Base commit: `af8ca8901b2a23affcd40ac74865d99637448346`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

GitHub #339 collects follow-ups from the tester's check of #336 (#265). Leif decided each one (2026-11-18), relayed by hc t4c as the scope approval:
1. **Code fences, exempt them:** a record that quotes an example conflict, or a lone `<<<<<<<` line of tool output, inside a fenced code block was refused.
2. **A stray `=======`** left by a half-resolved conflict, with no `<<<<<<<` or `>>>>>>>` around it, was not caught.
3. **Case TOML:** markers in a committed `.chough/cases/*.cases.toml` got no `doctor` finding, and `cases check` only said "invalid chough-testing definition".
4. **Migrate message:** the preview said nothing, and the `--apply` refusal named the 0.6 target, not the 0.5.x source that holds the markers.

The local team's attempt (chough-nemo-265) caught 2 and 3, but it refused plain heading underlines and wrote partially in migrate, so its rules are not copied.

For item 2, a heading underline and a stray separator look alike. Leif chose "Warn only" (relayed by hc t4c, 2026-11-18). A lone line of exactly seven `=` with no `<<<<<<<` or `>>>>>>>` around it is reported by `doctor` as "possible leftover conflict separator at line N", and no write is ever refused for it, since a valid heading underline may be followed directly by text. Doctor has no warning level, so the finding is marked advisory and never sets its exit status.

## From the change's design.md

# Design

- `records::markers_in(text, markdown)`, behind `conflict_markers` (records) and `conflict_markers_in_toml` (case definitions):
  - Skips the front matter, and in Markdown, every line inside a fenced code block: a backtick or tilde fence of three or more, indented at most three spaces, closed by the same mark at least as long.
  - Counts a `=======` as a marker only inside a conflict. A lone one outside a conflict and a fence is returned separately, by `lone_separators`.
- `cases::load` checks a definition file for markers before parsing it and refuses, naming the file, the line and the marker.
- `doctor::conflict_marker_findings` also reads committed `.chough/cases/*.cases.toml`, and reports each lone separator as an advisory `possible_conflict_separator` finding. `Finding::advisory` adds `"advisory": true` to its JSON and keeps it out of the exit status.
- `migrate`'s `planned_conflicts` and `conflict_refusal` find each planned record that would hold a marker, and its 0.5.x source through the plan's `becomes`.
  - The preview sets `apply_refused` to `refused:conflict-markers`, naming the source, when no open change refuses first.
  - `--apply` gives the same refusal before writing anything.

## From the change's testing.md

# Testing

Each test fails on main:
- `tests/records.rs::a_fenced_example_and_a_lone_separator_both_save`: backtick and tilde fences quoting a conflict save, and so does a lone `=======` before text.
- `tests/doctor.rs::doctor_warns_of_a_lone_conflict_separator_without_failing`: a lone `=======` in the committed session gives one advisory finding naming its line, the one inside a fence gives none, and doctor's exit status is unchanged.
- `tests/doctor.rs::doctor_reports_conflict_markers_in_committed_records_only`, updated: the committed case definition now gives three findings, while `.chough/README.md` gives none and the heading underline is not a marker.
- `tests/cases.rs::cases_check_names_a_conflict_marker_in_a_definition`.
- `tests/migrate.rs::migrate_refuses_a_plan_whose_files_would_hold_conflict_markers`, extended: the JSON and text previews name the refusal and the 0.5.x `INB-….json` source, and the apply refusal names it first.

Cases `TC-records-a-fenced-example-and-a-lone-separator-both-save` and `TC-records-doctor-warns-of-a-lone-conflict-separator` bind the first two.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
