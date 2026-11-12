---
kind: evidence
id: 2026-11-12-tc-cli-cases-check-a-module-without-requirements-md
aliases: []
claim: "Ran cargo test --locked --test cases -- --exact a_module_without_requirements_md_links_what_its_approved_delta_adds for TC-cli-cases-check-a-module-without-requirements-md: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-cases-check-a-module-without-requirements-md]
case_digest: [f09e191067157651290527bfd1136c7111e6b4d5ef9f6d1a323119a333a0fdac]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - cases
  - "--"
  - "--exact"
  - a_module_without_requirements_md_links_what_its_approved_delta_adds
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: dd3e2ab5df329a8dc68f9f89725310b2b56e029f
content: 6287c845d27db39c417052a4608cf898a2c1017c06b5e7c461b069727288236f
environment: local-macos-arm64
fingerprint: sha256:f31cb7c9f3ef72221bbd53c52a6e02fb975842884cac18e2ec6b2b93c3c37eb8
criteria: [LIFE-5]
requirements: [REQ-cli-009]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:42:32Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test cases -- --exact a_module_without_requirements_md_links_what_its_approved_delta_adds for TC-cli-cases-check-a-module-without-requirements-md: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision dd3e2ab5df32, observed 2026-11-12T17:42:32Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_module_without_requirements_md_links_what_its_approved_delta_adds stdout ----
- thread 'a_module_without_requirements_md_links_what_its_approved_delta_adds' (921046148) panicked at tests/common/mod.rs:154:9:
- ["cases", "check"] failed: Error: .chough/cases/cli.cases.toml: case TC-cli-0001
- Caused by:
-     0: contract requires its native requirements.md companion
-     1: No such file or directory (os error 2)
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 34 filtered out; finished in 1.20s
