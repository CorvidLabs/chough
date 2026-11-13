---
kind: evidence
id: 2026-11-13-tc-cli-integrations-templates-11a405
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
duration_seconds: 3
revision: a442a2acc4a23948c031c36675861ba57f04c980
content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
environment: local-macos-arm64
fingerprint: sha256:f5ac40617a76a92475c13f4a74e473619b0804c20cbcf001b60a4027619377c8
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:54:34Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a442a2acc4a2, observed 2026-11-13T04:54:34Z. The raw log stays in the machine-local store.
