---
kind: evidence
id: 2026-11-12-tc-cli-init-claude-host-prints-the-claude-md-line-f074cf
aliases: []
claim: "Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-init-claude-host-prints-the-claude-md-line]
case_digest: [3eac1ceb4d2119359f445c1f9d95150a4644fb4c90fca9be1a964dafedff4106]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - optional
  - "--"
  - "--exact"
  - init_with_the_claude_host_prints_the_claude_md_line_and_writes_none
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:f8fa46e728056db018dc9a2ebfaab3fc1b7bf4d0db5f74c76ffeb58706c47468
criteria: [LIFE-12, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:25:04Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:25:04Z. The raw log stays in the machine-local store.
