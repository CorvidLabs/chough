---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
aliases: []
claim: "The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 9a081e0c, #246's final head; clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: 9a081e0c6317beacb6ea4afa7365d1f0884914e6
content: 31575271f78a6fda3693493a400c10f02b053187d68cc82330a44690f043bf84
environment: local-macos-arm64
criteria: [DRIVE-27]
requirements: []
contracts: []
limitations: [The raw log is not committed]
observed: 2026-11-13T07:59:26Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 736 of 736 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 9a081e0c, #246's final head; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 9a081e0c6317, observed 2026-11-13T07:59:26Z.
