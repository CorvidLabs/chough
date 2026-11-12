---
kind: evidence
id: 2026-11-12-tc-cli-context-gives-the-timeline-and-bugs-as-markdown
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact context_gives_the_timeline_and_open_bugs_as_markdown for TC-cli-context-gives-the-timeline-and-bugs-as-markdown: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-context-gives-the-timeline-and-bugs-as-markdown]
case_digest: [4e04ba922768c9140a77b9619c0ddf8b1847bc0e920b556b5dfed4a7eb7743a9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - context_gives_the_timeline_and_open_bugs_as_markdown
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
content: 073d7330cf258fdd25b33d439e35f9344842f8fdfa2a1cd81c388ab4af69e3ca
environment: local-macos-arm64
fingerprint: sha256:f7470c5540945d352a16e9883f692a9bd8957d11e8a4e2711a981348419041c6
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:04:19Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_timeline_and_open_bugs_as_markdown for TC-cli-context-gives-the-timeline-and-bugs-as-markdown: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 4e245263fe4e, observed 2026-11-12T23:04:19Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- context_gives_the_timeline_and_open_bugs_as_markdown stdout ----
- thread 'context_gives_the_timeline_and_open_bugs_as_markdown' (927439350) panicked at tests/cli.rs:3229:5:
- the Timeline section is JSON:
- [
-   {
-     "at_unix_seconds": 1794524657,
-     "by": {
-       "agent": "claude-code",
-       "terminal": "t382"
-     },
-     "from": "intent",
-     "gate": null,
-     "id": "t-16cc92",
-     "kind": "start",
-     "reason": "Fix a reported bug",
-     "refresh": [],
-     "revision": null,
-     "schema_version": 1,
-     "to": "intent"
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 57 filtered out; finished in 0.67s
