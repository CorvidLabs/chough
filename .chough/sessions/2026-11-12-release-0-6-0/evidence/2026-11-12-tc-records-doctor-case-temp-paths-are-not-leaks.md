---
kind: evidence
id: 2026-11-12-tc-records-doctor-case-temp-paths-are-not-leaks
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_temporary_path_in_a_case_definition_is_not_a_leak for TC-records-doctor-case-temp-paths-are-not-leaks: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-doctor-case-temp-paths-are-not-leaks]
case_digest: [370f3cf8905f4572b0e450a0a4c8bf13736e848adc9cc6df334808b927bde0fc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_temporary_path_in_a_case_definition_is_not_a_leak
exit_code: 101
tests_run: 1
duration_seconds: 4
revision: 423169dee4924d188777c611349ce0e0318820a8
content: d6225f745d500fe2fe4b1988410ab21474daacab529a7e82b06c53e516869afa
environment: local-macos-arm64
fingerprint: sha256:cdae191e6b9b56cd35c859ad59e5ceae9cd64057dfa89feef094ef919f3b8a62
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:03:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test doctor -- --exact a_temporary_path_in_a_case_definition_is_not_a_leak for TC-records-doctor-case-temp-paths-are-not-leaks: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 423169dee492, observed 2026-11-12T16:03:51Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_temporary_path_in_a_case_definition_is_not_a_leak stdout ----
- thread 'a_temporary_path_in_a_case_definition_is_not_a_leak' (918526649) panicked at tests/doctor.rs:589:5:
- {
-   "findings": [
-     {
-       "decision": false,
-       "fix": "chough doctor --fix --redact",
-       "kind": "leak",
-       "line": 19,
-       "match": "system-path",
-       "status": "found",
-       "target": ".chough/cases/cli.temp-data.cases.toml"
-     },
-     {
-       "decision": false,
-       "fix": "chough doctor --fix --redact",
-       "kind": "leak",
-       "line": 20,
-       "match": "system-path",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 14 filtered out; finished in 3.65s
