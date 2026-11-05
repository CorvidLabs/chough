---
change: show-a-failed-native-command-as-exit-n-or-killed-by-signal-n-instead-of-rust-debug-text-issue-87
artifact: testing
---

# Testing

- tests/native_failure.rs:
  - `a_failed_native_read_shows_its_exit_in_plain_words`: a stub `specsync` that exits 2
    gives `status` content starting `exit 2` and a `next` line with `(failed): exit 2`,
    never `Some(`.
  - `a_native_read_killed_by_a_signal_says_so`: a stub that kills itself with SIGTERM
    gives `killed by signal 15`.
- tests/cli.rs: the stderr-flood case now expects `exit 2` instead of `exit=Some(2)`.
- Each fails on main 91a1fdd; `fledge lanes run verify` passes on Linux.
