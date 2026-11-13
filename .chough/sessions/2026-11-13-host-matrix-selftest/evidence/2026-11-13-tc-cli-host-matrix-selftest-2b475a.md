---
kind: evidence
id: 2026-11-13-tc-cli-host-matrix-selftest-2b475a
aliases: []
claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-host-matrix-selftest]
case_digest: [c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2]
command: [sh, tools/host-matrix/selftest.sh]
exit_code: 0
duration_seconds: 86
revision: 30d6405e58c204b7643531412d31856036dd1895
content: 2fa1b777102b16a83cd3d8c9e54b14a723c990f4d6e6d5c44077dce37c2dc37f
environment: local-macos-arm64
fingerprint: sha256:635ea36defd727c787e21fc1d9673fd714c36bea1576cf4127d44e56c3d45b27
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T14:10:03Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 30d6405e58c2, observed 2026-11-13T14:10:03Z. The raw log stays in the machine-local store.
