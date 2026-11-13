---
kind: evidence
id: 2026-11-13-tc-cli-context-gives-the-timeline-and-bugs-as-markdown-38a627
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:5408c5e9edc7bd6cf5b90c9c9605fcc932df296552a21564ae939e378ea0c328
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:19:18Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_timeline_and_open_bugs_as_markdown for TC-cli-context-gives-the-timeline-and-bugs-as-markdown: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:19:18Z. The raw log stays in the machine-local store.
