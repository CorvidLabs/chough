---
kind: evidence
id: 2026-11-19-tc-records-a-fence-hides-a-conflict-only-when-closed-f54b61
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
revision: 08801812d7fe82d42f54451eb25087df5ddaeec6
content: 2b860426c9c1f962e92c5eaef48d4cf1ed31a739e1fc4da29359c8cb5baba200
environment: local-macos-arm64
fingerprint: sha256:11c421d901c13b23b99e21c316e1722ea36e936dbb6f924390f708f5b8865483
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:41:22Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact a_fence_hides_a_conflict_only_when_it_is_one_and_closes for TC-records-a-fence-hides-a-conflict-only-when-closed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08801812d7fe, observed 2026-11-19T00:41:22Z. The raw log stays in the machine-local store.
