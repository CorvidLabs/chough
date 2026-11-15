---
kind: evidence
id: 2026-11-15-tc-cli-readme-verify-list-matches-fledge-82659a
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-readme-verify-list-matches-fledge]
case_digest: [8053fe1bd76560e758c7d613849c909d72c620944f864dc5d9bfa8e51fdae130]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - readme_verify_lane_list_names_every_fledge_verify_step
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 8b5bd53cab53ec98725db6feb53ee0f6570e3451
content: 12c5104737c421d6e72a751786ffabf99172508dabf9ee642dda94f38685a761
environment: local-macos-arm64
fingerprint: sha256:d40c39ada86f3bb4c91080aedef5b0fe1ae6ffe54cd8ea11ff69f0807a984706
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T21:09:06Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test docs -- --exact readme_verify_lane_list_names_every_fledge_verify_step for TC-cli-readme-verify-list-matches-fledge: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8b5bd53cab53, observed 2026-11-15T21:09:06Z. The raw log stays in the machine-local store.
