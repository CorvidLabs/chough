---
kind: evidence
id: 2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing-f04562
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
revision: 551db294729e36b9053c6983d6aa624ddb22b8de
content: b1409508a033fe2a4fe12ee5a8652a321ea8de65ad74db695a858a4ccdeb92c8
environment: local-macos-arm64
fingerprint: sha256:6ede0fb636a2e006b17d7a6abdfafc3203e0c669678da2b2af90879c511a10cb
criteria: [DRIVE-4]
requirements: [REQ-records-014]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T21:38:22Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 551db294729e, observed 2026-11-18T21:38:22Z. The raw log stays in the machine-local store.
