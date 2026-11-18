---
kind: evidence
id: 2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations
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
duration_seconds: 0
revision: 9820c7fae0adf96edcd1cdd466c1ec5b1a47ede1
content: 5a54645b2fd690d4ff584e5b36f63b2cd2182d829d59b6103f3e1d801100c5e7
environment: local-macos-arm64
fingerprint: sha256:c12f50def4ce24abd01dee98b22d7eb9b5ed06303a06a68214ddb92e539149a9
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:50:13Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_bugs_body_links_its_carried_observations for TC-records-a-migrated-bugs-body-links-its-carried-observations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9820c7fae0ad, observed 2026-11-18T19:50:13Z. The raw log stays in the machine-local store.
