---
kind: evidence
id: 2026-11-18-tc-cli-gate-summary-names-its-change-with-a-plain-dash-fb30c0
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
duration_seconds: 1
revision: ca5a2838bf2d882bea9c9a544486289f9ba5258b
content: 4d8d83aa83242007df0ec3340c6164769a169343ef7032a74703fb34e8808658
environment: local-macos-arm64
fingerprint: sha256:090fbf111949daa3e52297b2e09bf97841b6a565ccfa12efee936b04a1cdbeea
criteria: [LIFE-21]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T17:00:25Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test gates -- --exact a_gate_summary_names_its_change_with_a_plain_dash for TC-cli-gate-summary-names-its-change-with-a-plain-dash: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ca5a2838bf2d, observed 2026-11-18T17:00:25Z. The raw log stays in the machine-local store.
