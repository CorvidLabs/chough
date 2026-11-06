# Lesson bundle: chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Chough doctor finds and fixes the Chough debt that stops a fresh worktree from entering Chough (DRIVE-17)
- **Kind**: Feature
- **Specs**: cli, history
- **Paths**: src/main.rs, src/doctor.rs, src/sessions.rs, src/checkpoint.rs, src/history.rs, tests/doctor.rs, specs/cli/cli.spec.md, hi/drive.md, docs/cli.md, skills/chough-workflow/references/checkpoint.md, .agents/skills/chough-workflow/references/checkpoint.md, CHANGELOG.md
- **Acceptance**: DRIVE-17 (Leif, relayed by orc 2026-11-06, 'Capture + both defaults'), in the new REQ-cli-055 and REQ-history-012. chough doctor reports legacy_checkpoint, stale_pointer, finished_session, stale_gates and unfinished_session findings without a native read. No flag and --dry-run report only and change nothing. --fix applies the safe fixes: adopt the legacy checkpoint, remove a stale pointer, archive finished sessions byte for byte (a session with unfolded inbox entries is skipped). Decision fixes (withdraw stale gates, close sessions named by --close) need --reason; without it they are skipped. No human decision is ever recorded. Exit 0 only when nothing is left. The five new tests in tests/doctor.rs fail first; docs, the skill reference and CHANGELOG say so; fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `47874558f954d8d49ec70b80ec17358a558a1b2c`
- Base commit: `1030252cb5ca2e2b45b736833013f8cd5dbabab0`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Leif asked for a `chough doctor` on 2026-11-06, through the orc. It is one command that finds and fixes repo debt, the debt spec-sync #941 cleaned up by hand: twelve leftover sessions on main, a legacy `session.json` that needed adopting, and stale approval gates on archived changes. A fresh worktree could not enter Chough until that was done. He captured DRIVE-17 with "Capture + both defaults": with no flag the doctor only reports, and `--fix` without `--reason` applies only the safe fixes. #163's finished rule and `session close` (#166) are reused here, and #166 is not held for this change.

## From the change's design.md

# Design

## Approach

- **`src/doctor.rs`** collects the findings in the REQ-history-012 order. Each finding records its kind, target, fix command and decision flag, plus an apply function.
- **Report and dry run:** report mode and `--dry-run` print the list and touch nothing. They take no lock and write no `.gitignore`.
- **`--fix`** takes the project lock once (as the session commands do), recomputes the findings under the lock, and applies them in order:
  1. **Legacy checkpoint:** run history adoption when the checkpoint has no cursor, then the session adoption. The key is the native change if valid, else `--adopt-as`; otherwise the finding is skipped.
  2. **Stale pointer:** remove the pointer file.
  3. **Stale gates,** only with `--reason`: pin to that session and clear `awaiting_human` through the checkpoint save path, so history records a gate withdrawal and no decision. Then archive it.
  4. **Finished sessions:** the same no-replace rename as `archive_done`, with the unfolded-inbox rule.
  5. **`--close <key>`,** only with `--reason`: #166's `close`.
- **Exit code:** the command exits 1 if any finding is still `found` or `skipped`.
- **No native read:** findings come from `.chough/` files and `.specsync/archive/changes/*/finalization.json`, through `library::finalized_at`.

- **Shared code, not copies.** `history::adopt` and `checkpoint::withdraw_gates` are extracted from `history adopt --apply` and `checkpoint --clear-awaiting-human`. Those commands keep their exact output and behavior. The doctor calls the same functions, so the gate-closed events and the history cursor are written one way.

## Risks

- **Ordering:** adopting the legacy checkpoint makes it the active session, so its pointer is valid. If its change is archived, step 4 archives it and removes the pointer in the same run.
- **Gate withdrawal on a session that is not active** needs the checkpoint save path for another file. Pinning per command already supports one resolved session per run, and the doctor saves at most one session at a time under the lock.
- **Builds on #166.** This branch starts from #166's head and merges main after #166 lands.

## Constraints consulted

DRIVE-17, DRIVE-16, REQ-history-006, REQ-cli-037, REQ-cli-055 (new), REQ-history-012 (new).

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
