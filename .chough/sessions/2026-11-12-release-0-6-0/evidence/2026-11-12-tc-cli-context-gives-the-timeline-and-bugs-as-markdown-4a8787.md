---
kind: evidence
id: 2026-11-12-tc-cli-context-gives-the-timeline-and-bugs-as-markdown-4a8787
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact context_gives_the_timeline_and_open_bugs_as_markdown for TC-cli-context-gives-the-timeline-and-bugs-as-markdown: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 421a7034dfce0c052d48c4d1fb5abe041c272a46
content: 9e16bb2234c3aadbf27905868186d8ca01d209615750221de4f1f7201e15af54
environment: local-macos-arm64
fingerprint: sha256:ef946762d6cc0ae72221085eedf6fc1bf7975dc2668376a2e52a9e2bf4277a4a
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:05:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_timeline_and_open_bugs_as_markdown for TC-cli-context-gives-the-timeline-and-bugs-as-markdown: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 421a7034dfce, observed 2026-11-12T23:05:54Z. The raw log stays in the machine-local store.
