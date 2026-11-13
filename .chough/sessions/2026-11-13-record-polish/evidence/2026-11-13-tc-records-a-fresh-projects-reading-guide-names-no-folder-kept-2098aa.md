---
kind: evidence
id: 2026-11-13-tc-records-a-fresh-projects-reading-guide-names-no-folder-kept-2098aa
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
revision: f9b22f2215f9f61de538bbc17c45829385438cba
content: 84a021f93c1361a396a48300717a2b72419f9657749cffc84f701402a1bc2d9a
environment: local-macos-arm64
fingerprint: sha256:681d50e5d978e708cf89dbb0a1b69bdff2f856fb999631bc9bfd74d9f1c87271
criteria: [DRIVE-24, KNOW-32]
requirements: [REQ-records-002]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:30:27Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test records -- --exact a_fresh_projects_reading_guide_names_no_folder_kept_from_0_5 for TC-records-a-fresh-projects-reading-guide-names-no-folder-kept-from-0-5: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f9b22f2215f9, observed 2026-11-13T03:30:27Z. The raw log stays in the machine-local store.
