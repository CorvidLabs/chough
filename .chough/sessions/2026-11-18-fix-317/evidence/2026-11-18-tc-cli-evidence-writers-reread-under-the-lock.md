---
kind: evidence
id: 2026-11-18-tc-cli-evidence-writers-reread-under-the-lock
aliases: []
claim: "Ran cargo test --locked --test evidence -- --exact evidence_add_rereads_the_session_under_the_writer_lock for TC-cli-evidence-writers-reread-under-the-lock: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-evidence-writers-reread-under-the-lock]
case_digest: [575910c8212b17c6b7bf602674e60cb4627a440a51b12a9905324dcd18485608]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - evidence
  - "--"
  - "--exact"
  - evidence_add_rereads_the_session_under_the_writer_lock
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: caa810e4ef02f64753c2d8ddb1bd0b61a9e2c25d
content: 7c45b7802733e796b7ad5489ded4950293700e9d6fc31ccf43a40e8fddc1e8a3
environment: local-macos-arm64
fingerprint: sha256:838d226739af2b0c6dcca7e907a712495b056ff54d77ecd11edcc3768c2cd447
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:26:57Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test evidence -- --exact evidence_add_rereads_the_session_under_the_writer_lock for TC-cli-evidence-writers-reread-under-the-lock: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision caa810e4ef02, observed 2026-11-18T16:26:57Z. The raw log stays in the machine-local store.
