---
kind: evidence
id: 2026-11-20-tc-cli-help-after-the-global-options-7ca4da
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact help_after_the_global_options_prints_the_top_level_help for TC-cli-help-after-the-global-options: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-help-after-the-global-options]
case_digest: [759743d0e3b8c4d67ef3022bd26a08242bcf9dcc26aa33b1251a84d580abdeb6]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - help_after_the_global_options_prints_the_top_level_help
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e60f19787d96c9a0ebcf9399d93a04939d4696fb
content: 1640f508756b07e5571731954b75bcad62ee1ad2bddb05f87a5dd462354e2c55
environment: local-macos-arm64
fingerprint: sha256:839e0a1c2692ad412ef97047949174d0d3ae8fc35a1dc47a90da239e2de03ec9
criteria: [LIFE-4]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:07:21Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact help_after_the_global_options_prints_the_top_level_help for TC-cli-help-after-the-global-options: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e60f19787d96, observed 2026-11-20T17:07:21Z. The raw log stays in the machine-local store.
