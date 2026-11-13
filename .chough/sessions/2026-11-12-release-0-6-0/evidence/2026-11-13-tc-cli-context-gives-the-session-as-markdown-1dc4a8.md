---
kind: evidence
id: 2026-11-13-tc-cli-context-gives-the-session-as-markdown-1dc4a8
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:36c33594126f81de6abcd77d40e7b9d68a9b8a543966a288a27539b37a6434de
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:51:04Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_session_as_markdown for TC-cli-context-gives-the-session-as-markdown: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:51:04Z. The raw log stays in the machine-local store.
