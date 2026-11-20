---
kind: evidence
id: 2026-11-20-tc-history-a-front-matter-conflict-is-named
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-front-matter-conflict-is-named]
case_digest: [c28d317bab6b5af2bf356176cf7b7c972412547511c18dba7abc93ca1bafeac0]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - a_front_matter_conflict_is_named_and_resolved_not_shelved
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:c4e85ecdb929d78e7926b6f28001697fccf71b2e904d8ad14070ab87d1396e8a
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:48:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact a_front_matter_conflict_is_named_and_resolved_not_shelved for TC-history-a-front-matter-conflict-is-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:48:35Z. The raw log stays in the machine-local store.
