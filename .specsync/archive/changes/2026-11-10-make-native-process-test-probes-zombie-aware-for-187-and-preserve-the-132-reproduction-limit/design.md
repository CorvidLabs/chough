---
change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
artifact: design
---

# Design

Add `common::alive(pid: &str) -> bool` for Unix integration tests. Begin with the existing `kill -0` behavior and a regression that spawns `sleep 60`, confirms the running child is detected, SIGKILLs it without waiting, and asserts that the killed, unreaped child is not alive. Reap the child before asserting so the failing-first run leaves no zombie behind.

Then bound the helper to one second with 20 ms polls. Each poll returns false when `kill -0` fails, when `ps -o stat= -p <pid>` returns no process, or when the state starts with `Z`. A process still in a non-zombie state after the deadline returns true. This preserves detection of a real survivor while allowing a killed process to exit or be reaped.

Use the helper in the three existing probes: `surviving_children` in `tests/timing.rs`, the timed-out native process check in `tests/native.rs`, and the SIGTERM check in `tests/change.rs`. Do not change `src/records.rs`, `stop_tree`, checkpoint locking, assertions' intended behavior, or any public interface.
