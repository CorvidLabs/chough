---
change: chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17
artifact: testing
---

# Testing

New tests in tests/doctor.rs. A `specsync` stub on PATH records any call and fails, so each test also asserts there was no native read.
- `doctor_reports_debt_and_changes_nothing`:
  - setup: alpha is finished (native change archived, at intent), beta is at done, gamma is open, and the pointer names `ghost`;
  - with no flag and with `--dry-run`, the doctor reports `finished_session` alpha and beta and `stale_pointer` ghost, each `found` with a `chough` fix and not a decision;
  - it exits 1, and every byte under `.chough/` is unchanged.
- `doctor_fix_archives_finished_sessions_and_removes_a_stale_pointer`:
  - `--fix` archives alpha and beta byte for byte, removes the pointer and leaves gamma;
  - `checkpoint` then lands in gamma;
  - a second run reports no findings and exits 0.
- `doctor_fix_adopts_a_legacy_checkpoint`:
  - a legacy checkpoint with native change `ch-legacy` is adopted as `ch-legacy` with a history cursor;
  - one without a native change is skipped, naming `--adopt-as`, and is unchanged; `--adopt-as legacy` adopts it.
- `doctor_decision_fixes_need_a_reason_and_record_no_human_decision`:
  - `--fix --close gamma` skips `stale_gates` alpha and `unfinished_session` gamma, both decisions naming `--reason`, with the files unchanged;
  - with `--reason`, alpha's gate is withdrawn with no decision recorded and alpha is archived;
  - gamma is closed with the reason, and delta stays open.
- `doctor_fix_keeps_a_finished_session_with_unfolded_entries_open`: the finding is skipped, names `chough inbox fold`, and the file is unchanged.

All five fail first: `doctor` is an unrecognized subcommand. Cases bind the tests. `fledge lanes run verify` passes on Linux.
