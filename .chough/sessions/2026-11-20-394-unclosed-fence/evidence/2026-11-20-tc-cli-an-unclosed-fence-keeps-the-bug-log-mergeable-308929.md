---
kind: evidence
id: 2026-11-20-tc-cli-an-unclosed-fence-keeps-the-bug-log-mergeable-308929
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact a_bug_with_an_unclosed_fence_still_merges_both_log_entries for TC-cli-an-unclosed-fence-keeps-the-bug-log-mergeable: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-an-unclosed-fence-keeps-the-bug-log-mergeable]
case_digest: [3e0f58982cc3900167e3f1d7d67534ae57521b138a4382cde00916e837e98f03]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - a_bug_with_an_unclosed_fence_still_merges_both_log_entries
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: f851c62d38e6a1e46f2abc92fdc9190b02bf31fd
content: 66ec4cbf7168fd59132e9ead6fc36fb3dced1359a0030dd4115f187abd136775
environment: local-macos-arm64
fingerprint: sha256:4d5014a50172b2f1ecca15cd818d80ccb22dfec81233a4fcb8c74489f620d8b6
criteria: [DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:15:11Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test bugs -- --exact a_bug_with_an_unclosed_fence_still_merges_both_log_entries for TC-cli-an-unclosed-fence-keeps-the-bug-log-mergeable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f851c62d38e6, observed 2026-11-20T16:15:11Z. The raw log stays in the machine-local store.
