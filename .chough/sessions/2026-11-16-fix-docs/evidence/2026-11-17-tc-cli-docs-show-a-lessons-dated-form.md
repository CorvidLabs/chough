---
kind: evidence
id: 2026-11-17-tc-cli-docs-show-a-lessons-dated-form
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
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:3e89d8da2c50e3851f3f1aef81e095f47602b7e1ef6e7058666f3ccfb6a0123d
criteria: [KNOW-12]
requirements: [REQ-cli-015]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:33Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test docs -- --exact a_lessons_dated_form_is_documented_and_its_example_counts for TC-cli-docs-show-a-lessons-dated-form: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:33Z. The raw log stays in the machine-local store.
