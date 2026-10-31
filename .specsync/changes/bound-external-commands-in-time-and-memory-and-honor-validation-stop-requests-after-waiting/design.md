---
change: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
artifact: design
---

# Design

`records.rs` gains one bounded runner used by case execution and every native read:

```text
run_bounded(root, argv, Limits { stdout_cap, stderr_cap, deadline }) -> Captured {
    exit_code, timed_out, stdout, stdout_total, stderr, stderr_total,
    descendants_stopped, capture_incomplete }
```

- One reader thread per stream retains up to its cap, counts every byte and reports EOF
  on a channel. The caller polls the child every 25 ms.
- Deadline reached: stop the tree (Unix group SIGKILL; Windows `taskkill /T /F`), reap
  the child, wait up to 1 s for readers, then abandon them.
- Child exited but a stream is still open after a 2 s grace (bounded by the deadline):
  stop the tree, wait up to 1 s, then abandon; `descendants_stopped` is set.
- Caps: cases 1 MiB per stream (artifact keeps the existing 1 MiB total and adds
  `[truncated: <stream> wrote N bytes; M discarded]`); native stdout 4 MiB + 1 (so
  "over the bound" is exact), native stderr 16 KiB.
- Native deadline 120 s, overridable by `CHOUGH_NATIVE_TIMEOUT_SECONDS` (1 to 3600) for
  slow repositories and tests. A native deadline gives section state `timed-out`;
  `native_json` returns no value, so gates refuse as native-unknown.
- Case limitations add `timed out after N s; the command and its descendants were
  stopped`, `descendants still held output after exit and were stopped`, and `output
  capture incomplete`. A run whose capture is incomplete cannot be `passed`.
- Validation: before every attempt, and after waiting, stop if the stop file exists
  (`stopped`) or the checkpoint stage is paused (`paused`, resumable). The interval is
  slept in 200 ms slices that re-check both.
