---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-728-of-728-across-48-targets-cargo
aliases: []
claim: The full test suite passed 728 of 728 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at dbbcedb9 with the fourth round's requirement text materialized, the final source for Leif's review; clippy --all-targets -D warnings and rustfmt --check were clean there
result: passed
evidence_kind: product-test
method: automated
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
criteria: [DRIVE-27, FLOW-16, FLOW-23]
requirements: []
contracts: []
limitations:
  - Run on the working tree at dbbcedb9 before the materialized spec text was committed; the commits after it add only that text, the records spec's documentation and Chough records
  - Local macOS run on Rust 1.96.0; hosted CI runs the suite only once the change is archived
  - The raw log is not committed
observed: 2026-11-13T00:15:59Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 728 of 728 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at dbbcedb9 with the fourth round's requirement text materialized, the final source for Leif's review; clippy --all-targets -D warnings and rustfmt --check were clean there.**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:15:59Z.
