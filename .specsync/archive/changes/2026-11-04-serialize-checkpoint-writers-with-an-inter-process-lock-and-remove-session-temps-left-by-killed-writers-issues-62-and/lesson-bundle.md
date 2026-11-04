# Lesson bundle: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Serialize checkpoint writers with an inter-process lock and remove session temps left by killed writers (issues 62 and 64)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/checkpoint.rs, src/validate.rs, Cargo.toml, Cargo.lock, tests/cli.rs, tests/common/mod.rs, docs/cli.md, CHANGELOG.md, examples/feature-bug-loop.sh
- **Acceptance**: Concurrent chough checkpoint writers in one clone keep every update: in 8 rounds of three parallel writers every process succeeds and every decision observation and bug link is in the final checkpoint; a writer that cannot take the checkpoint lock within 30 seconds refuses with refused:checkpoint-busy and saves nothing; a session temp left by a writer killed before its rename is removed by the next checkpoint writer with a stderr notice naming it and the session keeps only saved updates while a writer killed after its rename keeps its update and nothing is removed; only regular files named .session.json.<digits>.tmp directly in .chough are removed and read-only commands remove nothing; init still leaves only .chough/session.json and refused writers still change nothing under .chough; the new tests fail with src at main 4daf861 and docs/cli.md describes the lock and cleanup and fledge lanes run verify passes; the test suite leaves no chough-checkpoint lock files in the system temporary directory

## Evidence

- Verification commit: `85caeb2de741b185f599e8c9ba3cf664d70c8b93`
- Base commit: `4daf861509b70c14282ac16b5d0547d701e385b2`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Kimi's second lab review (2026-11-03, Chough 7bd1b62, round 2 items 1 and 3):

- Three `chough checkpoint` lanes run at once in one clone all exited 0 with success,
  but only the last lane's decision and native change survived. Reproduced on main
  4daf861 with six writers: one decision of six survived. Filed as
  https://github.com/CorvidLabs/chough/issues/62.
- A `chough checkpoint` killed with SIGKILL partway through left
  `.chough/.session.json.<pid>.tmp` (valid JSON). The session itself stayed valid, but
  nothing ever removed the temp. Reproduced on 4daf861 at a 250 microsecond kill delay;
  the temp survived later `status` and `checkpoint` calls. Filed as
  https://github.com/CorvidLabs/chough/issues/64.

Cause: `write_replace_bounded` writes a create-new temp, syncs it and renames it, so
each write is atomic, but every command reads the checkpoint, changes it in memory and
replaces it with nothing serializing the read-modify-write, so the last rename wins.
The temp is removed on an error return but not when the process is killed.

The two fixes are one change because the cleanup is only safe under the lock: every
session writer holds the lock through its rename, so a session temp found while holding
the lock belongs to a writer that is gone. Related but separate: #30 (record identity
across parallel agents) and #63 (unknown --native-change), handled on its own branch.

## From the change's design.md

# Design

1. `src/checkpoint.rs` `lock_checkpoint(root)`: open (creating if needed) the
   project's lock file, `<system temp dir>/chough-checkpoint-<first 16 bytes of
   SHA-256 of the canonical .chough path, hex>.lock` (outside the project, so it is never
   a managed record or a source change; a read-only handle is used if another account
   owns the file), take an exclusive `fs4` lock with
   `try_lock` polled every 25 ms for up to 30 seconds, and keep the handle in a
   process-wide table keyed by path, so the lock is held until the process exits and
   repeated calls in one process are no-ops. On timeout: `refused:checkpoint-busy:
   another Chough command is changing .chough/session.json and did not finish within 30
   seconds; nothing was saved. Rerun when it finishes.`
2. `read_checkpoint` takes the lock before reading when the session file exists (an
   uninitialized project gets no `.chough/` created by a failed read). `save_checkpoint`
   and `migrate` take it too. `init` only creates files that do not exist and takes no
   lock, so it still leaves exactly `.chough/session.json`. Read-only reports
   (`read_session`: `status`, `next`, `cases`, `evidence`, `timing`) take no lock, and
   `src/validate.rs` reads through a new `read_checkpoint_unlocked`, because a repeated
   validation run only reads the checkpoint (pause checks between attempts) and can run
   for a long time; holding the lock there would block every writer for the whole run.
3. Right after taking the lock, `remove_orphaned_session_temps` scans `.chough/` (not
   recursively) for regular files named exactly `.session.json.<ASCII digits>.tmp`,
   removes each and prints one stderr line naming it, its size, and that the session
   was not changed by it. Symlinks, directories and other names are left alone.
4. `Cargo.toml`/`Cargo.lock`: `fs4 = { version = "1.1.0", default-features = false,
   features = ["sync"] }`.
5. `src/validate.rs`: its two checkpoint reads use `read_checkpoint_unlocked`.
6. `docs/cli.md` and `CHANGELOG.md` describe the lock, the busy refusal and the cleanup.

Not changed: record formats, JSON outputs, the merge driver's own temp files, other
managed files' temps, and identity allocation (#30).

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
