---
kind: evidence
id: 2026-11-18-tc-cli-journal-add-holds-the-writer-lock
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact journal_add_waits_for_the_writer_lock for TC-cli-journal-add-holds-the-writer-lock: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-journal-add-holds-the-writer-lock]
case_digest: [1d8c3f333026283a2038d72c8a87551964c1f90d5ddc4fcef5416c65c09c2f58]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - journal_add_waits_for_the_writer_lock
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
content: 932d46f3b7b6ddc804d56c1863d37fb498ce4a4ebf3fd8e8a1dc2b2770ab0270
environment: local-macos-arm64
fingerprint: sha256:016425ac95ffcb563346ce34d352d7242aad7863ae620c3cc8bb9b474982f54b
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T00:36:30Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test journal -- --exact journal_add_waits_for_the_writer_lock for TC-cli-journal-add-holds-the-writer-lock: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8abb5bb89b52, observed 2026-11-18T00:36:30Z. The raw log stays in the machine-local store.
