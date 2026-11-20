---
kind: evidence
id: 2026-11-20-tc-cli-bug-add-leaves-the-session-record
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-add-leaves-the-session-record]
case_digest: [5865d20fc1880780d79a943f924712df017375d0e2d58e01bcfe04507f94bb22]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - bug_add_in_a_session_records_the_session_and_leaves_its_record
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
environment: local-macos-arm64
fingerprint: sha256:802fade65057e811b36ff54dd97b44484d33b0ee05cdc7d56fd79c6652f8dcd6
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:48:24Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_add_in_a_session_records_the_session_and_leaves_its_record for TC-cli-bug-add-leaves-the-session-record: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2f99dcfb42f1, observed 2026-11-20T15:48:24Z. The raw log stays in the machine-local store.
