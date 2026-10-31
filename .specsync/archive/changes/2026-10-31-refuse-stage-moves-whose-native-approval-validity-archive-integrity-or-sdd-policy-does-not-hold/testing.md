---
change: refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold
artifact: testing
---

# Testing

| Requirement | Evidence |
|---|---|
| REQ-cli-008 | tests/stages.rs regression tests; cases for BUG-0006, BUG-0007, BUG-0008 |
| REQ-cli-003 | tests/stages.rs policy tests (invalid policy is unknown, not disabled) |

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-008 | `tests/stages.rs`: approval-invalid build refusal; reset/back/skip/release destination checks; corrupt and missing archive validity |
| REQ-cli-003 | `tests/stages.rs`: malformed, non-object, ill-typed and missing-enabled policies |

Real SpecSync runs in disposable repositories (local Git identity); approvals in them
are labelled SYNTHETIC. Stubbed native status covers values that real SpecSync cannot
be driven to cheaply (`corrupt-history`, missing fields). Regression: full `cargo test`,
`fledge lanes run verify` (includes the feature-and-bug exercise), and a rerun of the
recorded reproductions, which must now refuse. Platforms: hosted CI on Ubuntu, macOS
and Windows; no platform-specific code.
