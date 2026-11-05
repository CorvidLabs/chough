# Lesson bundle: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep the checkpoint writer lock in the project so every temp directory shares it, name the file being written when busy, and specify serialized writers and the local state directory (issue 83)
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/checkpoint.rs, tests/cli.rs, tests/common/mod.rs, tests/sessions.rs, tests/evidence.rs, examples/feature-bug-loop.sh, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: A writer with a different TMPDIR waits for the project lock and then saves. A busy refusal in sessions mode names the session file and saves nothing. Legacy writers racing session adopt either save in the remaining checkpoint or refuse naming the adoption with nothing saved. The lock lives in .chough/local which Git ignores and which in a project without sessions holds only the lock and its gitignore. Each new test fails where stated and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `59519b6b661c72980f4f9bcc7fbb61723bff610a`
- Base commit: `d9269bec6ea0bc6c9e6c9f37bb9a6f6a05cd5024`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue https://github.com/CorvidLabs/chough/issues/83 collects follow-ups from the Chough
bot approval of #77 (this branch is stacked on #77 at 75a732d; it retargets to main when
#77 merges):

- a. The checkpoint writer lock from #70 lives in the system temp directory, keyed on
  the canonical `.chough` path, so processes with different `TMPDIR` values, or a temp
  cleaner deleting it while held, each get their own lock; a reviewer's repro kept 11 of
  20 concurrent updates.
- b. When busy, the refusal names `.chough/session.json` even in sessions mode, where
  the write goes to `.chough/sessions/<key>.json`.
- c. The serialized read-modify-write and the legacy writer's refusal after `session
  adopt` are not literally specified; the wording had to come from Leif through HI.
- d. `legacy_writers_racing_adopt_end_in_one_mode` asserts only that one mode remains.

Leif chose on 2026-11-04, from offered options, to keep the lock in `.chough/local/`
(Git-ignored, one per worktree) and confirmed the wording captured as DRIVE-11. The
Git common directory and keeping the temp directory were offered and not chosen.

During the first build, two #77 tests failed: `a_project_without_sessions_is_unchanged`
asserts `.chough/local/` never appears in a project without sessions, and an evidence
retention snapshot spans a `consent grant`. Leif chose on 2026-11-04 ("Lock everywhere,
relax #77") to keep the lock in `.chough/local/` in every project: #77's test checks no
tracked or session file changes instead, and a new REQ-history-007 states what
`.chough/local/` holds. Sessions-only and the Git directory were offered and not chosen.
So this change covers the cli and history modules.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes, his brief to
fix issues 81 to 88 one PR each, and his #83 choice.

## From the change's design.md

# Design

- src/checkpoint.rs: `checkpoint_lock_path` is `.chough/local/checkpoint.lock`; taking
  the lock first creates `.chough/local/` and its `.gitignore` of `*` when missing
  (create-only, the same file #77's active pointer uses). No temp-directory path or
  digest remains. The busy refusal names the file the command would write, resolved with
  `sessions::checkpoint_path` (`.chough/session.json` or the active session's file; the
  command refuses and exits right after, so resolving it there is harmless), or "the
  checkpoint" when it cannot be resolved. src/sessions.rs (history module) is unchanged.
- tests/common/mod.rs: `checkpoint_lock_path` follows the new location.
- examples/feature-bug-loop.sh: the exit cleanup of the temp-directory lock goes; the
  lock now lives inside the run's project.
- tests/sessions.rs `a_project_without_sessions_is_unchanged` (#77) now asserts no
  `.chough/sessions/`, that `.chough/local/` holds only `.gitignore` and `checkpoint.lock`,
  and that Git ignores them; tests/evidence.rs's retention snapshot leaves out the lock
  files the `consent grant` writer creates, as it already leaves out the session file.
- tests: different-TMPDIR writers serialize (fails first); the busy refusal names the
  session file in sessions mode (unit, 300 ms test wait); a new adopt-race test in
  tests/cli.rs asserts each writer either saved its decision in the one remaining
  checkpoint or refused with the adoption message and nothing saved (tests/sessions.rs
  belongs to the history module and is unchanged).

## From the change's testing.md

# Testing

- tests/cli.rs `writers_with_different_temp_directories_share_the_checkpoint_lock`: a
  holder process holds the project's lock; a `checkpoint --decision` run with another
  `TMPDIR` is still waiting after 1 s, and after the holder ends it saves its decision.
- src/checkpoint.rs unit test `a_busy_refusal_names_the_session_file_being_written`:
  with the lock held, a writer in sessions mode refuses as `refused:checkpoint-busy`
  naming `.chough/sessions/alpha.json`, not `.chough/session.json`.
- tests/cli.rs `legacy_writers_racing_adopt_each_save_or_refuse_with_nothing_saved`: four
  legacy writers race `session adopt`; each either exited 0 with its decision in the one
  remaining checkpoint or refused naming the adoption with its decision absent, and the
  decisions present match the writers that succeeded. #77's
  `legacy_writers_racing_adopt_end_in_one_mode` in tests/sessions.rs is unchanged.
- tests/cli.rs: the lock file is ignored by Git.
- tests/sessions.rs `a_project_without_sessions_is_unchanged` (#77): no `.chough/sessions/`;
  `.chough/local/` holds only `.gitignore` and `checkpoint.lock`, both ignored by Git.
- tests/evidence.rs `native_manifests_protect_their_sets_and_a_failed_read_protects_everything`:
  its before/after snapshot leaves out the lock files the `consent grant` writer creates.
- Each new or strengthened test fails at 75a732d where stated; `fledge lanes run verify`
  passes on Linux.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-040 | tests/cli.rs `writers_with_different_temp_directories_share_the_checkpoint_lock`, `the_checkpoint_lock_lives_in_the_project_and_git_ignores_it`, `legacy_writers_racing_adopt_each_save_or_refuse_with_nothing_saved`; src/checkpoint.rs `a_busy_refusal_names_the_session_file_being_written`, `a_held_checkpoint_lock_refuses_as_busy_after_the_wait` |
| REQ-history-007 | tests/sessions.rs `a_project_without_sessions_is_unchanged`, `in_sessions_mode_local_holds_only_the_pointer_and_the_lock`; tests/cli.rs `the_checkpoint_lock_lives_in_the_project_and_git_ignores_it` |

## Where these lessons go

- `specs/cli/context.md`
- `specs/history/context.md`
