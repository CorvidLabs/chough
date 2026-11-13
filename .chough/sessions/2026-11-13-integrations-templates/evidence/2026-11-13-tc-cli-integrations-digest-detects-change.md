---
kind: evidence
id: 2026-11-13-tc-cli-integrations-digest-detects-change
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-digest-detects-change]
case_digest: [adaaf77380423e15ce74e2525868d5f59885efefd6aff95edb4a5c54c1fc2fbd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - a_changed_template_byte_no_longer_matches_its_listed_digest
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
environment: local-macos-arm64
fingerprint: sha256:bd713a180810297ea0ce6d871e8b3efb75f0542ef0016bf49927f5973f5f76ce
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:12:41Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact a_changed_template_byte_no_longer_matches_its_listed_digest for TC-cli-integrations-digest-detects-change: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision ced59395ac51, observed 2026-11-13T04:12:41Z. 1 recorded line was redacted. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_changed_template_byte_no_longer_matches_its_listed_digest stdout ----
- thread 'a_changed_template_byte_no_longer_matches_its_listed_digest' (939135670) panicked at tests/integrations.rs:26:54:
- <redacted:home-path>: No such file or directory (os error 2)
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
