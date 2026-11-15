---
kind: evidence
id: 2026-11-15-tc-cli-knowledge-index-check-dc265b
aliases: []
claim: "Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-knowledge-index-check]
case_digest: [9d8e03aa6c11246bac9437a73345439438d54d93a358854132ac25fb362ea7cf]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - vault
  - "--"
  - "--exact"
  - index_check_names_stale_files_and_passes_after_a_rebuild
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
environment: local-macos-arm64
fingerprint: sha256:dcec9482844b93056b421326c1ac85d228a3d7f385a843bd90d6de809bdbb8d1
criteria: [DRIVE-12]
requirements: [REQ-cli-047]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T21:09:09Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test vault -- --exact index_check_names_stale_files_and_passes_after_a_rebuild for TC-cli-knowledge-index-check: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8b5bd53cab53, observed 2026-11-15T21:09:09Z. The raw log stays in the machine-local store.
