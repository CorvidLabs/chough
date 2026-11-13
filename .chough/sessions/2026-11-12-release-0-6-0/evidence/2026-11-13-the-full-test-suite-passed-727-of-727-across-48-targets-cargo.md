---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-727-of-727-across-48-targets-cargo
aliases: []
claim: The full test suite passed 727 of 727 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a467c61e, the head with the fourth round's four fixes integrated; clippy --all-targets -D warnings and rustfmt --check were clean there
result: passed
evidence_kind: product-test
method: automated
revision: a467c61e6edb686a2c02f1709443bbd963446b25
content: 1e3d4b1dc96fcd222b7b4c3cb1e922d5c0b93aa47190bb61424d2d99c9956761
environment: local-macos-arm64
criteria: [DRIVE-27, FLOW-16, FLOW-23]
requirements: []
contracts: []
limitations:
  - "Run at a467c61e, not at the HEAD this entry records: the commits after it add only this session's title and summary, this entry and a ticked task"
  - Local macOS run on Rust 1.96.0; hosted CI runs the suite only once the change is archived
  - The raw log is not committed
observed: 2026-11-13T00:03:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 727 of 727 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a467c61e, the head with the fourth round's four fixes integrated; clippy --all-targets -D warnings and rustfmt --check were clean there.**

Recorded as passed in local-macos-arm64 at revision a467c61e6edb, observed 2026-11-13T00:03:11Z.
