---
kind: evidence
id: 2026-11-13-tc-cli-integrations-manifest
aliases: []
claim: "Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-integrations-manifest]
case_digest: [f2dc5b12c78e1adc195ab83332e1a2f823e4fd34da08456cd7f51f72af9bdf29]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - integrations
  - "--"
  - "--exact"
  - the_manifest_versions_every_template_with_its_digest
exit_code: 101
tests_run: 1
duration_seconds: 0
revision: ced59395ac51df57dfd36bdfd15c2258859e98b1
content: ddb19099f22805755afc1bc77de18bf519dd8c72639381e6de65e4167a01045b
environment: local-macos-arm64
fingerprint: sha256:5c8fb3c0f8fb509a6d7fee2bfb76282769c4870a34648a99fc0978e27ad29de0
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T04:12:39Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision ced59395ac51, observed 2026-11-13T04:12:39Z. 1 recorded line was redacted. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_manifest_versions_every_template_with_its_digest stdout ----
- thread 'the_manifest_versions_every_template_with_its_digest' (939134142) panicked at tests/integrations.rs:26:54:
- <redacted:home-path>: No such file or directory (os error 2)
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
