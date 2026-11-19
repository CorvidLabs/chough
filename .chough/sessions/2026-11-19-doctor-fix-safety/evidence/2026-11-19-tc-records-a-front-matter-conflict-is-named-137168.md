---
kind: evidence
id: 2026-11-19-tc-records-a-front-matter-conflict-is-named-137168
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-front-matter-conflict-is-named]
case_digest: [18be79babe813b859103021cf7d5c686461a85549f7a415e35e90cdaa6e93862]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_front_matter_conflict_is_named_and_resolved_not_shelved
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:823384ca74f80b0e7df98ce0000d58c53094672d1a325db262157163215996cb
criteria: [DRIVE-17]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:15:24Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-records-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:15:24Z. The raw log stays in the machine-local store.
