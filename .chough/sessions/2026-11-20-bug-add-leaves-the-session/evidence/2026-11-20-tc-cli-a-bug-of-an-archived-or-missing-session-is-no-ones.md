---
kind: evidence
id: 2026-11-20-tc-cli-a-bug-of-an-archived-or-missing-session-is-no-ones
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
duration_seconds: 1
revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
environment: local-macos-arm64
fingerprint: sha256:b9c74961c48ac9af1c0331fd668e56152002c191ccacd63af33f88a7afba3cb6
criteria: [LIFE-5]
requirements: [REQ-cli-012, REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:48:33Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact a_bug_of_an_archived_or_missing_session_is_listed_nowhere_and_raises_nothing for TC-cli-a-bug-of-an-archived-or-missing-session-is-no-ones: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2f99dcfb42f1, observed 2026-11-20T15:48:33Z. The raw log stays in the machine-local store.
