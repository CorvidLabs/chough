---
change: keep-the-checkpoint-writer-lock-in-the-project-so-every-temp-directory-shares-it-name-the-file-being-written-when-busy
artifact: research
---

# Research

- Where the lock lives today (#70): `std::env::temp_dir()/chough-checkpoint-<digest>.lock`,
  the digest taken over the canonical `.chough` path. `TMPDIR` changes `temp_dir()`, so two
  processes on one project can hold different files. The #77 approval review reproduced
  lost updates (11 of 20 kept) with half the writers on another `TMPDIR`.
- Options Leif was offered on 2026-11-04: `.chough/local/` (chosen), the Git common
  directory (per worktree key; needs Git), or keeping the temp directory and documenting
  a shared `TMPDIR`.
- `.chough/local/` already exists in sessions mode (#77) for the active pointer, with a
  `.gitignore` of `*`; it is per worktree, like `.chough/`, so worktrees never contend.
- flock(2)/LockFileEx via fs4 locks per open file description: a lock held through one
  handle blocks another handle in the same process too, which the unit test relies on;
  locks are released when the holder exits or is killed, so no stale lock remains.
- Read-only filesystems: only writers take the lock, and a writer needs write access to
  `.chough/` anyway; creating `.chough/local/` happens only when a writer locks.
- Mixed versions: an older Chough on the same project still locks in the temp directory
  and does not serialize with a new one; the CHANGELOG says so.
