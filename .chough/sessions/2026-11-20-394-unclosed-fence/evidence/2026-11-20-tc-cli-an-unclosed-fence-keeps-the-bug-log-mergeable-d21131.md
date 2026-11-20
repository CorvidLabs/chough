---
kind: evidence
id: 2026-11-20-tc-cli-an-unclosed-fence-keeps-the-bug-log-mergeable-d21131
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
revision: c25f159e58a733b195dd7b615ce63129f385a4c8
content: 834697ee573ee398c64c3309f1ff12a43af91e4b61347cb2cb6636f105dab103
environment: local-macos-arm64
fingerprint: sha256:eb5f1e6eff53f8f66380198c523e47db990bcc1424f0c81263372595f2ff1902
criteria: [DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:28:44Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test bugs -- --exact a_bug_with_an_unclosed_fence_still_merges_both_log_entries for TC-cli-an-unclosed-fence-keeps-the-bug-log-mergeable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c25f159e58a7, observed 2026-11-20T19:28:44Z. The raw log stays in the machine-local store.
