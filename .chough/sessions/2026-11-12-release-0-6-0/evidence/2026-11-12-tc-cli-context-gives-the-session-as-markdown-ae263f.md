---
kind: evidence
id: 2026-11-12-tc-cli-context-gives-the-session-as-markdown-ae263f
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact context_gives_the_session_as_markdown for TC-cli-context-gives-the-session-as-markdown: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-context-gives-the-session-as-markdown]
case_digest: [af75a0edcc2147b76f8f22d7036fcdb1721e4ef9798f4a5f1dc15f98527769e4]
command: [cargo, test, --locked, --test, cli, --, --exact, context_gives_the_session_as_markdown]
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 421a7034dfce0c052d48c4d1fb5abe041c272a46
content: 9e16bb2234c3aadbf27905868186d8ca01d209615750221de4f1f7201e15af54
environment: local-macos-arm64
fingerprint: sha256:07974c22410ff3a889c120df6a79df4bea5c39d457e09142a64670abcb2f4e9b
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:05:41Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_session_as_markdown for TC-cli-context-gives-the-session-as-markdown: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 421a7034dfce, observed 2026-11-12T23:05:41Z. The raw log stays in the machine-local store.
