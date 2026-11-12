---
kind: evidence
id: 2026-11-12-tc-cli-gate-record-reports-the-approval-it-ran
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record for TC-cli-gate-record-reports-the-approval-it-ran: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-record-reports-the-approval-it-ran]
case_digest: [48137d87ee867ae18dfda0f6199139e38ff84b74d0d103abb0760738adb4945c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: c7cefe408f5e9b926ffab814452bb27ecf044d71
content: 689c6fe45ce152ebad43eda1f8c4b2b202ff0cf2434882b5ae05929bef8bbbb8
environment: local-macos-arm64
fingerprint: sha256:c1cb42f2215cf15723afac3f0b346208ce2132ead17aca38a91675427bcc7413
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-history-014]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:37:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record for TC-cli-gate-record-reports-the-approval-it-ran: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision c7cefe408f5e, observed 2026-11-12T17:37:21Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record stdout ----
- thread 'gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record' (920973456) panicked at tests/humans.rs:376:5:
- assertion `left == right` failed: {
-   "closed": [
-     {
-       "gate": "Scope approval",
-       "number": 1,
-       "summary_digest": "adbc01de8ed1f0f7d27e9e24a4e0794263eb0d110c034bca238852b6bb9306bb"
-     }
-   ],
-   "list": "fe56e24cfbba",
-   "schema_version": 1
- }
-   left: Null
-  right: "ran specsync change approve human-change as SYNTHETIC-actor"
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 9 filtered out; finished in 1.22s
