---
kind: evidence
id: 2026-11-20-tc-cli-an-unclosed-fence-keeps-the-bug-log-mergeable
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
revision: 13592d020e7f6e249d55666613b574b4c18dec1e
content: 5b20ce8c0fda513a4488deaa1aa90fec4c36e99f0101105a75b46cf64127b68b
environment: local-macos-arm64
fingerprint: sha256:a2f2304cf280cb17ed5477d67045736f26af10b85e848c90e5eb2349af94f01e
criteria: [DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:52:23Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test bugs -- --exact a_bug_with_an_unclosed_fence_still_merges_both_log_entries for TC-cli-an-unclosed-fence-keeps-the-bug-log-mergeable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 13592d020e7f, observed 2026-11-20T14:52:23Z. The raw log stays in the machine-local store.
