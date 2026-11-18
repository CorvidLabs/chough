---
kind: evidence
id: 2026-11-18-tc-cli-gate-summary-names-its-change-with-a-plain-dash
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-gate-summary-names-its-change-with-a-plain-dash]
case_digest: [a19385917de566020aa6f9aa973a87569340ad1af030ac46f8fbf83925a51c74]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - a_gate_summary_names_its_change_with_a_plain_dash
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
content: 155ef6dbb084df8ddbea4d153055b030280874ae96301fabd8d351f4f0f1593b
environment: local-macos-arm64
fingerprint: sha256:feb07bd0018e602daf50b775f2e4c410afc7cac567bb9c404d6aa9c1c23b820e
criteria: [LIFE-21]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:11:03Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 66543c2bf509, observed 2026-11-18T16:11:03Z. The raw log stays in the machine-local store.
