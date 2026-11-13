---
kind: evidence
id: 2026-11-13-tc-cli-integrations-digest-detects-change-3f83e7
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-digest-detects-change]
case_digest: [adaaf77380423e15ce74e2525868d5f59885efefd6aff95edb4a5c54c1fc2fbd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - a_changed_template_byte_no_longer_matches_its_listed_digest
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: a442a2acc4a23948c031c36675861ba57f04c980
content: 2fb922c6ab66ac421f2cbd88feaa0ec2fd87004f296b827945dfb087f7231437
environment: local-macos-arm64
fingerprint: sha256:d2221a8234592f3cba2a3c4615842519f1e0d1086507ad3e4b82539c0deef22c
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:54:38Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision a442a2acc4a2, observed 2026-11-13T04:54:38Z. The raw log stays in the machine-local store.
