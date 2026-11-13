---
kind: evidence
id: 2026-11-13-tc-cli-ship-passes-the-relay-to-the-native-review
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_passes_the_relay_to_the_native_review for TC-cli-ship-passes-the-relay-to-the-native-review: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-passes-the-relay-to-the-native-review]
case_digest: [f8447be060e01366a3d554586a1473896332970e01aa2d05783452c6ce17ef4c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_passes_the_relay_to_the_native_review
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: da86ddcb2ed26059cef30effd419f2513a703f8f
content: 7d32c750c9713a5af53db6e07f5ef7e165d03878f169e5cea2e68190316f4702
environment: local-macos-arm64
fingerprint: sha256:b8ccb7ced9c9aba055d621c36d209ac842552b5f9e57f084aece6a69b9da358b
criteria: [DRIVE-20, FLOW-23]
requirements: [REQ-cli-059, REQ-records-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T07:44:17Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_passes_the_relay_to_the_native_review for TC-cli-ship-passes-the-relay-to-the-native-review: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision da86ddcb2ed2, observed 2026-11-13T07:44:17Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- ship_passes_the_relay_to_the_native_review stdout ----
- thread 'ship_passes_the_relay_to_the_native_review' (978152335) panicked at tests/change.rs:684:5:
- assertion `left == right` failed
-   left: [["change", "review", "a5", "--reviewer", "SYNTHETIC-reviewer", "--verdict", "pass"]]
-  right: [["change", "review", "a5", "--reviewer", "SYNTHETIC-reviewer", "--verdict", "pass", "--words", "Ship it, the review passes", "--where", "SYNTHETIC chat, 2026-11-13 03:44Z, relayed by SYNTHETIC-coordinator", "--recorded-by", "SYNTHETIC-agent"]]
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 20 filtered out; finished in 1.33s
