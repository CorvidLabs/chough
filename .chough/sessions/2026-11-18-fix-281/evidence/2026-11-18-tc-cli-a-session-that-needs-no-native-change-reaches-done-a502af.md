---
kind: evidence
id: 2026-11-18-tc-cli-a-session-that-needs-no-native-change-reaches-done-a502af
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact a_session_that_needs_no_native_change_reaches_done_and_archives_as_done for TC-cli-a-session-that-needs-no-native-change-reaches-done: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-session-that-needs-no-native-change-reaches-done]
case_digest: [5a567551727c57599eca5a5fddcb2c091c3a7023b9d78fd5f69f58895acf729a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - a_session_that_needs_no_native_change_reaches_done_and_archives_as_done
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 01f1283a72e63585b4e39154ef0476d69294fe28
content: 3052f8027a57ef5f8c273b16db1b2357b482d3d032657d6e30522b6a4615cc8a
environment: local-macos-arm64
fingerprint: sha256:7125811fdaa06096e3d01fe47200b2a50a9c507797078183a29c21dbf1703de4
criteria: [DRIVE-16]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:06:31Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_session_that_needs_no_native_change_reaches_done_and_archives_as_done for TC-cli-a-session-that-needs-no-native-change-reaches-done: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 01f1283a72e6, observed 2026-11-18T21:06:31Z. The raw log stays in the machine-local store.
