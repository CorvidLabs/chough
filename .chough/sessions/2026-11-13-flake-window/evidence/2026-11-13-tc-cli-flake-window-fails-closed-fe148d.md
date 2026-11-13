---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-fails-closed-fe148d
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-fails-closed]
case_digest: [0e0806d15d6b566c3f6d2c5bfe6480ed7feb0f861f604189adc7b8c0b473a2d4]
command: [python3, .github/scripts/test-flake-window.py, FailClosedTests]
exit_code: 0
duration_seconds: 2
revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
environment: local-macos-arm64
fingerprint: sha256:0d47e6f94db721901d853ebe7ec8f254757fb68b78bb5b92fff3fb28e1738819
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T05:02:30Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 6d69df93c2d2, observed 2026-11-13T05:02:30Z. The raw log stays in the machine-local store.
