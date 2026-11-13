---
kind: evidence
id: 2026-11-13-tc-cli-integrations-templates-ed769d
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
duration_seconds: 1
revision: c0e3c340ae4fce048f37442831d242d42eeeef28
content: 1012c8e4277ff01558fc9805d340ea45db95300ec2124f149e6c4af0ae6b866a
environment: local-macos-arm64
fingerprint: sha256:a3f68efe12aab162e390a0360524da4c497fb26cc3f1293d98ae93a5c8cf7764
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:19:30Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c0e3c340ae4f, observed 2026-11-13T06:19:30Z. The raw log stays in the machine-local store.
