---
change: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and
artifact: design
---

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
