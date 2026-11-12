---
kind: evidence
id: 2026-11-11-tc-history-165-earlier-archives-stay-9ac9bd
aliases: [OBS-tc-history-165-earlier-archives-stay-e355ee5b96eaa89fb762f797]
claim: "Ran cargo test --locked --test sessions -- --exact done_leaves_earlier_archived_stories_in_place for TC-history-165-earlier-archives-stay: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-165-earlier-archives-stay]
case_digest: [a3f2daf09cfe4288c7be3bb2aec33abb2536dfea4d088fe4a72cf3ee4e33aad6]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - done_leaves_earlier_archived_stories_in_place
exit_code: 0
duration_seconds: 1
revision: ec8f8a6f6162b324850d551a37cfc56f9c9d5d5a
content: 3f6ba540266bffc9c15f3048cf7f301fe63daa7ddad1a87fe86edfee6920b00c
environment: local-macos-arm64
fingerprint: sha256:5019e0e31eb19f509b981a488cae9cafca16503c527dac0af38b2113b1adce16
criteria: [KNOW-34, KNOW-30]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-11T18:16:57Z
---
**Ran cargo test --locked --test sessions -- --exact done_leaves_earlier_archived_stories_in_place for TC-history-165-earlier-archives-stay: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision ec8f8a6f6162, observed 2026-11-11T18:16:57Z. The raw log stays in the machine-local store.
