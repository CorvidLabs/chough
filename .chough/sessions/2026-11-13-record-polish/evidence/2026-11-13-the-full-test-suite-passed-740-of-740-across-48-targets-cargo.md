---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-740-of-740-across-48-targets-cargo
aliases: []
claim: "The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 3453e6e8, PR 5 with main merged in (#244, #248); clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
environment: local-macos-arm64
criteria: [LIFE-20]
requirements: []
contracts: []
limitations: [The raw log is not committed]
observed: 2026-11-13T06:08:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 740 of 740 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 3453e6e8, PR 5 with main merged in (#244, #248); clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 3453e6e8a2ed, observed 2026-11-13T06:08:20Z.
