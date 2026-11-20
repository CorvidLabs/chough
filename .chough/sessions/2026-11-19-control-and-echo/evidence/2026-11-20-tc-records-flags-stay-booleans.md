---
kind: evidence
id: 2026-11-20-tc-records-flags-stay-booleans
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
revision: d8aa8a63acd5bca47a65e589bb069f2251730b53
content: f6cfee0c808bba0d07d4bc2685239aab35a594977879eebe450153e3399173d7
environment: local-macos-arm64
fingerprint: sha256:72e386f0e7398fc25807919c855be2b2ad2771a2e8a5e7ed92811e173803d1c3
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:21:32Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d8aa8a63acd5, observed 2026-11-20T03:21:32Z. The raw log stays in the machine-local store.
