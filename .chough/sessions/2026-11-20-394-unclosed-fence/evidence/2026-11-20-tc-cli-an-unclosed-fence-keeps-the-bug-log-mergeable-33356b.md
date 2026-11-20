---
kind: evidence
id: 2026-11-20-tc-cli-an-unclosed-fence-keeps-the-bug-log-mergeable-33356b
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
revision: 5b05c2d86c43595eee276a8262feaf3d29df4ea0
content: f5edfe0dcfdabed26a44ae2088035d282d40efcb6945f4033385d9b7c5fb5fd9
environment: local-macos-arm64
fingerprint: sha256:76270b284f424815ca1cabe0a21842f95f6e43d07fff6eb3e2c038358ab29531
criteria: [DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:51:14Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test bugs -- --exact a_bug_with_an_unclosed_fence_still_merges_both_log_entries for TC-cli-an-unclosed-fence-keeps-the-bug-log-mergeable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 5b05c2d86c43, observed 2026-11-20T17:51:14Z. The raw log stays in the machine-local store.
