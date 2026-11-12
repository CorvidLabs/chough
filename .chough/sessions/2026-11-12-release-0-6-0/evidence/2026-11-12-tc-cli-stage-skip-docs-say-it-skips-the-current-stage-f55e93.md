---
kind: evidence
id: 2026-11-12-tc-cli-stage-skip-docs-say-it-skips-the-current-stage-f55e93
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
revision: 06407b98e650d73a951f505369cb7c694765ffde
content: 20c9710c0555251958526a8d2c4296aeac3f42743b807b3fecd9dd54614bf1da
environment: local-macos-arm64
fingerprint: sha256:390582cf11e5775db904ded7b6759eabeb1a98519b684d0373610fb7e6641930
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:20:12Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact stage_skip_docs_say_it_skips_the_current_stage_and_how_to_skip_design for TC-cli-stage-skip-docs-say-it-skips-the-current-stage: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 06407b98e650, observed 2026-11-12T21:20:12Z. The raw log stays in the machine-local store.
