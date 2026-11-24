---
kind: evidence
id: 2026-11-24-tc-cli-refusal-note-quotes-the-recorded-refusal
aliases: []
claim: "Ran cargo test --locked --test native_check -- --exact a_recorded_refusal_is_shown_in_the_not_passed_note for TC-cli-refusal-note-quotes-the-recorded-refusal: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-refusal-note-quotes-the-recorded-refusal]
case_digest: [8781940d3f1abcc9ecc6ba0d142bd969c0d8d4a661f360e19f07bc5c7313e9ea]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - native_check
  - "--"
  - "--exact"
  - a_recorded_refusal_is_shown_in_the_not_passed_note
exit_code: 0
duration_seconds: 0
revision: e6cbf99f443f647259a452e0371384c85fa4fee4
content: 2dd3feff4ad669bbcb01f34f4fd2f45076ce9c30175af729afca5869a7a13314
environment: local-macos-arm64
fingerprint: sha256:05608364b6221f012ea199835948e212be7b1ded080805fc0145af3338da5466
criteria: [LIFE-7]
requirements: [REQ-cli-039]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-24T04:45:40Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test native_check -- --exact a_recorded_refusal_is_shown_in_the_not_passed_note for TC-cli-refusal-note-quotes-the-recorded-refusal: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision e6cbf99f443f, observed 2026-11-24T04:45:40Z. The raw log stays in the machine-local store.
