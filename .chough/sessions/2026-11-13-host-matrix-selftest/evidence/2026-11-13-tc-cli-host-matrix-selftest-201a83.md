---
kind: evidence
id: 2026-11-13-tc-cli-host-matrix-selftest-201a83
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
duration_seconds: 149
revision: 6b7376cf61b3b7431180580c989661393f26ff44
content: 8327d299d9048bc9377db6a93dc4cfccc37b3fd42e35d3e1f690ea5fcfaa4a79
environment: local-macos-arm64
fingerprint: sha256:701726f03a054818714773b57dee3b9e73f79a6fdb0f8436aa1f42e803a3ec72
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
  - "recorded on a toolchain that differs from CI: differs: Fledge 1.8.1 (CI 1.8.0)\n2026-11-13T04:28:54Z\nHI: installed 0.8.0, CI pins 0.8.0\nSpecSync: installed 6.3.0, CI pins 6.3.0\nFledge: installed 1.8.1, CI pins 1.8.0\nRust (rustc): installed 1.99.0,"
observed: 2026-11-13T04:31:27Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 6b7376cf61b3, observed 2026-11-13T04:31:27Z. The raw log stays in the machine-local store.
