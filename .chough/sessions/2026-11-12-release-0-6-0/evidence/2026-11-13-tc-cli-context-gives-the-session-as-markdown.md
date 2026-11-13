---
kind: evidence
id: 2026-11-13-tc-cli-context-gives-the-session-as-markdown
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
duration_seconds: 2
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:07974c22410ff3a889c120df6a79df4bea5c39d457e09142a64670abcb2f4e9b
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:24:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_session_as_markdown for TC-cli-context-gives-the-session-as-markdown: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:24:16Z. The raw log stays in the machine-local store.
