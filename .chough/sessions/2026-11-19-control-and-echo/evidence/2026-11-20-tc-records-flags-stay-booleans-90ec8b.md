---
kind: evidence
id: 2026-11-20-tc-records-flags-stay-booleans-90ec8b
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:b3c28d2abcf2dc2c1bcb8351a542cc223e377496e2b32f0cb053ff1924c5ea69
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:17Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact flags_stay_booleans_and_either_form_still_reads for TC-records-flags-stay-booleans: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:17Z. The raw log stays in the machine-local store.
