---
kind: evidence
id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-d27b46
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
revision: a1f4735a820dacdb102a4ef4d9c1bda53e31f7bb
content: 65b5bda886fd3fbadb3834aeb5a77124da7d7b9a0d3f00f12ba7d6040b8ce43b
environment: local-macos-arm64
fingerprint: sha256:355e2c9b36a96e20ad140601484a72d6fe4d728c98ea69839e0e82f171df5ca0
criteria: [DRIVE-24, KNOW-32]
requirements: [REQ-records-002]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:25:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a1f4735a820d, observed 2026-11-13T04:25:53Z. The raw log stays in the machine-local store.
