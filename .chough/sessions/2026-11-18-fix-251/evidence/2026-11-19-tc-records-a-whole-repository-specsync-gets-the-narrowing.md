---
kind: evidence
id: 2026-11-19-tc-records-a-whole-repository-specsync-gets-the-narrowing
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-whole-repository-specsync-gets-the-narrowing-advice]
case_digest: [51836e0878629078a5df3ccdfa23351bfa14a8e1281a6bf3a96fe0804edeabd0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_whole_repository_specsync_gets_the_narrowing_advice
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 509be0742ff459b0e50801b48d8e8f30b5be3684
content: 0d67dd251a29abd163b1049dab9043540c189129a1b58f6db86286cef980fac5
environment: local-macos-arm64
fingerprint: sha256:caf9362edb536590b53b9c4fff9270332853bf696f504fc0575216cac7b4e3a8
criteria: [DRIVE-4]
requirements: [REQ-records-014]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T00:28:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 509be0742ff4, observed 2026-11-19T00:28:13Z. The raw log stays in the machine-local store.
