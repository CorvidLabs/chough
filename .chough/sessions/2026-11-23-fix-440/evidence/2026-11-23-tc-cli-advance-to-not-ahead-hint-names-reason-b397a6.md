---
kind: evidence
id: 2026-11-23-tc-cli-advance-to-not-ahead-hint-names-reason-b397a6
aliases: []
claim: "Ran cargo test --locked --test advance_to -- --exact a_refused_step_refuses_the_whole_batch_with_nothing_recorded for TC-cli-advance-to-not-ahead-hint-names-reason: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-advance-to-not-ahead-hint-names-reason]
case_digest: [89f13f1a7c7368849b97dcc7c26bd48553a16a2306b8dd73b8e96904c584588a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - advance_to
  - "--"
  - "--exact"
  - a_refused_step_refuses_the_whole_batch_with_nothing_recorded
exit_code: 0
duration_seconds: 1
revision: 3fc69be26ef4049b23eea300e2c2fba78ebfcfdb
content: 3364b1b473420d3a0bb46f4f41cabc78f980de3973619e6daa56aa4733e46646
environment: local-macos-arm64
fingerprint: sha256:297213e0122c1296193cac0d83111703b8cf5cc86692e6c35640acd1b83dfa44
criteria: [FLOW-2]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T14:20:03Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test advance_to -- --exact a_refused_step_refuses_the_whole_batch_with_nothing_recorded for TC-cli-advance-to-not-ahead-hint-names-reason: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 3fc69be26ef4, observed 2026-11-23T14:20:03Z. The raw log stays in the machine-local store.
