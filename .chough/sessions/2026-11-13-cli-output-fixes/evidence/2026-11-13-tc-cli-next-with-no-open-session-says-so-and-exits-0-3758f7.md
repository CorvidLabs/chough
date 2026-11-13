---
kind: evidence
id: 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0-3758f7
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
revision: 4001748c5694f643883219e37d690c4271bfc064
content: 489d14fd5c396a13d756cc137bef855dfad477971358d696dc797684d4e06219
environment: local-macos-arm64
fingerprint: sha256:10e8b60780f0059224613d252c68c94b7273fafb7ebc3c3e275a804396f7b764
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:30:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4001748c5694, observed 2026-11-13T03:30:55Z. The raw log stays in the machine-local store.
