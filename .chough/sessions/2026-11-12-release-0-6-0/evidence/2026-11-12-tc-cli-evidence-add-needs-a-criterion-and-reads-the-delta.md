---
kind: evidence
id: 2026-11-12-tc-cli-evidence-add-needs-a-criterion-and-reads-the-delta
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact evidence_add_needs_a_criterion_and_resolves_the_sessions_delta_requirements for TC-cli-evidence-add-needs-a-criterion-and-reads-the-delta: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-evidence-add-needs-a-criterion-and-reads-the-delta]
case_digest: [dfe34b0df32d5b174108adb169b81b6639c3fc9f429bd69d50e0f16af7b24c47]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - evidence_add_needs_a_criterion_and_resolves_the_sessions_delta_requirements
exit_code: 101
tests_run: 1
duration_seconds: 3
revision: c5058a4bcc7182ca7df4cf086e113e1d231a62e6
content: 4872d07872702c13751a039c974bba214d78cfc5c0032e9c70707524a14eeb18
environment: local-macos-arm64
fingerprint: sha256:5124154c0033b98a74564250684f43ac8956ece4a86ff398b79ae171d45a70db
criteria: [LIFE-5, FLOW-7]
requirements: [REQ-cli-005]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:36:42Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact evidence_add_needs_a_criterion_and_resolves_the_sessions_delta_requirements for TC-cli-evidence-add-needs-a-criterion-and-reads-the-delta: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision c5058a4bcc71, observed 2026-11-12T20:36:42Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- evidence_add_needs_a_criterion_and_resolves_the_sessions_delta_requirements stdout ----
- thread 'evidence_add_needs_a_criterion_and_resolves_the_sessions_delta_requirements' (923244369) panicked at tests/cases.rs:2862:9:
- ["--criterion", "LIFE-1", "--requirement", "REQ-cli-090"]: error: the following required arguments were not provided:
-   --contract <CONTRACTS>
- Usage: chough evidence add --criterion <CRITERIA> --requirement <REQUIREMENTS> --contract <CONTRACTS> --kind <KIND> --outcome <OUTCOME> --environment <ENVIRONMENT> --summary <SUMMARY> [ID]
- For more information, try '--help'.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 36 filtered out; finished in 1.73s
