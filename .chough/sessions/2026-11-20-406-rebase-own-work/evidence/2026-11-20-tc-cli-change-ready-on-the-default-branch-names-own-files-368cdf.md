---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files-368cdf
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-on-the-default-branch-names-own-files]
case_digest: [0cb9bc7a1551cc2ca552005b982baf6dcb01712bb51e8fbfd1c12d8a3b0a2f12]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - on_the_default_branch_itself_the_change_s_files_are_still_named
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: ff13be893391d1866f1f881d014ec5d181c7e60f
content: 79d69754fcc4e759ac7714c82d28e0cf90a2841d03adf3a6e37f92b923396d4d
environment: local-macos-arm64
fingerprint: sha256:e7abfb9ed95ce9c00b272cd9abecad9103e0081bb6bfcfe622e2be6da0a12868
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:14:31Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ff13be893391, observed 2026-11-20T14:14:31Z. The raw log stays in the machine-local store.
