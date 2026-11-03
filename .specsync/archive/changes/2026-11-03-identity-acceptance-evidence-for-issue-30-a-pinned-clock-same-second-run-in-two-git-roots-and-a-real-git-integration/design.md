---
change: identity-acceptance-evidence-for-issue-30-a-pinned-clock-same-second-run-in-two-git-roots-and-a-real-git-integration
artifact: design
---

# Design

- `src/records.rs`: `#[cfg(test)]` thread-local hooks, built like `identity::with_random_results`:
  - `with_pinned_clock(seconds, run)`, which `now()` honors;
  - `with_native_stdout(program, stdout, run)`, which `native()` honors, so case-link validation can read a canned `hi export`.
  Release builds are unchanged.
- `src/main.rs` tests: a same-second test. It initializes one root, clones it into a second root, runs the same `command`-bound case (`git --version`) on branch `a` and branch `b` under one pinned second, merges `b` into `a`, and checks IDs, paths, times, bytes and freshness.
- `tests/cases.rs`: a real Git fixture test using the built binary.
