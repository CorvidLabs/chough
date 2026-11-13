---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
aliases: []
claim: "The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 72cb5ef6, #247's fix with main merged in (#248); clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: 72cb5ef655762dcd4b1979e3e1d87149428baa68
content: 471d8569037ea762369eded05e2e6db52624aedf49636a831eb073c7fdc8cdca
environment: local-macos-arm64
criteria: [DRIVE-19]
requirements: []
contracts: []
limitations: [The raw log is not committed]
observed: 2026-11-13T07:31:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 72cb5ef6, #247's fix with main merged in (#248); clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 72cb5ef65576, observed 2026-11-13T07:31:22Z.
