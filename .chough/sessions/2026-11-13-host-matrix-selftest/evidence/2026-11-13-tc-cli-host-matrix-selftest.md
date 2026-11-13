---
kind: evidence
id: 2026-11-13-tc-cli-host-matrix-selftest
aliases: []
claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-host-matrix-selftest]
case_digest: [c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2]
command: [sh, tools/host-matrix/selftest.sh]
exit_code: 1
duration_seconds: 0
revision: adf4bfe481f22dad28c8e31809c09a5bc072fabc
content: e5ab5fab705b055ae79c26426c8d5a5dce6b0d9089fc434b0d090fcb78b3a78d
environment: local-macos-arm64
fingerprint: sha256:77c89ae68051974383f1ee0ec830421de41740f0ba56027cdd7caffcc3abaa9b
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"HOST MATRIX SELFTEST PASSED\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:55:24Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision adf4bfe481f2, observed 2026-11-13T03:55:24Z. 2 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "HOST MATRIX SELFTEST PASSED".

## Key failing lines

- sh: <redacted:home-path>: No such file or directory
- HOST MATRIX SELFTEST FAILED: good: expected PASS, got no result: 
- Kept <redacted:system-path>
