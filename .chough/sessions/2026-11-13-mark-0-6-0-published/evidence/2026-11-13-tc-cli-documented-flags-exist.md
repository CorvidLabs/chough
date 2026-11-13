---
kind: evidence
id: 2026-11-13-tc-cli-documented-flags-exist
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact documented_chough_commands_and_flags_exist for TC-cli-documented-flags-exist: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-documented-flags-exist]
case_digest: [ff7a879860dfb3617227b405f954d0b77861c0f8995eeb3c0c37b8e0fe352347]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - documented_chough_commands_and_flags_exist
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 443c63ab65b747f47248ad2166f8fb299da94a41
content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
environment: local-macos-arm64
fingerprint: sha256:f2828bad028d83289bd60a55cb1432dfa06d073c6809afcb6ac521191241115b
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:55:12Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact documented_chough_commands_and_flags_exist for TC-cli-documented-flags-exist: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 443c63ab65b7, observed 2026-11-13T01:55:12Z. The raw log stays in the machine-local store.
