---
kind: evidence
id: 2026-11-20-tc-records-flags-stay-booleans-c8cc8a
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
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:44d33439157bfb05c31da9d6174e039d7926db7713ab20cee1f1897d1e50c81d
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:26:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:26:31Z. The raw log stays in the machine-local store.
