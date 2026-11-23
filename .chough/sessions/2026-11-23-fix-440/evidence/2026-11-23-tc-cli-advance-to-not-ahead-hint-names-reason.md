---
kind: evidence
id: 2026-11-23-tc-cli-advance-to-not-ahead-hint-names-reason
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
revision: 6fd77b85ab3c24501f1f818e8b230d34335c067c
content: acaba44368030e4be39bc542e1b193ce87688f980f44e0e9f463e900efe54eed
environment: local-macos-arm64
fingerprint: sha256:cee6afd79d5eb6eb5cd3351f78aa93fba0ceb24a6f0c7397826aeadec6ac6fb4
criteria: [FLOW-2]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-23T14:15:38Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test advance_to -- --exact a_refused_step_refuses_the_whole_batch_with_nothing_recorded for TC-cli-advance-to-not-ahead-hint-names-reason: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 6fd77b85ab3c, observed 2026-11-23T14:15:38Z. The raw log stays in the machine-local store.
