---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-741-of-741-across-49-targets-cargo
aliases: []
claim: "The full test suite passed 741 of 741 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at dbe09d42, #253's head after t384's two must-fixes; clippy -D warnings on the integrations test and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: cargo test --locked --no-fail-fast and cargo clippy --locked --test integrations -- -D warnings, under the heavy-tests lock; cargo fmt --check
revision: dbe09d422be76fe12904652e04ba10d8e0d3fdeb
content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
environment: local-macos-arm64
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: []
limitations: []
observed: 2026-11-13T06:47:47Z
actor:
  - agent: claude-code
    terminal: t38r
---
**The full test suite passed 741 of 741 across 49 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at dbe09d42, #253's head after t384's two must-fixes; clippy -D warnings on the integrations test and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision dbe09d422be7, observed 2026-11-13T06:47:47Z.
