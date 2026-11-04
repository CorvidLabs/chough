---
change: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and
artifact: research
---

# Research

- std `File::lock`/`try_lock` were stabilized in Rust 1.89. Chough declares
  `rust-version = "1.88"` (D-019, checked by the `MSRV / 1.88.0` CI job), so std locks
  would need an MSRV change, which is a human decision. Not taken.
- `fs4` 1.1.0 (MIT OR Apache-2.0, rust-version 1.75) provides `FileExt::lock`/`try_lock`
  over flock on Unix and LockFileEx on Windows. Its only dependencies, `rustix` (Unix)
  and `windows-sys` (Windows), are already in Chough's Cargo.lock, so the lock adds one
  package. The OS releases the lock when the process exits or is killed, so a killed
  writer can never leave a stale lock.
- Locking `session.json` itself does not work: the rename replaces the locked inode, so
  a waiter can hold a lock on the old file while a newcomer locks the new one. The lock
  is a separate, never-replaced file. It cannot live in `.chough/`: `init` must leave
  only `.chough/session.json` (tests/package.rs) and refused writers must change nothing
  under `.chough/` (REQ-cli-032 tests/cli.rs), and a lock file created on first use
  breaks both. A project-root file would dirty evidence. It lives in the system temp
  directory, named for the SHA-256 of the canonical `.chough` path. Processes with
  different temp directories (different `TMPDIR`) do not share it; documented.
- Every write of `.chough/session.json` goes through `checkpoint::save_checkpoint`
  (callers: checkpoint, stage, prefs, history; init and migrate). Every
  read-modify-write starts with `checkpoint::read_checkpoint`. Read-only reports use
  `read_session`.
- `validate run` reads the checkpoint with `read_checkpoint` between attempts but never
  writes it; locking there would hold the lock for the whole repeated run
  (tests/optional.rs `repeated_runs_halt_promptly...` caught this).
