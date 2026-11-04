---
change: stop-what-chough-started-when-chough-is-stopped-issue-75
artifact: design
---

# Design

- src/records.rs: a small Unix-only `interrupt` module. While `run_bounded` owns a child,
  it installs a handler for SIGINT, SIGTERM and SIGHUP that only stores the signal number
  in an atomic (async-signal-safe). A signal whose inherited disposition is ignored is
  left ignored. The previous dispositions are restored when the run ends.
- The runner's poll loop (every 25 ms) checks the stored signal. When one arrived, it
  stops the child's process group with the existing `stop_tree`, reaps the child and
  returns. `run_bounded` then restores the dispositions and, when a signal arrived at any
  point during the run, ends Chough by that same signal (default disposition, then
  `raise`), so callers never record an observation, artifact or checkpoint change, and
  the parent sees the usual death by signal.
- Nothing is written before the runner returns (the artifact and observation are written
  after it), so no cleanup of partial files is needed. Gates, schemas, output keys, the
  deadline and held-output handling are unchanged, and no dependency is added (`libc` is
  already a Unix dependency).
