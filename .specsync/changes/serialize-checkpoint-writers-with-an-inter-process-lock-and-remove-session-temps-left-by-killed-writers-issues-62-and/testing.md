---
change: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and
artifact: testing
---

# Testing

- tests/cli.rs `checkpoint_writers_in_parallel_keep_every_update`: 8 rounds of three
  concurrent `chough checkpoint` processes, each adding a decision, observation and bug
  link and setting a native change. Every process succeeds with `saved: true`, every
  value is in the final checkpoint, `native_change` is one of the three, no temp remains.
- `checkpoint_writer_killed_before_rename_leaves_a_temp_the_next_writer_removes`: a
  helper process (this test binary re-run with `CHOUGH_TEST_LOCK_HOLDER`) takes the
  lock, writes its temp and waits. The next `chough checkpoint` is still waiting after
  700 ms with the temp and session untouched. The helper is killed (SIGKILL on Unix,
  TerminateProcess on Windows). The waiting writer then succeeds, names the removed temp
  on stderr, and the session holds its decision but not the helper's.
- `checkpoint_writer_killed_after_rename_keeps_its_saved_update`: the helper renames
  its write in and is killed. The next writer removes nothing and keeps both decisions.
- `checkpoint_writer_removes_only_orphaned_session_temps`: `status` leaves an orphan.
  `checkpoint` removes `.session.json.4194304.tmp` and keeps `.session.json.notapid.tmp`
  and `.preferences.json.4194304.tmp`.
- Checked against the old behavior: with src/ at main 4daf861, the parallel,
  killed-before-rename and orphan tests fail ("was lost", "did not wait", temp kept),
  and the after-rename control passes.
- A unit test in src/checkpoint.rs: with the lock held by another handle, taking it
  refuses as `refused:checkpoint-busy` after the (test-shortened) wait, a second call
  in the same process is a no-op, and nothing is added to `.chough/`.
- Existing contracts unchanged and passing: tests/package.rs (init leaves only
  session.json), REQ-cli-032 refusal tests (no change under `.chough/`), and
  tests/optional.rs repeated-run halt tests.
- Manual: the issue repros (six concurrent writers; a SIGKILL timing loop) against a
  release build in an examples/feature-bug-loop.sh sample.
- No lock files left behind: the shared test Fixture (tests/common/mod.rs) and the
  tests/cli.rs Fixture remove their project's lock file when dropped, and the unit test
  removes its own. A full `cargo test --locked` run with TMPDIR set to an empty
  directory leaves that directory empty (before this, about 360 lock files per run).
- `cargo test --locked` and `fledge lanes run verify` pass.
