---
kind: evidence
id: 2026-11-13-tc-records-a-short-host-name-is-a-word-outside-a-path-a-url-or
aliases: []
claim: "Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: failed (exit 101)."
result: failed
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
exit_code: 101
duration_seconds: 0
revision: a6e35a2a2fc53c663b67ad9e1365ed93a75aca3a
content: 2aa7ebcd7186df65339001a98dedfc0f1449098bd6863b966f69547c0c2137af
environment: local-macos-arm64
fingerprint: sha256:683bb23c2fea47be250634c49e352e23b8f41ece666bf6fbe4d5f0c68f206b3d
criteria: [LIFE-26, LIFE-17, FLOW-22]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations:
  - "expected output missing: \"test leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host ... ok\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T02:14:49Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --bin chough -- --exact leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host for TC-records-a-short-host-name-is-a-word-outside-a-path-a-url-or-user-at-host: failed (exit 101).**

Recorded as failed in local-macos-arm64 at revision a6e35a2a2fc5, observed 2026-11-13T02:14:49Z. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "test leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host ... ok".

## Key failing lines

-     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
-      Running unittests src/main.rs (target/debug/deps/chough-eb51e27f36ee6e84)
- error: test failed, to rerun pass `--bin chough`
