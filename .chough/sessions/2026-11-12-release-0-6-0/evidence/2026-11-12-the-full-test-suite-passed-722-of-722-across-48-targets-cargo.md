---
kind: evidence
id: 2026-11-12-the-full-test-suite-passed-722-of-722-across-48-targets-cargo
aliases: []
claim: The full test suite passed 722 of 722 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a7d6406a, the head with every third-round fix integrated; clippy --all-targets -D warnings and rustfmt --check were clean there
result: passed
evidence_kind: product-test
method: automated
revision: a7d6406a9adf06712d4abd69563c231969a0f3ab
content: c858421dacd24efa60e900f2ea6ba9ac92b75d6a2dc2d54f6d8048f19c7d46e4
environment: local-macos-arm64
criteria: [DRIVE-27, FLOW-16, FLOW-23]
requirements: []
contracts: []
limitations:
  - "Run at a7d6406a, not at the HEAD this entry records: the commits after it add only the CHANGELOG's third-round items, this session's title and summary and a ticked task, where the changelog, docs and prose targets passed"
  - Local macOS run on Rust 1.96.0; hosted CI runs the suite only once the change is archived
  - The raw log is not committed
observed: 2026-11-12T23:15:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 722 of 722 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at a7d6406a, the head with every third-round fix integrated; clippy --all-targets -D warnings and rustfmt --check were clean there.**

Recorded as passed in local-macos-arm64 at revision a7d6406a9adf, observed 2026-11-12T23:15:36Z.
