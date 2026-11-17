---
kind: evidence
id: 2026-11-17-tc-cli-change-ship-outlives-read-bound
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_outlives_the_native_read_bound for TC-cli-change-ship-outlives-read-bound: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ship-outlives-read-bound]
case_digest: [773001b501a33eb8d8c5764be4341109b641f4dbb057809ddd670eba3ddee717]
command: [cargo, test, --locked, --test, change, --, --exact, ship_outlives_the_native_read_bound]
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:de774ced16fafa9232b7e4a1b738249b608893d17928690cde34f482cb225ec1
criteria: [DRIVE-20]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:17Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test change -- --exact ship_outlives_the_native_read_bound for TC-cli-change-ship-outlives-read-bound: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:17Z. The raw log stays in the machine-local store.
