---
kind: evidence
id: 2026-11-20-tc-cli-an-unclosed-fence-keeps-the-bug-log-mergeable-07de99
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
revision: b4e552e1d3a7cfd04506c85af555dbf7a646c8ea
content: 482d038fb3051d2afae72aac95127e191b50f665ad67272b09a9800bb3e8c20b
environment: local-macos-arm64
fingerprint: sha256:4932cc96bca8104e52fd79c2d33c3b4a519555c4917e8b74a6a8d08cb58714b9
criteria: [DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:34:48Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test bugs -- --exact a_bug_with_an_unclosed_fence_still_merges_both_log_entries for TC-cli-an-unclosed-fence-keeps-the-bug-log-mergeable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b4e552e1d3a7, observed 2026-11-20T18:34:48Z. The raw log stays in the machine-local store.
