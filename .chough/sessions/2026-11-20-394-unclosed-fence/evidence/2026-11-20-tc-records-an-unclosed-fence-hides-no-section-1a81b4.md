---
kind: evidence
id: 2026-11-20-tc-records-an-unclosed-fence-hides-no-section-1a81b4
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_code_fence_that_never_closes_hides_no_section for TC-records-an-unclosed-fence-hides-no-section: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-an-unclosed-fence-hides-no-section]
case_digest: [2619feb7346e38415773a859333377e9b6395ef39fd3b28c3da1f79c6734be75]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_code_fence_that_never_closes_hides_no_section
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: f851c62d38e6a1e46f2abc92fdc9190b02bf31fd
content: 66ec4cbf7168fd59132e9ead6fc36fb3dced1359a0030dd4115f187abd136775
environment: local-macos-arm64
fingerprint: sha256:4e86cc7c0c8761427eb8cee60bf3afdc7f2388fd8aa0213f5768bdcc52a58188
criteria: [DRIVE-24]
requirements: [REQ-records-004]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:15:09Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test records -- --exact a_code_fence_that_never_closes_hides_no_section for TC-records-an-unclosed-fence-hides-no-section: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f851c62d38e6, observed 2026-11-20T16:15:09Z. The raw log stays in the machine-local store.
