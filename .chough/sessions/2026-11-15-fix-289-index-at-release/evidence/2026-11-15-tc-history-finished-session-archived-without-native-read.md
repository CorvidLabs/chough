---
kind: evidence
id: 2026-11-15-tc-history-finished-session-archived-without-native-read
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_session_whose_native_change_is_archived_is_finished_without_a_native_read for TC-history-finished-session-archived-without-native-read: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-finished-session-archived-without-native-read]
case_digest: [513ca3bcffc73a6398d153bd10dc6e8d0f3e28b2d589a689baf27615ecbbe32a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_session_whose_native_change_is_archived_is_finished_without_a_native_read
exit_code: 0
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:977737c896b407664d08f0cf51f91d92a3e6235be10533aaf3182924fad3ab78
criteria: [DRIVE-16]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-15T20:22:16Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test sessions -- --exact a_session_whose_native_change_is_archived_is_finished_without_a_native_read for TC-history-finished-session-archived-without-native-read: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:16Z. The raw log stays in the machine-local store.
