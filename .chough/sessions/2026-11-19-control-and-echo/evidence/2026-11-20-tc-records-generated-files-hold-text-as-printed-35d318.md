---
kind: evidence
id: 2026-11-20-tc-records-generated-files-hold-text-as-printed-35d318
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
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:bcf138f69519bbab98b23831d1527fb0ec4326dad8f786c57e48743d435833ca
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:12:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact generated_files_hold_stored_control_characters_as_printed for TC-records-generated-files-hold-text-as-printed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:12:18Z. The raw log stays in the machine-local store.
