---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
aliases: []
claim: The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 2afedd91 with REQ-records-006's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean
result: passed
evidence_kind: product-test
method: automated
revision: 2afedd91aadf075a51e1933849c57d28b0273a09
content: e6ec921d16e0556b7c5eee7012581b903d4bce5df7161fa14a82466e480e9c6a
environment: local-macos-arm64
criteria: [LIFE-26]
requirements: []
contracts: []
limitations:
  - Run on the working tree before the materialized text was committed
  - The raw log is not committed
observed: 2026-11-13T02:23:14Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 2afedd91 with REQ-records-006's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 2afedd91aadf, observed 2026-11-13T02:23:14Z.
