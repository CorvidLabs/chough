---
kind: evidence
id: 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-a87838
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-with-no-open-session-says-so-and-exits-0]
case_digest: [3b27295d134205f159c1e25e54aac9aedf67971962308394c42ad1aa59335322]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_with_no_open_session_says_so_and_exits_0
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: bc275212d81eb8ac9c8498f189809431ab75340c
content: 289ed18f7eeb791ead6c5c6ddb8da209cbcde473ed6022b7b1aa7c4b4dbb7082
environment: local-macos-arm64
fingerprint: sha256:784bc8b77e7c5854c2d094373ad1032ead439032e003eae4861b945e363b96e1
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:00:58Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bc275212d81e, observed 2026-11-13T03:00:58Z. The raw log stays in the machine-local store.
