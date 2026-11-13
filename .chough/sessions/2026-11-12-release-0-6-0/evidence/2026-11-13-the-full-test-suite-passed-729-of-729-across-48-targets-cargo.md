---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-729-of-729-across-48-targets-cargo
aliases: []
claim: The full test suite passed 729 of 729 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 1e53ea35 with the fourth round's requirement text materialized, after the last source change; clippy --all-targets -D warnings and rustfmt --check were clean there
result: passed
evidence_kind: product-test
method: automated
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
criteria: [DRIVE-27, FLOW-16, FLOW-23]
requirements: []
contracts: []
limitations:
  - Run on the working tree at 1e53ea35 before the materialized spec text was committed; the commits after it add only that text, the records spec's documentation and Chough records
  - Local macOS run on Rust 1.96.0; hosted CI runs the suite only once the change is archived
  - The raw log is not committed
observed: 2026-11-13T00:40:56Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 729 of 729 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 1e53ea35 with the fourth round's requirement text materialized, after the last source change; clippy --all-targets -D warnings and rustfmt --check were clean there.**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:40:56Z.
