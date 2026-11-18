---
kind: evidence
id: 2026-11-18-tc-cli-a-busy-evidence-add-leaves-nothing
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved for TC-cli-a-busy-evidence-add-leaves-nothing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-busy-evidence-add-leaves-nothing]
case_digest: [171db9f5113ca1ed54e860a2ae631c98649b907269783242a47990e24116adb9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved
exit_code: 0
tests_run: 1
duration_seconds: 30
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: da5095487729cf07b537ff8298e20a7f96aa5029f72466901fa79cdeab50f76e
environment: local-macos-arm64
fingerprint: sha256:a4b5b2c50d192f001e08b2a8f6cd0c1bfa5f3a522bc3fbc7bc6843704ca987fd
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:37:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test evidence -- --exact a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved for TC-cli-a-busy-evidence-add-leaves-nothing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T18:37:45Z. The raw log stays in the machine-local store.
