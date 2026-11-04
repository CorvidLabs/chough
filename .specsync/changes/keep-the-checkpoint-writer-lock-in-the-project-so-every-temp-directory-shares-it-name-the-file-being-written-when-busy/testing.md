---
change: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy
artifact: testing
---

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
