---
kind: evidence
id: 2026-11-13-tc-cli-context-gives-the-session-as-markdown-cfa019
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:d8a3c25812d97c0928bca695f3490dd6c2f46f9fb7ee9dd38e16d1815c75f76a
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:18:43Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_session_as_markdown for TC-cli-context-gives-the-session-as-markdown: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:18:43Z. The raw log stays in the machine-local store.
