---
kind: evidence
id: 2026-11-12-the-full-test-suite-passed-703-of-703-across-48-targets-cargo
aliases: []
claim: The full test suite passed 703 of 703 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 55684263, the head with every second-round fix integrated; clippy --all-targets -D warnings and rustfmt --check were clean there
result: passed
evidence_kind: product-test
method: automated
revision: 425fa9485b49d2c6aa15bd39f06806a74e8910e5
content: 5947f7d01b200e63a0083424a7b204453f8efde1287143984eeb8cf94b1e00e5
environment: local-macos-arm64
criteria: [DRIVE-27, FLOW-16, FLOW-23]
requirements: []
contracts: []
limitations:
  - "Run at 55684263, not at the HEAD this entry records: 425fa948 adds only the CHANGELOG bullet for the second round and a ticked task, where the changelog, docs and prose targets passed (5, 17 and 1)"
  - Local macOS run on Rust 1.96.0; hosted CI runs the suite only once the change is archived
  - The raw log is not committed
observed: 2026-11-12T21:17:18Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 703 of 703 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 55684263, the head with every second-round fix integrated; clippy --all-targets -D warnings and rustfmt --check were clean there.**

Recorded as passed in local-macos-arm64 at revision 425fa9485b49, observed 2026-11-12T21:17:18Z.
