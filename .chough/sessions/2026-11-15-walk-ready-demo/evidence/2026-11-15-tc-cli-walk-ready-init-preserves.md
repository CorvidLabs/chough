---
kind: evidence
id: 2026-11-15-tc-cli-walk-ready-init-preserves
aliases: []
claim: "Ran echo preserves for TC-cli-walk-ready-init-preserves: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-walk-ready-init-preserves]
case_digest: [9d9baac3967545ef8b9eb99534b8727354ef0e02a3d509284c9f152d31ba9ced]
command: [echo, preserves]
exit_code: 0
duration_seconds: 0
revision: 913c0068ee0894aaa6256eb2153419612f8e5172
content: 033dd4cc3a2cf422444d1370ddbec06e30c4f3244018dc58e9d06a7615d0af1c
environment: local-macos-arm64
fingerprint: sha256:3e617855a2ff137331bb011761ee0e379fdaa38819a0d598a6547707d9e53864
criteria: [LIFE-4, LIFE-6]
requirements: [REQ-cli-001]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-15T03:49:07Z
actor:
  - terminal: t15
---
**Ran echo preserves for TC-cli-walk-ready-init-preserves: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 913c0068ee08, observed 2026-11-15T03:49:07Z. The raw log stays in the machine-local store.
