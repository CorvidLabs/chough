---
kind: evidence
id: 2026-11-13-tc-cli-integrations-no-runtime-dependency
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-no-runtime-dependency]
case_digest: [f7f634f29e7ef6671beb8a7fec69289f22983cf4ba5d71e54bf2be64e7d9daf5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
environment: local-macos-arm64
fingerprint: sha256:b0ac88fe3b767c4fd9523065f4bd2dea3ca07f539915c5f9fc26606b640f8d31
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:12:43Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them for TC-cli-integrations-no-runtime-dependency: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision ced59395ac51, observed 2026-11-13T04:12:43Z. 1 recorded line was redacted. The raw log stays in the machine-local store.

## Key failing lines

- ---- chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them stdout ----
- thread 'chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them' (939136687) panicked at tests/integrations.rs:226:13:
- <redacted:home-path> names hc
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.01s
