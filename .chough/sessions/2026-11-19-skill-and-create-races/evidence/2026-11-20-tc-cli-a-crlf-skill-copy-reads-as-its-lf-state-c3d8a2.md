---
kind: evidence
id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-c3d8a2
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
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:d54aa0aa0bf3293354e9cfffb83b80b7bc362ed54096350b2a17f5b864438a5f
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:35:42Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:35:42Z. The raw log stays in the machine-local store.
