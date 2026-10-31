---
change: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
artifact: testing
---

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-011 | `tests/cases.rs`: deadline with a pipe-holding descendant; exit with a pipe-holding descendant; per-stream output bound; `tests/optional.rs`: stop during the interval and pause before an attempt |
| REQ-cli-003 | `tests/cli.rs`: native stand-in that hangs (deadline override), that writes beyond the parse bound, and that floods stderr |
| REQ-cli-006 | unit tests of the bounded reader: retained bytes never exceed the cap while totals keep counting |

Unix tests use `sh` descendants; Windows tests use `cmd /c start /b` descendants; both
run in hosted CI. Timing assertions use generous margins (deadline + 10 s) to avoid
flakiness and still fail the 20 s reproductions. Regression: full `cargo test`,
`fledge lanes run verify`, and a rerun of the recorded reproductions on macOS.
