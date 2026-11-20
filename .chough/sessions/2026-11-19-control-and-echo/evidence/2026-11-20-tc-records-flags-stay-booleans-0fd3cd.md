---
kind: evidence
id: 2026-11-20-tc-records-flags-stay-booleans-0fd3cd
aliases: []
claim: "Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-flags-stay-booleans]
case_digest: [c2b89146c46ce1e5388d7923d71461021eec2dc8c20f47431877535d395d0dff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - checkpoint
  - "--"
  - "--exact"
  - flags_stay_booleans_and_either_form_still_reads
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:cf8188ba886057bbd9dc636ff90ce33938eb56a6417d1cb95180a075a08f553c
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:12:23Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:12:23Z. The raw log stays in the machine-local store.
