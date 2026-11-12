---
kind: evidence
id: 2026-11-12-tc-cli-context-gives-the-session-as-markdown
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact context_gives_the_session_as_markdown for TC-cli-context-gives-the-session-as-markdown: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-context-gives-the-session-as-markdown]
case_digest: [af75a0edcc2147b76f8f22d7036fcdb1721e4ef9798f4a5f1dc15f98527769e4]
command: [cargo, test, --locked, --test, cli, --, --exact, context_gives_the_session_as_markdown]
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
content: 073d7330cf258fdd25b33d439e35f9344842f8fdfa2a1cd81c388ab4af69e3ca
environment: local-macos-arm64
fingerprint: sha256:4c81a71b516de05de7bdfc5f4623c9e9fe100950681d45c40269304111d3bbcb
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:04:06Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_session_as_markdown for TC-cli-context-gives-the-session-as-markdown: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 4e245263fe4e, observed 2026-11-12T23:04:06Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- context_gives_the_session_as_markdown stdout ----
- thread 'context_gives_the_session_as_markdown' (927438029) panicked at tests/cli.rs:3164:5:
- the Session section is JSON:
- {
-   "awaiting_human": [
-     "SYNTHETIC intent gate"
-   ],
-   "blockers": [],
-   "bugs": [],
-   "completed": [
-     "SYNTHETIC: wrote the parser"
-   ],
-   "decisions": [
-     "SYNTHETIC: use TOML"
-   ],
-   "intent_scope": "LIFE",
-   "modules": [
-     "cli"
-   ],
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 57 filtered out; finished in 0.71s
