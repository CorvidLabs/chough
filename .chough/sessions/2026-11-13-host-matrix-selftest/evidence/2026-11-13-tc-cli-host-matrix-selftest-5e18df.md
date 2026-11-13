---
kind: evidence
id: 2026-11-13-tc-cli-host-matrix-selftest-5e18df
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
duration_seconds: 118
revision: f2184c94d1988ff3ebe286cf586253250ea05362
content: 45f54f7cc5e75b9c9fefff749e94de7eb28e786471d42f738487b79935dc472b
environment: local-macos-arm64
fingerprint: sha256:a2afb191c3d6660834faac9d5436eac8b86890382bb6436940a8bbe46aaa91c8
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T04:14:42Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision f2184c94d198, observed 2026-11-13T04:14:42Z. The raw log stays in the machine-local store.
