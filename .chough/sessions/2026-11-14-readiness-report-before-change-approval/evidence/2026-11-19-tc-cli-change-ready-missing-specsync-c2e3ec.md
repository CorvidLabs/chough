---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-missing-specsync-c2e3ec
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-missing-specsync]
case_digest: [83e8e7b95935df836bbc58875983636d8de079dd71ab45eb2e380260f9708881]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_missing_specsync_names_the_install_command
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 1e2c643fed4bab3506f7deaee246c4eb2510896e
content: bc5ef64f9ce07c56ca46f88b34d97601bc8cda6da18d08502b5f60d8912b4206
environment: local-macos-arm64
fingerprint: sha256:0a374f5c406ea3fb84c1f9ff41bab466009bc666a289dcd6525dea1d2494e7e2
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:38:18Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e2c643fed4b, observed 2026-11-19T04:38:18Z. The raw log stays in the machine-local store.
