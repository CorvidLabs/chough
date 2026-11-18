---
kind: evidence
id: 2026-11-18-tc-cli-a-session-that-needs-no-native-change-reaches-done
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
revision: 7780a0543faafb328fc765f01c96f5fe3b5d60f4
content: da7c10b28f979f9fad4560e23339e2b43ebd4d3a7bc44e61a08d1f587ea7aaf0
environment: local-macos-arm64
fingerprint: sha256:12908a67d5164a926d0be290d33458100273790c381753433332c665bd01ef07
criteria: [DRIVE-16]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:22:47Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_session_that_needs_no_native_change_reaches_done_and_archives_as_done for TC-cli-a-session-that-needs-no-native-change-reaches-done: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7780a0543faa, observed 2026-11-18T20:22:47Z. The raw log stays in the machine-local store.
