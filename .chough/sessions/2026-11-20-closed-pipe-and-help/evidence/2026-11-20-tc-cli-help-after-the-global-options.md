---
kind: evidence
id: 2026-11-20-tc-cli-help-after-the-global-options
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
revision: be53e668fdf94ba61ff800b3702fc70d89517b82
content: 34322c35469b860b51450cad40a3b0e14c5024c1f945e7dbebfcf2ce4563c892
environment: local-macos-arm64
fingerprint: sha256:fdf1eb4b141e701d432645c8cc4aae21688c38f36f5d5f3e78d00748a4edd180
criteria: [LIFE-4]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:44:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact help_after_the_global_options_prints_the_top_level_help for TC-cli-help-after-the-global-options: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision be53e668fdf9, observed 2026-11-20T16:44:31Z. The raw log stays in the machine-local store.
