---
kind: evidence
id: 2026-11-15-tc-cli-librarian-off-no-knowledge-at-done
aliases: []
claim: "Ran cargo test --locked --test librarian_trim -- --exact a_librarian_off_project_gets_no_knowledge_directory_at_done for TC-cli-librarian-off-no-knowledge-at-done: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-librarian-off-no-knowledge-at-done]
case_digest: [cdb85ba6754f0ab0b40223e69a8c407febb1cfb59053209b6b432527c81f05fc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - librarian_trim
  - "--"
  - "--exact"
  - a_librarian_off_project_gets_no_knowledge_directory_at_done
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 26bf0f5405e166e6ee354ea070336fb5c259d4d3
content: 09d97d20c39bbbc737c1c024e7bab088f38916ec29d4ab3db19035f2779ddfe5
environment: local-macos-arm64
fingerprint: sha256:305bc353ffa3f3828df59d8f2bec92ff4114201d42655424e1de628eea9abb29
criteria: [KNOW-5]
requirements: [REQ-cli-043]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T20:22:06Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test librarian_trim -- --exact a_librarian_off_project_gets_no_knowledge_directory_at_done for TC-cli-librarian-off-no-knowledge-at-done: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26bf0f5405e1, observed 2026-11-15T20:22:06Z. The raw log stays in the machine-local store.
