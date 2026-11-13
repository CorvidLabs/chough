---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
aliases: []
claim: "The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a1f4735a, PR 5 with #243 merged in; clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
environment: local-macos-arm64
criteria: [LIFE-20]
requirements: []
contracts: []
limitations:
  - "Run before merging #244 and #248 from main; the post-merge run is recorded separately"
  - The raw log is not committed
observed: 2026-11-13T04:26:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a1f4735a, PR 5 with #243 merged in; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision a1f4735a820d, observed 2026-11-13T04:26:54Z.
