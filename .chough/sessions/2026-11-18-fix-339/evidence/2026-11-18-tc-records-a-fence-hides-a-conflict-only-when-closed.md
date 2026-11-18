---
kind: evidence
id: 2026-11-18-tc-records-a-fence-hides-a-conflict-only-when-closed
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_fence_hides_a_conflict_only_when_it_is_one_and_closes for TC-records-a-fence-hides-a-conflict-only-when-closed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-fence-hides-a-conflict-only-when-closed]
case_digest: [5b9febc9e28afd4d7ac7f830c751fa4821b8b1384e1daef416793ac1cd2afe80]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_fence_hides_a_conflict_only_when_it_is_one_and_closes
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 05ea2811ead5c40449265fe3e68f3114375526eb
content: 668614d7d416992d94bdab698001370290f983469c669fc57b014ed26862ee3c
environment: local-macos-arm64
fingerprint: sha256:bcb10098bbcc6e0ccbe79da56409c09b0aeee070cb44629d81761841855207f0
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:07:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fence_hides_a_conflict_only_when_it_is_one_and_closes for TC-records-a-fence-hides-a-conflict-only-when-closed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 05ea2811ead5, observed 2026-11-18T23:07:00Z. The raw log stays in the machine-local store.
