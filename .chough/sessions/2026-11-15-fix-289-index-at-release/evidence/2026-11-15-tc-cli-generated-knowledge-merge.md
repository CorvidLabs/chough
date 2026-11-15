---
kind: evidence
id: 2026-11-15-tc-cli-generated-knowledge-merge
aliases: []
claim: "Ran cargo test --locked --test journal -- two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver library_merge_keeps_ours_and_refuses_a_missing_file for TC-cli-generated-knowledge-merge: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-generated-knowledge-merge]
case_digest: [0346e0936854daca606069c14dd376226479d780570cd92c5d1aaae210d041b7]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver
  - library_merge_keeps_ours_and_refuses_a_missing_file
exit_code: 0
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:cb0ad0dddc685e69462f979ed1b9bf2a580591cca7124f25f1284dc15f31dcc9
criteria: [DRIVE-12, KNOW-33]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-15T20:22:07Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test journal -- two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver library_merge_keeps_ours_and_refuses_a_missing_file for TC-cli-generated-knowledge-merge: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:07Z. The raw log stays in the machine-local store.
