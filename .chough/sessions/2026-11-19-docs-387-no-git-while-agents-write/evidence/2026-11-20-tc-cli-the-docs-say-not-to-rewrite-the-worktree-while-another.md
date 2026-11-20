---
kind: evidence
id: 2026-11-20-tc-cli-the-docs-say-not-to-rewrite-the-worktree-while-another
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
revision: 705c7a97f66d1c1152f87392259ecec13f3ce5b3
content: 80426c9eac481c1e21a8963db755e9a4f4f3a234b502d8a9912113b0a7c4cf66
environment: local-macos-arm64
fingerprint: sha256:19bd75f1e0742071771bfb46ffc6e923cac77d1c1d4907ade6345e00a18bbf7a
criteria: [DRIVE-11, DRIVE-3]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:57:22Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test docs -- --exact the_docs_say_not_to_rewrite_the_worktree_while_another_agent_works for TC-cli-the-docs-say-not-to-rewrite-the-worktree-while-another-agent-works: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 705c7a97f66d, observed 2026-11-20T00:57:22Z. The raw log stays in the machine-local store.
