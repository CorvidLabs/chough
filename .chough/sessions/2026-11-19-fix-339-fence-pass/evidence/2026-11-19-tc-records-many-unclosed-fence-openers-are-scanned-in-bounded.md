---
kind: evidence
id: 2026-11-19-tc-records-many-unclosed-fence-openers-are-scanned-in-bounded
aliases: []
claim: "Ran cargo test --locked --test records -- --exact many_unclosed_fence_openers_are_scanned_in_bounded_time for TC-records-many-unclosed-fence-openers-are-scanned-in-bounded-time: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-many-unclosed-fence-openers-are-scanned-in-bounded-time]
case_digest: [4da28086c5de787f61fb57dadc3ff20a0c4e9696823d9de41e6573be8066c9c2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - records
  - "--"
  - "--exact"
  - many_unclosed_fence_openers_are_scanned_in_bounded_time
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 4f74fa25cd491a82518ff5aead551e4814f3e043
content: 9f356f9326eedc20a0cb6c07e4dee6e014efc412386eb573f09bdec9a54197b6
environment: local-macos-arm64
fingerprint: sha256:4e15f77c11eaa0612f87b4452e5e8c4bcc7f404060338f8514633ff54698db27
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T01:08:04Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact many_unclosed_fence_openers_are_scanned_in_bounded_time for TC-records-many-unclosed-fence-openers-are-scanned-in-bounded-time: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4f74fa25cd49, observed 2026-11-19T01:08:04Z. The raw log stays in the machine-local store.
