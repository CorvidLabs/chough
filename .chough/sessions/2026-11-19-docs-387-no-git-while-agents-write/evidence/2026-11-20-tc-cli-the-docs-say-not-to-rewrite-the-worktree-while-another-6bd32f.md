---
kind: evidence
id: 2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-6bd32f
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works for TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works]
case_digest: [564225552610950daaefdbc4ec93ef0dde54bff1caa408fe481de66a82d71c61]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 37255266c625664b9a5c3091c37806083dd9c438
content: 8186b3e3919e28e50b043943bccd0cb5a8e90296f6805bf14cb73b86e8bcacf3
environment: local-macos-arm64
fingerprint: sha256:251b9e6157a540247075521c5407e80d012991991fa49f74f86b1af78388c80b
criteria: [DRIVE-11, DRIVE-3]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T01:42:21Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works for TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37255266c625, observed 2026-11-20T01:42:21Z. The raw log stays in the machine-local store.
