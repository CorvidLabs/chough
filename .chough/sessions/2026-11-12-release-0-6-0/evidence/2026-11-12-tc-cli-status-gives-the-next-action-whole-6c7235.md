---
kind: evidence
id: 2026-11-12-tc-cli-status-gives-the-next-action-whole-6c7235
aliases: []
claim: "Ran cargo test --locked --test next -- --exact status_gives_the_next_valid_action_whole for TC-cli-status-gives-the-next-action-whole: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-status-gives-the-next-action-whole]
case_digest: [b684c5e4208964f55d65a715d31ef8bedf9bd03cebd5d42b444f909ac0176acb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - status_gives_the_next_valid_action_whole
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 7be760ccbc7da0fbb0ebe1a63ff562e5782ae91b
content: 2bc6f78c658c710d8247ff73e0cc5320123d1398568de992f6a8852e275b18e1
environment: local-macos-arm64
fingerprint: sha256:6f57810b1783e2283b3257103295aea9c416ec0e96afbf0f15b9db5f1dad10f3
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:45:01Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact status_gives_the_next_valid_action_whole for TC-cli-status-gives-the-next-action-whole: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7be760ccbc7d, observed 2026-11-12T17:45:01Z. The raw log stays in the machine-local store.
