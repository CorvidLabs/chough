---
kind: evidence
id: 2026-11-13-tc-cli-stage-skip-docs-say-it-skips-the-current-stage
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-skip-docs-say-it-skips-the-current-stage]
case_digest: [3d6f5c15e943a0c26a1fb5d37e5be324a5a58f319890b5879729edb6ff45d13a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:9d7f9fa051573c68bb9d4135c2e9d830bc812262a1d12ed03e470102251178e8
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:19:32Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:19:32Z. The raw log stays in the machine-local store.
