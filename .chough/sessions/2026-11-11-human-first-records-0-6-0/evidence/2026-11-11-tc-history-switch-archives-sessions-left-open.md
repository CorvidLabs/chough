---
kind: evidence
id: 2026-11-11-tc-history-switch-archives-sessions-left-open
aliases: [OBS-tc-history-switch-archives-sessions-left-open-098c034bb55f6a0dff1be113]
claim: "Ran cargo test --locked --test sessions -- starting_or_switching_never_touches_another_session a_done_session_with_unfolded_entries_stays_open session_list_reports_an_archived_session_as_archived inbox_add_racing_an_archive_never_strands_its_entry for TC-history-switch-archives-sessions-left-open: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-switch-archives-sessions-left-open]
case_digest: [11b818fe297f4cbfeb5209555b45e050eeec7af07a62ffb5dd8c10b70217cf28]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - starting_or_switching_never_touches_another_session
  - a_done_session_with_unfolded_entries_stays_open
  - session_list_reports_an_archived_session_as_archived
  - inbox_add_racing_an_archive_never_strands_its_entry
exit_code: 0
duration_seconds: 2
revision: ec8f8a6f6162b324850d551a37cfc56f9c9d5d5a
content: 3f6ba540266bffc9c15f3048cf7f301fe63daa7ddad1a87fe86edfee6920b00c
environment: local-macos-arm64
fingerprint: sha256:5f4bb4368a5fab04cfe858c9b32e84c6d330163e15096cb21bf19b6cb47bd49c
criteria: [DRIVE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-11T18:17:44Z
---
**Ran cargo test --locked --test sessions -- starting_or_switching_never_touches_another_session a_done_session_with_unfolded_entries_stays_open session_list_reports_an_archived_session_as_archived inbox_add_racing_an_archive_never_strands_its_entry for TC-history-switch-archives-sessions-left-open: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision ec8f8a6f6162, observed 2026-11-11T18:17:44Z. The raw log stays in the machine-local store.
