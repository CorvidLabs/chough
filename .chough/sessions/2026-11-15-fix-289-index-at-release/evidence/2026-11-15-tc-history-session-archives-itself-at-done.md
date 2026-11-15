---
kind: evidence
id: 2026-11-15-tc-history-session-archives-itself-at-done
aliases: []
claim: "Ran cargo test --locked --test reduce -- the_move_into_done_archives_the_session_and_clears_the_pointer the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open a_session_with_release_authorized_archives_itself_after_release for TC-history-session-archives-itself-at-done: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-session-archives-itself-at-done]
case_digest: [0f8db0da0138d44c7dee4543239712e3cc4f6795e8f4a33b80c5cad97819aa54]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - reduce
  - "--"
  - the_move_into_done_archives_the_session_and_clears_the_pointer
  - the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open
  - a_session_with_release_authorized_archives_itself_after_release
exit_code: 0
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:4373ccc3617efbbd84e544cd5676c12bf8da1ad6d573b700d321323be3651284
criteria: [DRIVE-13, DRIVE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-15T20:22:15Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test reduce -- the_move_into_done_archives_the_session_and_clears_the_pointer the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open a_session_with_release_authorized_archives_itself_after_release for TC-history-session-archives-itself-at-done: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:15Z. The raw log stays in the machine-local store.
