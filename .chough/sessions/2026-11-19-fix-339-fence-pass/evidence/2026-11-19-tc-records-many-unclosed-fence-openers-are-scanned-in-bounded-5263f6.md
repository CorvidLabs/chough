---
kind: evidence
id: 2026-11-19-tc-records-many-unclosed-fence-openers-are-scanned-in-bounded-5263f6
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
revision: eb828bedb11ed25410cf3961188a17cd516f2007
content: d8e7328df6aad290260d31c16bff20b466380b238047da93fd7359a1b61dbc71
environment: local-macos-arm64
fingerprint: sha256:f9a62ed229cdc04cdab2baf08725a766869960092c72b9a8212bf103fecbc5ff
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T01:39:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test records -- --exact many_unclosed_fence_openers_are_scanned_in_bounded_time for TC-records-many-unclosed-fence-openers-are-scanned-in-bounded-time: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision eb828bedb11e, observed 2026-11-19T01:39:00Z. The raw log stays in the machine-local store.
