---
change: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy
artifact: design
---

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
