---
change: give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138
artifact: design
---

# Design

## Tests

New tests go into the existing test file of each requirement; no new test file is added, so no spec's file list changes. Every test that reads repository text normalizes CRLF and checks a CRLF copy too (#131 lesson). For each new assertion, one targeted mutation is applied by hand, the test is shown to fail, and the file is restored. The logs are kept as Chough evidence.

## examples/measure-report.sh

`sh examples/measure-report.sh OLD NEW ROOT [RUNS]` (POSIX sh, like the exercise script; RUNS defaults to 3).

- Refusals exit 2 with `usage: sh examples/measure-report.sh OLD NEW ROOT [RUNS]`: a missing or non-executable binary, a ROOT that is not a Git project with a commit, or a RUNS that is not a positive integer.
- It prints, in order:
  - `root <absolute path> revision <commit> dirty <yes|no>`
  - `native_timeout_seconds <CHOUGH_NATIVE_TIMEOUT_SECONDS or default>`
  - one `binary <old|new> sha256 <hex> version <--version output>` line per binary
  - one `run <side> <n> <first-process|repeated> exit=<code> wall_ms=<n> native_ms=<n> local_ms=<n> native_reads=<n> incomplete_reads=<n>` line per run of `CHOUGH_TRACE=1 <binary> --root ROOT next --json`. `native_ms` sums the trace's completed (`observed`) native reads, `local_ms` is wall minus that, and any other native state counts as incomplete.
- Summary:
  - `summary local old_median_ms=… new_median_ms=…`, always printed.
  - `summary native old_median_ms=… new_median_ms=…` only when every native read completed.
  - Otherwise `summary native incomplete: <k> native reads timed out or were unavailable; no native comparison`.
- No line combines local and native time into one verdict, so a local change is never reported as fixing native slowness.
- docs/cli.md's profiling procedure names the script.
