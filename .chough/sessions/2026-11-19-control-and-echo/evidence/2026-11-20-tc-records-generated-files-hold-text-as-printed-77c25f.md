---
kind: evidence
id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-77c25f
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-generated-files-hold-text-as-printed]
case_digest: [cdf17d5ad26155010759ca01fa49e266001d10da788d705e387c20ae3d353b72]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - generated_files_hold_stored_control_characters_as_printed
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:383df9f90997867fe68aa102967f705755aca156e9e9ceeb41057001b9632129
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:13Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:13Z. The raw log stays in the machine-local store.
