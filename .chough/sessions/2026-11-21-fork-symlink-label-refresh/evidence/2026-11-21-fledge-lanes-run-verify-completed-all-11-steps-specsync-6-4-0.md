---
kind: evidence
id: 2026-11-21-fledge-lanes-run-verify-completed-all-11-steps-specsync-6-4-0
aliases: []
claim: fledge lanes run verify completed all 11 steps. SpecSync 6.4.0 and Fledge 1.8.0 were placed first on PATH because the copies found first on this machine were newer than the CI pins.
result: passed
evidence_kind: structural
revision: 178b66a1607e3d830bbae4de41ce33996f057ef7
content: fa88d8509c33bb33f34fb1a4445868ab8afad994e194116155fdd1d326830d07
environment: local-macos-arm64
criteria: [LIFE-31]
requirements: [REQ-cli-070]
contracts: []
limitations:
  - The lane ran before the follow-up commits. change check then materialized the spec and recorded verification without changing the label workflow, the script, or the tests.
observed: 2026-11-21T17:36:05Z
actor:
  - terminal: t10j
---
**fledge lanes run verify completed all 11 steps. SpecSync 6.4.0 and Fledge 1.8.0 were placed first on PATH because the copies found first on this machine were newer than the CI pins.**

Recorded as passed in local-macos-arm64 at revision 178b66a1607e, observed 2026-11-21T17:36:05Z.
