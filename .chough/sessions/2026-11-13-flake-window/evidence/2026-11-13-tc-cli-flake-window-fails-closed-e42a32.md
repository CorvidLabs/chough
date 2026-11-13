---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-fails-closed-e42a32
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-fails-closed]
case_digest: [b41c1406363789e9833fe3673f7285e4aaca6bc9cad9653be174651d49deea4d]
command: [python3, .github/scripts/test-flake-window.py, FailClosedTests]
exit_code: 0
duration_seconds: 1
revision: 1d711eddf2ac335963f51cd802604e839b212a7e
content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
environment: local-macos-arm64
fingerprint: sha256:3204661e793c7d18300d40c2d56990aa38e1b5fc8faf05e650ddf9c484c05f8b
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:50:30Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 1d711eddf2ac, observed 2026-11-13T03:50:30Z. The raw log stays in the machine-local store.
