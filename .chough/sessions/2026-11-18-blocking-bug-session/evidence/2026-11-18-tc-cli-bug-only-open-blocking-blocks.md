---
kind: evidence
id: 2026-11-18-tc-cli-bug-only-open-blocking-blocks
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test for TC-cli-bug-only-open-blocking-blocks: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-only-open-blocking-blocks]
case_digest: [c9dafba98df618115f41b036fccb2b66d1057cbaf9d0ea49346aac76953a0423]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - only_an_open_blocking_bug_blocks_leaving_test
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 7bb2ffc7a8579bd0d69f47b0040d03d9ea36d8c3
content: 960c6d0f19fc3725c7582da2d7b299176a839ec9d092f2e909f35bc1cfa3dc16
environment: local-macos-arm64
fingerprint: sha256:5da783ad34cf59242f7196542377b8f9a4e732be74a5b53a9f84a8c57c1b507a
criteria: [FLOW-8, LIFE-5]
requirements: [REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T00:33:33Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test bugs -- --exact only_an_open_blocking_bug_blocks_leaving_test for TC-cli-bug-only-open-blocking-blocks: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7bb2ffc7a857, observed 2026-11-18T00:33:33Z. The raw log stays in the machine-local store.
