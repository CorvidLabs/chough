---
kind: evidence
id: 2026-11-12-tc-cli-context-reads-agents-md-from-the-git-root-bad891
aliases: []
claim: "Ran cargo test --locked --test subdirectory -- --exact context_reads_agents_md_from_the_git_root_down_only for TC-cli-context-reads-agents-md-from-the-git-root: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-context-reads-agents-md-from-the-git-root]
case_digest: [4cc31020a8c0670707d9224e0f69aea72b38861fd0319f474ff668f3429896ce]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - subdirectory
  - "--"
  - "--exact"
  - context_reads_agents_md_from_the_git_root_down_only
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 7be760ccbc7da0fbb0ebe1a63ff562e5782ae91b
content: 2bc6f78c658c710d8247ff73e0cc5320123d1398568de992f6a8852e275b18e1
environment: local-macos-arm64
fingerprint: sha256:2a99c74d06d796dc83ad79e8bdb3ec8e9d856f2c1441bd29687a304034bf5673
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:44:58Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test subdirectory -- --exact context_reads_agents_md_from_the_git_root_down_only for TC-cli-context-reads-agents-md-from-the-git-root: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7be760ccbc7d, observed 2026-11-12T17:44:58Z. The raw log stays in the machine-local store.
