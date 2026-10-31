---
change: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
artifact: research
---

# Research

- Rust std: `std::os::unix::process::CommandExt::process_group(0)` (stable since 1.64)
  puts the child in its own group; std has no killpg, so `libc::kill(-pgid, SIGKILL)`
  is used. `libc` 0.2 is already in Cargo.lock through existing dependencies.
- A process group id is not reused while any member lives, so killing the group after
  the leader exits is safe exactly when a pipe is still held (a live member exists).
- Windows: `taskkill /PID <pid> /T /F` stops a live process and its descendants without
  a new dependency; it cannot find descendants once their parent has exited, so the
  runner never waits on a reader past the grace period and reports capture incomplete.
  Job objects would close that gap but need windows-sys and a suspended start that std
  does not expose; deferred.
- Pipes: reading both streams on separate threads avoids the full-pipe deadlock; a
  reader that keeps draining after its cap keeps the child from blocking on write.
