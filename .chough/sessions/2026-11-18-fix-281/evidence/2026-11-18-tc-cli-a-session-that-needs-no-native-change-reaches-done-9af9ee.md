---
kind: evidence
id: 2026-11-18-tc-cli-a-session-that-needs-no-native-change-reaches-done-9af9ee
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
revision: b570d286147e9720e9f632a6b1469f537336ca5f
content: e4c824c3138a5a32b3832f7cfafd4fccc52686e686b637be61997cbe44e3e64b
environment: local-macos-arm64
fingerprint: sha256:bb84bb44725aa787fbc2955051914b298f1eb7d0c0f3c3de8371a88d731bb4cb
criteria: [DRIVE-16]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:41:08Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_session_that_needs_no_native_change_reaches_done_and_archives_as_done for TC-cli-a-session-that-needs-no-native-change-reaches-done: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b570d286147e, observed 2026-11-18T20:41:08Z. The raw log stays in the machine-local store.
