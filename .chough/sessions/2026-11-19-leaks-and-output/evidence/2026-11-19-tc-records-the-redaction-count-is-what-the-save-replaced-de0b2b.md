---
kind: evidence
id: 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced-de0b2b
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-the-redaction-count-is-what-the-save-replaced]
case_digest: [64420d75b9474d13ef66f57850ee92362ed4092a98b5fd39f02439bb2ed6110d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - the_redaction_count_is_what_the_save_replaced
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:457e04749074e576c5fd335e26be09b5bc5a8ede248c44cb90156d54a3e3f3c9
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:23:36Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75f43060e66b, observed 2026-11-19T14:23:36Z. The raw log stays in the machine-local store.
