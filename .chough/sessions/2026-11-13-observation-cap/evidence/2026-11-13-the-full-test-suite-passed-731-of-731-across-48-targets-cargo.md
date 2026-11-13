---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo
aliases: []
claim: The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 31f85795, PR 2 with main merged in; clippy --all-targets -D warnings and rustfmt --check were clean
result: passed
evidence_kind: product-test
method: automated
revision: d247ac6b17a0a48cfa00ece86730ed758d212389
content: f1693f112aee648629016053eeb1da4f43148a8d7945c6bee3912127fc705702
environment: local-macos-arm64
criteria: [DRIVE-14]
requirements: []
contracts: []
limitations:
  - Run before the file list was widened, which changed only the change's records
  - The raw log is not committed
observed: 2026-11-13T02:59:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 31f85795, PR 2 with main merged in; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision d247ac6b17a0, observed 2026-11-13T02:59:54Z.
