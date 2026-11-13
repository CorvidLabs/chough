---
kind: evidence
id: 2026-11-13-tc-cli-observation-cap-refuses-before-writing
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-observation-cap-refuses-before-writing]
case_digest: [922a8c1ea9a4708af23602a00ee5e79158af7a0df3cdb98152c5da944e2b5f33]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: c7bf6347e3ba43cc30275a34d08fb175604c31ee
content: f0563e466a515e7e5afa6291a5cd5b388b6dd82f916b71024efd7f953f4476a4
environment: local-macos-arm64
fingerprint: sha256:89fb33e7a5553732e49f484d1bc3b33cb034e0a19c90db73592cdb3d9b8e384a
criteria: [DRIVE-14, FLOW-13]
requirements: [REQ-cli-032]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:39:56Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision c7bf6347e3ba, observed 2026-11-13T02:39:56Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs stdout ----
- thread 'at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs' (931504699) panicked at tests/next.rs:3232:9:
- ["cases", "run", "TC-cli-f4-rerun", "--environment", "local"] names no recovery: Error: too many observation (limit 400)
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 35 filtered out; finished in 1.57s
