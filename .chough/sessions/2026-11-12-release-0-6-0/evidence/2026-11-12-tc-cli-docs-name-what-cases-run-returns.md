---
kind: evidence
id: 2026-11-12-tc-cli-docs-name-what-cases-run-returns
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-name-what-cases-run-returns]
case_digest: [3459bfcddf1b5e82fa632dea9e9a3cf99c4eb6153919a396d795e933f959a482]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - the_cli_reference_names_what_cases_run_returns
exit_code: 101
tests_run: 1
duration_seconds: 51
revision: e005da2e408386cbc35a21498f87135c42952257
content: ea3944c1edfdad8431229957114e4716be4f731c5c63f406c4ae3b3d649b5470
environment: local-macos-arm64
fingerprint: sha256:c6949cc9f894a7c5035e58eecd97a07537b57482a8b83beded8566ca03035ca5
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:34:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact the_cli_reference_names_what_cases_run_returns for TC-cli-docs-name-what-cases-run-returns: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision e005da2e4083, observed 2026-11-12T16:34:22Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_cli_reference_names_what_cases_run_returns stdout ----
- thread 'the_cli_reference_names_what_cases_run_returns' (919237392) panicked at tests/cases.rs:463:28:
- the cases row does not say what `run` returns: | `cases check\|list\|select\|run\|record` | Validate curated slug and exact legacy numeric definitions; list digests; select focused cases (`--criterion`, `--requirement`, `--path`, `--track copy-localization\|design-qa`); run an automated binding with `--environment`; record a manual or agent-assisted run with `--environment`, `--method manual\|agent-assisted`, `--summary`, optional `--artifact` and `--limitation`, and `--verdict N=pass\|fail` for every expected outcome. `run` and `record` name the evidence when `--observation-id` is omitted (see [Record names](#record-names)), return it as `observation` with the case ID as `case` beside `recorded`, and exit 3 when the recorded outcome is not passed. |
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 33 filtered out; finished in 36.55s
