---
kind: evidence
id: 2026-11-18-tc-cli-a-pinned-shell-writes-its-own-session-d91946
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
duration_seconds: 0
revision: f6c1f97bc0bbcd34900e798f3998deb9e8c96a8b
content: e7d6dec64f04b40d817059beea015d1b92a5ccf1395e90715e370cffc7df937f
environment: local-macos-arm64
fingerprint: sha256:9fe5079939dfe6ddd0f486ddf99641e699956546693a2b3294924d9b0ab95099
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:58:05Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_pinned_shell_writes_its_own_session_whatever_the_pointer_says for TC-cli-a-pinned-shell-writes-its-own-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f6c1f97bc0bb, observed 2026-11-18T18:58:05Z. The raw log stays in the machine-local store.
