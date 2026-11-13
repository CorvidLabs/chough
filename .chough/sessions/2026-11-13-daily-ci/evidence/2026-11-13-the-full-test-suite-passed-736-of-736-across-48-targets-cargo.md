---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-736-of-736-across-48-targets-cargo
aliases: []
claim: The full test suite passed 736 of 736 across 48 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at a2d271c9, the daily CI head on main 38fe24c0 with REQ-cli-064 materialized; rustfmt --check was clean
result: passed
evidence_kind: product-test
method: cargo test --locked --no-fail-fast under the heavy-tests lock
revision: a2d271c9c60126932d4dd03a40ddc5433419eb25
content: ccd87ffdd45b3166049b6bc37bf8988c0bc7e17e3c1886e60f4e743bd4f3ae3d
environment: local-macos-arm64
criteria: [LIFE-21]
requirements: [REQ-cli-064]
contracts: []
limitations: []
observed: 2026-11-13T06:18:33Z
actor:
  - agent: claude-code
    terminal: t38r
---
**The full test suite passed 736 of 736 across 48 targets (cargo test --locked --no-fail-fast, under the heavy-tests lock) at a2d271c9, the daily CI head on main 38fe24c0 with REQ-cli-064 materialized; rustfmt --check was clean.**

Recorded as passed in local-macos-arm64 at revision a2d271c9c601, observed 2026-11-13T06:18:33Z.
