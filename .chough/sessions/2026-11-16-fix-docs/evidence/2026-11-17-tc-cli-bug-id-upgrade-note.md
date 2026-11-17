---
kind: evidence
id: 2026-11-17-tc-cli-bug-id-upgrade-note
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids for TC-cli-bug-id-upgrade-note: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-id-upgrade-note]
case_digest: [8b0990f42852eab3729b518979243b7e9b061314eefcdaab67e5aae32884d7fe]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:03a7fc4c6762689c8366e7b985404a25f1c3bf2173c3b45a8d95a648d203b5d6
criteria: [FLOW-8]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:37Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test bugs -- --exact the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids for TC-cli-bug-id-upgrade-note: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:37Z. The raw log stays in the machine-local store.
