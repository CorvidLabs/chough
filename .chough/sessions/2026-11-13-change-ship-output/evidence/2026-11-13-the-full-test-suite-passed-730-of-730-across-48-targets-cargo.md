---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-730-of-730-across-48-targets-cargo
aliases: []
claim: The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 22568b99 with REQ-cli-059's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean
result: passed
evidence_kind: product-test
method: automated
revision: d0fc7e970adb7ed2f55cc8c23c2bbb5d6112fcd9
content: b98eab13fdc77afc8782e0d29cb271be64181bed096c9014b27bebff794e3366
environment: local-macos-arm64
criteria: [FLOW-19]
requirements: []
contracts: []
limitations:
  - Run before merging main (#241); after the merge the change, leaks, docs and changelog targets and the bin unit tests were rerun
  - The raw log is not committed
observed: 2026-11-13T02:42:02Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 730 of 730 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 22568b99 with REQ-cli-059's new text materialized; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision d0fc7e970adb, observed 2026-11-13T02:42:02Z.
