---
kind: evidence
id: 2026-11-18-tc-cli-bug-add-holds-the-writer-lock-5d0dbd
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
duration_seconds: 2
revision: 9c83ec0ac3c1bba8cc7b07fc6f7087dd103a7d62
content: bcc171b5309451c52cda846ab004933cbb31d5ee8a28e1274703dd2c48012946
environment: local-macos-arm64
fingerprint: sha256:4bbc55fcd8050c8d4571597c3f1607592317d9d507b0cec31199614ffbe437ce
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T17:02:43Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test evidence -- --exact bug_add_holds_the_writer_lock_and_keeps_another_writers_update for TC-cli-bug-add-holds-the-writer-lock: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9c83ec0ac3c1, observed 2026-11-18T17:02:43Z. The raw log stays in the machine-local store.
