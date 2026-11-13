---
kind: evidence
id: 2026-11-13-tc-cli-host-matrix-selftest-349558
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
duration_seconds: 92
revision: 91b227be06a40a5fea5950bcf92c8a812820491b
content: 37601ca808fa35558fbf1b9a6c5612b97deaf5d05c594db7a389cd36732b8592
environment: local-macos-arm64
fingerprint: sha256:635ea36defd727c787e21fc1d9673fd714c36bea1576cf4127d44e56c3d45b27
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T07:31:25Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 91b227be06a4, observed 2026-11-13T07:31:25Z. The raw log stays in the machine-local store.
