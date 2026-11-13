---
kind: evidence
id: 2026-11-13-tc-cli-init-claude-host-prints-the-claude-md-line
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
duration_seconds: 2
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:9e47f0949b01fb97b7c28ec1971715a87f70a1d8b269decffc5447e3f09e2433
criteria: [LIFE-12, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:22:23Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test optional -- --exact init_with_the_claude_host_prints_the_claude_md_line_and_writes_none for TC-cli-init-claude-host-prints-the-claude-md-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:22:23Z. The raw log stays in the machine-local store.
