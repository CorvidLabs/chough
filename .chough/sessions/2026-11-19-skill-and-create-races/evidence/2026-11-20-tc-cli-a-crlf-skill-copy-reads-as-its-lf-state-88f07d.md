---
kind: evidence
id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-88f07d
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
duration_seconds: 0
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:54d33ada09042ffcc22b4a40234abbc397da88eb62ca6f9a991f7250be4c5943
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:59:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T14:59:47Z. The raw log stays in the machine-local store.
