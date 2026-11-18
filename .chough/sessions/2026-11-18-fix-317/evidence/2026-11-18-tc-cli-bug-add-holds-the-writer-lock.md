---
kind: evidence
id: 2026-11-18-tc-cli-bug-add-holds-the-writer-lock
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-add-holds-the-writer-lock]
case_digest: [88b43a2e26a9350c6cfdd0ad3deba6c79784fb2ff7c4280abcbce8120c782701]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - bug_add_holds_the_writer_lock_and_keeps_another_writers_update
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: caa810e4ef02f64753c2d8ddb1bd0b61a9e2c25d
content: 7c45b7802733e796b7ad5489ded4950293700e9d6fc31ccf43a40e8fddc1e8a3
environment: local-macos-arm64
fingerprint: sha256:2b802b2ea39611c96e72b794567fe851882372832986c04b68a581ec10d85af8
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:27:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision caa810e4ef02, observed 2026-11-18T16:27:00Z. The raw log stays in the machine-local store.
