---
change: align-kimi-integration-with-new-cycle-done-guidance
artifact: testing
---

# Testing

## Evidence sequence

- TC-cli-0105 proves the integrated BUG-0025 failure before the fix and the REQ-cli-021 behavior afterward. Retain both observations and link them to the bug.
- TC-cli-0121 proves archived authenticated done recommends a reason-bearing reset and new native change.
- TC-cli-0122 proves reset replaces the checkpoint next action and back preserves it.
- `cargo test --test stages` and `cargo test --test next` exercise the focused routing and report behavior.
- `fledge lanes run verify` covers formatting, Rust tests, Clippy, HI, SpecSync, audit, cases, exercise and build. `fledge lanes run pre-push` covers the required push checks.
- Use HI 0.8.0, Fledge 1.7.2 and SpecSync 6.1.1 on `local-macos-arm64`. Report hosted checks separately and do not turn analysis jobs into product-test claims.

A structural check is not a fresh Kimi cold-start run. Any unrun platform or check remains unclaimed.
