---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo-a7ed99
aliases: []
claim: "The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 558befd7, #249's final head after the workflow-v3 docs note; clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: 558befd746ef9e9da45b4989d5762ea8cec000a5
content: e201dd394a545c8e93a18d5c2faf05e7b825ccfb2d9d332796f11b9055fa4cf3
environment: local-macos-arm64
criteria: [FLOW-23]
requirements: []
contracts: []
limitations: [The raw log is not committed]
observed: 2026-11-13T08:42:47Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 558befd7, #249's final head after the workflow-v3 docs note; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 558befd746ef, observed 2026-11-13T08:42:47Z.
