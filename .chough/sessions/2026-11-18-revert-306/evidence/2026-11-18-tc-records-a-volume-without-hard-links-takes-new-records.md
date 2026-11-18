---
kind: evidence
id: 2026-11-18-tc-records-a-volume-without-hard-links-takes-new-records
aliases: []
claim: "Verdicts 1=pass. Ran the test by hand on a 64 MB exFAT image at about 08:50 on 2026-11-18, before it was marked ignored: with main's writer chough init failed with 'Operation not supported (os error 45)'; with the revert init, bug add and journal add passed (1 passed, 3.08s)."
result: passed
evidence_kind: human-observation
method: manual
track: functional
case: [TC-records-a-volume-without-hard-links-takes-new-records]
case_digest: [db56acd340062f115536a8b7a72100af9b919fa10040f13e4252cf475ef30190]
revision: 922f70c7548ee108787caae16219d340c69d9cf3
content: b45755596385ee5f8bff5c2085b75705c28dd9e69d31452c41c26fcb2723a4f0
environment: local-macos-arm64
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations:
  - A second run at 09:21 hung in hdiutil attach and was stopped; the passing run was not repeated after the test was marked ignored
observed: 2026-11-18T16:28:22Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Verdicts 1=pass. Ran the test by hand on a 64 MB exFAT image at about 08:50 on 2026-11-18, before it was marked ignored: with main's writer chough init failed with 'Operation not supported (os error 45)'; with the revert init, bug add and journal add passed (1 passed, 3.08s).**

Recorded as passed in local-macos-arm64 at revision 922f70c7548e, observed 2026-11-18T16:28:22Z.
