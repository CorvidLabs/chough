---
kind: evidence
id: 2026-11-18-tc-cli-a-pinned-shell-writes-its-own-session
aliases: []
claim: "Ran cargo test --locked --test session_pin -- --exact a_pinned_shell_writes_its_own_session_whatever_the_pointer_says for TC-cli-a-pinned-shell-writes-its-own-session: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-pinned-shell-writes-its-own-session]
case_digest: [b8c01ad95549223a50c9ecceb74263659ccc64ebb276810bdc11fe89ab87750e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - session_pin
  - "--"
  - "--exact"
  - a_pinned_shell_writes_its_own_session_whatever_the_pointer_says
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: 094dd2bde21a5b85d6db35218b6e28ef21fb461ca8fdace241ded34bcf42faa9
environment: local-macos-arm64
fingerprint: sha256:4ef4b5d00d88dc43fcec7e252924d282b1be1d8d6f0a65e0e5e63d2a79f5446c
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:31:57Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_pinned_shell_writes_its_own_session_whatever_the_pointer_says for TC-cli-a-pinned-shell-writes-its-own-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T18:31:57Z. The raw log stays in the machine-local store.
