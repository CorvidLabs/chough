---
kind: evidence
id: 2026-11-13-tc-cli-integrations-templates-853b5f
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-templates]
case_digest: [3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 12f2cafe777a48ef6e6cb92c9435997ab5b4e0d2
content: dd44a2f14e6358d057ef934649d20ff96745157b0280619cc43ef2f0e973b2c4
environment: local-macos-arm64
fingerprint: sha256:6808d5d41d08f689f36dac300d50be2d13bd401f2d8a45b9f969556c83f2106c
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:14:00Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 12f2cafe777a, observed 2026-11-13T04:14:00Z. The raw log stays in the machine-local store.
