---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-bug-reference-only-when-clean]
case_digest: [b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_reference_is_shown_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: d7a716ff88a492e7d4c31e77a3ea828a5f5e00d5
content: ba0e70f36f347b58b2f49da39e0b55e9067112bffcd451b86d9c47ec3803a28c
environment: local-macos-arm64
fingerprint: sha256:ed8c5aa2a6ae2d06cd5fbaa69a149ea6f3d5594fa53dcd0cdef778d0748ce387
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:43:32Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d7a716ff88a4, observed 2026-11-19T02:43:32Z. The raw log stays in the machine-local store.
