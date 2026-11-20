---
kind: evidence
id: 2026-11-20-tc-records-a-fragment-of-a-stored-control-line-is-refused-a21996
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused for TC-records-a-fragment-of-a-stored-control-line-is-refused: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-fragment-of-a-stored-control-line-is-refused]
case_digest: [4e563d17db1240c7916c6bdaedeb23d6066dd601b8d7be50c3774d89b75f7cf1]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - a_fragment_of_a_stored_control_line_is_still_refused
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:43f6bb48e1b356f4e665b9de040fc1e651aa0c4f18c89d048a1972c4a09d0caa
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:39:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_fragment_of_a_stored_control_line_is_still_refused for TC-records-a-fragment-of-a-stored-control-line-is-refused: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:39:18Z. The raw log stays in the machine-local store.
