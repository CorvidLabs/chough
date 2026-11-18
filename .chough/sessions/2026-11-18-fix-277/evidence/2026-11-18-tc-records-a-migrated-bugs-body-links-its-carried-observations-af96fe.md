---
kind: evidence
id: 2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations-af96fe
aliases: []
claim: "Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_bugs_body_links_its_carried_observations for TC-records-a-migrated-bugs-body-links-its-carried-observations: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-migrated-bugs-body-links-its-carried-observations]
case_digest: [bc3c1d94fdcbb63ba70f9b439ac39315cd6056842475ef710c9bdd3ba9fa4038]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_evidence
  - "--"
  - "--exact"
  - a_migrated_bugs_body_links_its_carried_observations
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: a1ed8089c52511a7e73a3f07aa1596f63238f54d
content: 880cba53cfe4f55cd5274d3e02e80024370ac7088462c6340a9b389eb9b18e68
environment: local-macos-arm64
fingerprint: sha256:8048ecb5a3a7abc5c06d6c2f42e2db7addbba792e5fdd7900ecdfcdc3a4dde0f
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:27:58Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_bugs_body_links_its_carried_observations for TC-records-a-migrated-bugs-body-links-its-carried-observations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a1ed8089c525, observed 2026-11-18T20:27:58Z. The raw log stays in the machine-local store.
