---
kind: evidence
id: 2026-11-18-tc-records-a-migrated-bugs-body-links-its-carried-observations-1d1109
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
revision: bc43b047122c16d13223c45bc6a4e79a7ae0b382
content: ac44d3be9a821f4712151092e8caddfeee2677d28a42a603061d4f888c6c1557
environment: local-macos-arm64
fingerprint: sha256:4e89ef0ea2b263265475ed11b701ec8a4e0e97f75a0cb57f2111c44e01ee0175
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T19:54:20Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_bugs_body_links_its_carried_observations for TC-records-a-migrated-bugs-body-links-its-carried-observations: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bc43b047122c, observed 2026-11-18T19:54:20Z. The raw log stays in the machine-local store.
