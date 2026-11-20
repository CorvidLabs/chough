---
kind: evidence
id: 2026-11-20-tc-records-flags-stay-booleans-5fee50
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
revision: cdc0d2e2f85a6d09567b8780662656ccec551526
content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
environment: local-macos-arm64
fingerprint: sha256:44d33439157bfb05c31da9d6174e039d7926db7713ab20cee1f1897d1e50c81d
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:34:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cdc0d2e2f85a, observed 2026-11-20T04:34:14Z. The raw log stays in the machine-local store.
