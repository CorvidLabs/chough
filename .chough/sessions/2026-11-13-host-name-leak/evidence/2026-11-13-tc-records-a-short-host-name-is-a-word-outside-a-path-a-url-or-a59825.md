---
kind: evidence
id: 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-a59825
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
content: e6ec921d16e0556b7c5eee7012581b903d4bce5df7161fa14a82466e480e9c6a
environment: local-macos-arm64
fingerprint: sha256:4e56ccfdc2badbd4c3599a576438478b010dfafcdc96e382ad67f979971de04d
criteria: [LIFE-26, LIFE-17, FLOW-22]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T02:23:14Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 2afedd91aadf, observed 2026-11-13T02:23:14Z. The raw log stays in the machine-local store.
