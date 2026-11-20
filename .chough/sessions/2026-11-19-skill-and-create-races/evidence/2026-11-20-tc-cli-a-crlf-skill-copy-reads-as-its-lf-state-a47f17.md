---
kind: evidence
id: 2026-11-20-tc-cli-a-crlf-skill-copy-reads-as-its-lf-state-a47f17
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
revision: 29fb39131c6435612182f52929b94e69ab0dcba3
content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
environment: local-macos-arm64
fingerprint: sha256:89538eeb371bffdc239c27e385d72e4798602692c5d496b45c916aa66b61aa87
criteria: [FLOW-1, LIFE-4]
requirements: [REQ-cli-016]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:19:21Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test cli -- --exact a_crlf_skill_copy_reads_as_its_lf_state_and_is_refreshed_with_crlf for TC-cli-a-crlf-skill-copy-reads-as-its-lf-state: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 29fb39131c64, observed 2026-11-20T14:19:21Z. The raw log stays in the machine-local store.
