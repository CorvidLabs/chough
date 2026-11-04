---
change: show-a-failed-native-command-as-exit-n-or-killed-by-signal-n-instead-of-rust-debug-text-issue-87
artifact: design
---

# Design

- src/records.rs: `Captured` gains `signal: Option<i32>` (Unix: the terminating signal
  from `ExitStatusExt::signal`; None elsewhere or when it exited or timed out).
  `failure` starts with `exit <code>`, `killed by signal <number>`, `timed out` or
  `no exit status`, then the bounded stdout and stderr as before.
- No other output changes; the native section state stays `failed`.
