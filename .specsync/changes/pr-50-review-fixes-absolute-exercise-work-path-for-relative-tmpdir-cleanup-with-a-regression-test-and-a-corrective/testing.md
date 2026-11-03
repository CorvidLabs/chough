---
change: pr-50-review-fixes-absolute-exercise-work-path-for-relative-tmpdir-cleanup-with-a-regression-test-and-a-corrective
artifact: testing
---

# Testing

- The new `tests/cli.rs` regression test passes. It fails when the absolute-path line is removed.
- Its case is run with `chough cases run`.
- A full exercise run with `TMPDIR=.` leaves no `chough-exercise.*` directory behind.
- `fledge lanes run verify` passes.
