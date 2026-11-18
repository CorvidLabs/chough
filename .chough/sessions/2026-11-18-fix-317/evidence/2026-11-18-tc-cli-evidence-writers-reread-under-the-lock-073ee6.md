---
kind: evidence
id: 2026-11-18-tc-cli-evidence-writers-reread-under-the-lock-073ee6
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
duration_seconds: 1
revision: 9c83ec0ac3c1bba8cc7b07fc6f7087dd103a7d62
content: bcc171b5309451c52cda846ab004933cbb31d5ee8a28e1274703dd2c48012946
environment: local-macos-arm64
fingerprint: sha256:cbbc04a9f19a10947988b568dcb838f9d7936d4c5e0c18b288df89a207d2b2ad
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T17:02:40Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test evidence -- --exact evidence_add_rereads_the_session_under_the_writer_lock for TC-cli-evidence-writers-reread-under-the-lock: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9c83ec0ac3c1, observed 2026-11-18T17:02:40Z. The raw log stays in the machine-local store.
