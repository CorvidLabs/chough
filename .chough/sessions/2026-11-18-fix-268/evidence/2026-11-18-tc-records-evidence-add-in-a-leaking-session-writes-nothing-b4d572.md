---
kind: evidence
id: 2026-11-18-tc-records-evidence-add-in-a-leaking-session-writes-nothing-b4d572
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact evidence_add_in_a_session_holding_a_leak_writes_nothing for TC-records-evidence-add-in-a-leaking-session-writes-nothing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-evidence-add-in-a-leaking-session-writes-nothing]
case_digest: [1f377c2c55944a6a9b764cb69d519db6deb1af6b64540e99b3cef5a9cadd3b4a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - evidence_add_in_a_session_holding_a_leak_writes_nothing
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 6f5347343890308fc96ff050d004404d5c35bbe9
content: 33fab87e35740e5ff5f96d0121f4200d560a55a78ba7ed398d0cbf5795120a1a
environment: local-macos-arm64
fingerprint: sha256:5e9845c07e905b59805a2d673a7e75a7fe17d745661776a0a120f3f65fb4ddfe
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:09:16Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact evidence_add_in_a_session_holding_a_leak_writes_nothing for TC-records-evidence-add-in-a-leaking-session-writes-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6f5347343890, observed 2026-11-18T16:09:16Z. The raw log stays in the machine-local store.
