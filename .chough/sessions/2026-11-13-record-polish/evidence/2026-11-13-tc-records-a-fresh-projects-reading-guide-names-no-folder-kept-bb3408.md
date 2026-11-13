---
kind: evidence
id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-bb3408
aliases: []
claim: "Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5]
case_digest: [fc6498e97a800fd70ade8c296a2b7daa5e7caa967d3b099e0f536fb7ef7c8f35]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3453e6e8a2edd34624c6c89b620ed8b262aa3a2c
content: 13e8fbe68ad3139b1c422a706d9abd7d321e4259afebf7c54f9ab96bea045d8e
environment: local-macos-arm64
fingerprint: sha256:37d378af0b471590d84bd983a81401f9bbc53fc18c267524ac6f203da4a0e40c
criteria: [DRIVE-24, KNOW-32]
requirements: [REQ-records-002]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:07:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3453e6e8a2ed, observed 2026-11-13T06:07:53Z. The raw log stays in the machine-local store.
