---
kind: evidence
id: 2026-11-19-tc-history-session-use-names-a-session-only-when-clean
aliases: []
claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-history-session-use-names-a-session-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-session-use-names-a-session-only-when-clean]
case_digest: [a547df812106cb5ac65af3b5c0d2597f6d7e57607acf4932f6a01cbc9f843f80]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - session_pin
  - "--"
  - "--exact"
  - a_session_name_is_shown_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:7f368bc11f4122d410f2499e81797a72661f113d07fa1bb553910475f601d6d3
criteria: [LIFE-26]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:23:44Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-history-session-use-names-a-session-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75f43060e66b, observed 2026-11-19T14:23:44Z. The raw log stays in the machine-local store.
