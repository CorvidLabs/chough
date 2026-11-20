---
kind: evidence
id: 2026-11-20-tc-cli-ready-without-a-dry-run-names-6-4-0
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ready-without-a-dry-run-names-6-4-0]
case_digest: [39ac7efa62d44b4d17300d1bc9ad37e1cffd89e5d1d1dc9adeb330a9b4d6e187]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_specsync_without_the_dry_run_names_the_install_command
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: e8d2ecc43843abdac87b9c439fcbd58f991d112b
content: 8ae2b83a6c451ac9ac8c50f3ac993bc64bf2201b907353f186b31caddaab6902
environment: local-macos-arm64
fingerprint: sha256:44e1a1e1bc147c19396946f27ea7d915bc8062108f8280d84ccab12d6c29c723
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:12:47Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test ready -- --exact a_specsync_without_the_dry_run_names_the_install_command for TC-cli-ready-without-a-dry-run-names-6-4-0: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8d2ecc43843, observed 2026-11-20T19:12:47Z. The raw log stays in the machine-local store.
