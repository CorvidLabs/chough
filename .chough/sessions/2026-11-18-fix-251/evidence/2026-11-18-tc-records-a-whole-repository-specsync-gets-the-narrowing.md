---
kind: evidence
id: 2026-11-18-tc-records-a-whole-repository-specsync-gets-the-narrowing
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
revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
content: 300762638d04d625e69acb16ab5a414a2bcb802eb4953618963487475bc41448
environment: local-macos-arm64
fingerprint: sha256:afced5cde34aafeca4176050c063c4b5e84dfd72e1c4b0589a55a57d2c925ac4
criteria: [DRIVE-4]
requirements: [REQ-records-014]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:54:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 66efea4dd777, observed 2026-11-18T20:54:34Z. The raw log stays in the machine-local store.
