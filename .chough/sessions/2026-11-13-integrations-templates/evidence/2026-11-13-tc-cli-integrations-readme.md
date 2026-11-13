---
kind: evidence
id: 2026-11-13-tc-cli-integrations-readme
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-readme]
case_digest: [8789bd2fb23f68bf4f254a914934273ebf75ccf35a2b5a3b0bb3cd0c336c0440]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - the_readme_names_every_template_and_says_chough_has_no_runtime_dependency
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
environment: local-macos-arm64
fingerprint: sha256:88b8c12affee411b6fa561ba157f23d0f8162a90ec8224b5a2b6da981cd4586a
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:12:45Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_readme_names_every_template_and_says_chough_has_no_runtime_dependency for TC-cli-integrations-readme: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision ced59395ac51, observed 2026-11-13T04:12:45Z. 1 recorded line was redacted. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_readme_names_every_template_and_says_chough_has_no_runtime_dependency stdout ----
- thread 'the_readme_names_every_template_and_says_chough_has_no_runtime_dependency' (939138830) panicked at tests/integrations.rs:26:54:
- <redacted:home-path>: No such file or directory (os error 2)
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
