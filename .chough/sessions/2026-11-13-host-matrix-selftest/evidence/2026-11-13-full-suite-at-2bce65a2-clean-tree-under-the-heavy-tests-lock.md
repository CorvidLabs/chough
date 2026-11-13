---
kind: evidence
id: 2026-11-13-full-suite-at-2bce65a2-clean-tree-under-the-heavy-tests-lock
aliases: []
claim: "Full suite at 2bce65a2 (clean tree) under the heavy-tests lock with Rust 1.96.0: cargo +1.96.0 test --locked --no-fail-fast passed 735 tests in 48 targets, 0 failed, 0 ignored; cargo +1.96.0 clippy --all-targets --locked -- -D warnings and cargo +1.96.0 fmt --check exited 0."
result: passed
evidence_kind: product-test
revision: 2bce65a296608cb0206b3a61761f400e2b4d50cb
content: 8327d299d9048bc9377db6a93dc4cfccc37b3fd42e35d3e1f690ea5fcfaa4a79
environment: local-macos-arm64
criteria: [LIFE-21, FLOW-21]
requirements: [REQ-cli-066]
contracts: []
limitations: [macOS arm64 only; hosted CI runs Ubuntu and macOS after the change is archived]
observed: 2026-11-13T06:53:09Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Full suite at 2bce65a2 (clean tree) under the heavy-tests lock with Rust 1.96.0: cargo +1.96.0 test --locked --no-fail-fast passed 735 tests in 48 targets, 0 failed, 0 ignored; cargo +1.96.0 clippy --all-targets --locked -- -D warnings and cargo +1.96.0 fmt --check exited 0.**

Recorded as passed in local-macos-arm64 at revision 2bce65a29660, observed 2026-11-13T06:53:09Z.
