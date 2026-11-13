---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-731-of-731-across-48-targets-cargo
aliases: []
claim: "The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 595d0930, #243's final head with main merged in; clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: 595d09309e6644f46c0608e8d91bab5ff9db6e60
content: b98eab13fdc77afc8782e0d29cb271be64181bed096c9014b27bebff794e3366
environment: local-macos-arm64
criteria: [FLOW-19]
requirements: []
contracts: []
limitations: [The raw log is not committed]
observed: 2026-11-13T02:57:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 731 of 731 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 595d0930, #243's final head with main merged in; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 595d09309e66, observed 2026-11-13T02:57:54Z.
