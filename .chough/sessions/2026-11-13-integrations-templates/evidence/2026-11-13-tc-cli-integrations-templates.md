---
kind: evidence
id: 2026-11-13-tc-cli-integrations-templates
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-templates]
case_digest: [3666c27de6e873eab3f46399adac32565efbb8d0d2b3a7c6cb12645fd38ec4ff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
environment: local-macos-arm64
fingerprint: sha256:291f54571fa630717caa67140a54f92685f2b75d1b0fb777a60decbf3b578025
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:12:36Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path for TC-cli-integrations-templates: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision ced59395ac51, observed 2026-11-13T04:12:36Z. 1 recorded line was redacted. The raw log stays in the machine-local store.

## Key failing lines

- ---- each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path stdout ----
- thread 'each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path' (939132021) panicked at tests/integrations.rs:26:54:
- <redacted:home-path>: No such file or directory (os error 2)
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.63s
