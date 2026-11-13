---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
aliases: []
claim: "The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at aed5bb1c, #249's final head; clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: aed5bb1c539b53587477dea6dd4d6a5bdf2fdff4
content: 938ce32ddca4a521c54a2a904c9e2a7c795174d309d73644b9b67c0fdfedd641
environment: local-macos-arm64
criteria: [FLOW-23]
requirements: []
contracts: []
limitations: [The raw log is not committed]
observed: 2026-11-13T08:04:30Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at aed5bb1c, #249's final head; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision aed5bb1c539b, observed 2026-11-13T08:04:30Z.
