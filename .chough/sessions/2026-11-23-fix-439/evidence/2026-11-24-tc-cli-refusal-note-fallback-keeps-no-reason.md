---
kind: evidence
id: 2026-11-24-tc-cli-refusal-note-fallback-keeps-no-reason
aliases: []
claim: "Ran cargo test --locked --test native_check -- --exact the_fallback_note_is_kept_when_no_refusal_is_recorded for TC-cli-refusal-note-fallback-keeps-no-reason: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-refusal-note-fallback-keeps-no-reason]
case_digest: [8fe9591b95ddd6c8dcb2dff2e108f1ab98e32c685b18a0f9be3cd953d1a353ff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - native_check
  - "--"
  - "--exact"
  - the_fallback_note_is_kept_when_no_refusal_is_recorded
exit_code: 0
duration_seconds: 0
revision: e6cbf99f443f647259a452e0371384c85fa4fee4
content: 2dd3feff4ad669bbcb01f34f4fd2f45076ce9c30175af729afca5869a7a13314
environment: local-macos-arm64
fingerprint: sha256:9dafe23093ef9f61cfae387af43d8c7c3ef62478513e4ab04b3c3626bb3d11b6
criteria: [LIFE-7]
requirements: [REQ-cli-039]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-24T04:45:41Z
actor:
  - terminal: t14z
---
**Ran cargo test --locked --test native_check -- --exact the_fallback_note_is_kept_when_no_refusal_is_recorded for TC-cli-refusal-note-fallback-keeps-no-reason: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision e6cbf99f443f, observed 2026-11-24T04:45:41Z. The raw log stays in the machine-local store.
