---
kind: evidence
id: 2026-11-18-ran-tests-no-hard-links-rs-new-records-are-written-on-a-volume
aliases: []
claim: "Ran tests/no_hard_links.rs::new_records_are_written_on_a_volume_without_hard_links by hand on a 64 MB exFAT image at about 08:50 on 2026-11-18, before it was marked ignored: with main's writer chough init failed with 'Operation not supported (os error 45)'; with the revert init, bug add and journal add passed (1 passed, 3.08s). A second run at 09:21 hung in hdiutil attach and was stopped, so it gave no result."
result: passed
evidence_kind: product-test
revision: fc96d233cf1712cebbe1dd3731b688fff165d421
content: b45755596385ee5f8bff5c2085b75705c28dd9e69d31452c41c26fcb2723a4f0
environment: local-macos-arm64
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: []
limitations:
  - The passing run was not repeated after the test was marked ignored; hdiutil was hanging on this machine
observed: 2026-11-18T16:24:14Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran tests/no_hard_links.rs::new_records_are_written_on_a_volume_without_hard_links by hand on a 64 MB exFAT image at about 08:50 on 2026-11-18, before it was marked ignored: with main's writer chough init failed with 'Operation not supported (os error 45)'; with the revert init, bug add and journal add passed (1 passed, 3.08s). A second run at 09:21 hung in hdiutil attach and was stopped, so it gave no result.**

Recorded as passed in local-macos-arm64 at revision fc96d233cf17, observed 2026-11-18T16:24:14Z.
