---
kind: evidence
id: 2026-11-13-tc-cli-context-gives-the-timeline-and-bugs-as-markdown-b1f994
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:84ea1b0cb0df5b160933f0559cbaf1165da8a36d6a3fdc064e6f97199130b0f2
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:51:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cli -- --exact context_gives_the_timeline_and_open_bugs_as_markdown for TC-cli-context-gives-the-timeline-and-bugs-as-markdown: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:51:33Z. The raw log stays in the machine-local store.
