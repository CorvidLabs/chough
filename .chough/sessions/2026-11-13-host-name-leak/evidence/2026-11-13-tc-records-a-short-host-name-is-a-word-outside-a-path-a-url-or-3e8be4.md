---
kind: evidence
id: 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-3e8be4
aliases: []
claim: "Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host]
case_digest: [c88004f0d6ff74b260d77b5808d0ca8cfdfc535ac0d65f6f96d375fd6a50ecaa]
command:
  - cargo
  - test
  - "--locked"
  - "--bin"
  - chough
  - "--"
  - "--exact"
  - leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host
exit_code: 0
duration_seconds: 0
revision: 2afedd91aadf075a51e1933849c57d28b0273a09
content: 1924becf8e77d3ef9100282e20b749296511dc9aa70163bf32933bd51fb04c51
environment: local-macos-arm64
fingerprint: sha256:68eccee8e3b570361995e8d1921ead73b478545cae652bad9f0b2a5702be6538
criteria: [LIFE-26, LIFE-17, FLOW-22]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T02:15:02Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 2afedd91aadf, observed 2026-11-13T02:15:02Z. The raw log stays in the machine-local store.
