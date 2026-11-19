---
kind: evidence
id: 2026-11-19-tc-history-the-way-out-names-the-open-sessions
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-the-way-out-names-the-open-sessions]
case_digest: [ea61f8b1731f5ac470892706e868693c5ee7e9fbc9d462a5748f39380f4be02f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - the_way_out_names_the_open_sessions_when_none_is_active
exit_code: 0
tests_run: 1
duration_seconds: 7
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:b41070550a5b2af840a5061b8ebc72d26164cb30b606e531c94789f3ac01e84f
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:12:17Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:12:17Z. The raw log stays in the machine-local store.
