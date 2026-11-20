---
kind: evidence
id: 2026-11-20-tc-history-doctor-reads-uncommitted-records-375f68
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-reads-uncommitted-records]
case_digest: [ffa17dc175342e495e3f67aee544085353a8ffc9b06e24f88e5838113f777372]
command: [cargo, test, --locked, --test, doctor, --, --exact, doctor_reads_uncommitted_records]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:feb14ec337291f3cc6006037887654700376b3cc660894acba4a3fd88f18a07f
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:00:10Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_reads_uncommitted_records for TC-history-doctor-reads-uncommitted-records: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T15:00:10Z. The raw log stays in the machine-local store.
