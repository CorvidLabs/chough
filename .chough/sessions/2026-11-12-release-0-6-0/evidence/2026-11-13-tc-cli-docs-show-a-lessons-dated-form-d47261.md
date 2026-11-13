---
kind: evidence
id: 2026-11-13-tc-cli-docs-show-a-lessons-dated-form-d47261
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-show-a-lessons-dated-form]
case_digest: [c398f03757ff3c0531131399da18f71c70c7b7840dc1fb4f7e40c3128700c941]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - a_lessons_dated_form_is_documented_and_its_example_counts
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:ee47f63696e3d2153fcfe709840c222ef886f085b86017470dd7ce3f775f10b0
criteria: [KNOW-12]
requirements: [REQ-cli-015]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:48:25Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:48:25Z. The raw log stays in the machine-local store.
