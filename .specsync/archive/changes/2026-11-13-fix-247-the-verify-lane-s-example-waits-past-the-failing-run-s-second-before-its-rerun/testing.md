---
change: fix-247-the-verify-lane-s-example-waits-past-the-failing-run-s-second-before-its-rerun
artifact: testing
---

# Testing

A test in `tests/cli.rs` runs the example's rerun-and-verify lines, cut out between
comment markers as the existing exercise tests do, against a small fixture whose
failing run's recorded second is ahead of the clock, so the rerun lands at or before
it. Red at the test commit: `bug update --status verified` refuses with run
43357893936's error. Green at the fix: the script waits past that second, and the bug
verifies. The full local suite at the final head, and the verify lane's exercise in CI.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-012 | `tests/cli.rs`: the example's rerun lands after the failing run's second, and the same-second refusal itself is unchanged |
