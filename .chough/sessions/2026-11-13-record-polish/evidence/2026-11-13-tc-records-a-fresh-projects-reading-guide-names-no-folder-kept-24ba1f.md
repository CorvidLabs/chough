---
kind: evidence
id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-24ba1f
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
duration_seconds: 0
revision: 6b7cd4766a52829847a23d03d0d93262d70feeee
content: 49a2ab3c28dbd1ec2012ee1e52591ba22d6e01e8918fc4976e1aaf668257c7a5
environment: local-macos-arm64
fingerprint: sha256:ad7dedf780b62e53cd3758fd03c0005d40fd5574337547620d9471082ced96c9
criteria: [DRIVE-24, KNOW-32]
requirements: [REQ-records-002]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:12:28Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6b7cd4766a52, observed 2026-11-13T03:12:28Z. The raw log stays in the machine-local store.
