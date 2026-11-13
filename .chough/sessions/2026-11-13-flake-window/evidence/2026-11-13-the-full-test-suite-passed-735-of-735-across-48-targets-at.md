---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-735-of-735-across-48-targets-at
aliases: []
claim: "The full test suite passed 735 of 735 across 48 targets at 05786a14, #252's head after t384's must-fix, under the heavy-tests lock; the flake-window fixtures passed 27 of 27 and rustfmt --check was clean. No Rust file changed, so clippy was not rerun"
result: passed
evidence_kind: product-test
method: cargo test --locked --no-fail-fast under the heavy-tests lock; python3 -B .github/scripts/test-flake-window.py; cargo fmt --check
revision: 05786a14526a1c168e7ff6678d81ac884abd579f
content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
environment: local-macos-arm64
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: []
limitations: []
observed: 2026-11-13T05:51:56Z
actor:
  - agent: claude-code
    terminal: t38r
---
**The full test suite passed 735 of 735 across 48 targets at 05786a14, #252's head after t384's must-fix, under the heavy-tests lock; the flake-window fixtures passed 27 of 27 and rustfmt --check was clean. No Rust file changed, so clippy was not rerun.**

Recorded as passed in local-macos-arm64 at revision 05786a14526a, observed 2026-11-13T05:51:56Z.
