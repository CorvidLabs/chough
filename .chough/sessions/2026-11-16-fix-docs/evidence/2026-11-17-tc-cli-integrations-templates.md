---
kind: evidence
id: 2026-11-17-tc-cli-integrations-templates
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-templates]
case_digest: [3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:81af13820e0d9cb2ab2b01430250428e32c30664d077034176bd5ebc94c0e7cc
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:34Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:34Z. The raw log stays in the machine-local store.
