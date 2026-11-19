---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-bug-reference-only-when-clean-a69367
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-bug-reference-only-when-clean]
case_digest: [b551e5664a4de4ef71fc2047215e4b87b922d4d79044eefdc407da05c1d77550]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_reference_is_shown_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 402a3a5d4ae91b332f3bfbc86fd0661c5860a997
content: 265ddf43d047aa35119f882a9ef474d35d96af50d2b5a4a0230489d2076c36b9
environment: local-macos-arm64
fingerprint: sha256:9f1e3c564443b618b460b911051367eb860437953fb53d9eb761e670324e62b4
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:44:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_reference_is_shown_only_when_clean for TC-records-errors-show-a-bug-reference-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 402a3a5d4ae9, observed 2026-11-19T02:44:34Z. The raw log stays in the machine-local store.
