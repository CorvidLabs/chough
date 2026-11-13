---
kind: evidence
id: 2026-11-13-tc-cli-integrations-manifest-1e9fe6
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
revision: 2cf5facb7c792081f8f73104eb458f4cda4408b9
content: e87d689143e165f16ea94afa6e1b6bb1edd9f5a40749321661f272dfb48435ef
environment: local-macos-arm64
fingerprint: sha256:45e6e01e8b441fdd996febdf3a99aa2898b3864da3d6a41420400772a0af9aa5
criteria: [LIFE-24, LIFE-22]
requirements: [REQ-cli-065]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T06:07:11Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran cargo test --locked --test integrations -- --exact the_manifest_versions_every_template_with_its_digest for TC-cli-integrations-manifest: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 2cf5facb7c79, observed 2026-11-13T06:07:11Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_manifest_versions_every_template_with_its_digest stdout ----
- thread 'the_manifest_versions_every_template_with_its_digest' (960887204) panicked at tests/integrations.rs:197:28:
- template entry {"file": String("integrations/claude-code/hc-template.md"), "host": String("claude-code"), "init_host": String("claude"), "name": String("chough-claude"), "sha256": String("c8ff86ab08d0025b4441e1198cc7cd6fa7ef28b81ae084ecc7108568368fd985"), "status": String("supported"), "version": Integer(1)} has no versions list
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
