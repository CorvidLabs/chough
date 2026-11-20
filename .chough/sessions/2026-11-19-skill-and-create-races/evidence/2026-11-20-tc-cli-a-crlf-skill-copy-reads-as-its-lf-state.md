---
kind: evidence
id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state
aliases: []
claim: "Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-crlf-skill-copy-reads-as-its-lf-state]
case_digest: [84db714666013ed423ef083c32b53a0f123a47d64ec214ec4baacc0c69799225]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cli
  - "--"
  - "--exact"
  - a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:a231ed7cf6ae285943dac65cc32d1207703001c8336f18896de95be9b97d6136
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:32Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:32Z. The raw log stays in the machine-local store.
