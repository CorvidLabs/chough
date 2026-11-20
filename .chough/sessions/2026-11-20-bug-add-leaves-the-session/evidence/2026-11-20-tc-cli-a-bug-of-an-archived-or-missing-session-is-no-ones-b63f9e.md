---
kind: evidence
id: 2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones-b63f9e
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones]
case_digest: [2b866350a4c2ce5a8a9a45933c8c00a6a2e5b76512703347e027e335c6cae80e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
environment: local-macos-arm64
fingerprint: sha256:a5880167289511fc9bc52070e195cf8c12c122455c198d14feaea92433fbd589
criteria: [LIFE-5]
requirements: [REQ-cli-012, REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:34:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 04c4b86f441c, observed 2026-11-20T16:34:47Z. The raw log stays in the machine-local store.
