---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-missing-specsync
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
revision: 24323c5c04434521dfff65da788125d02010f545
content: 896ac699cac24ac978487013ca31fddc5a27f90a2931184ae75b0812c68d6280
environment: local-macos-arm64
fingerprint: sha256:767b8df73166a282a2d2b131f66bfea649acd406529952b307822046d3d9bd43
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T02:16:57Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_missing_specsync_names_the_install_command for TC-cli-change-ready-missing-specsync: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 24323c5c0443, observed 2026-11-19T02:16:57Z. The raw log stays in the machine-local store.
