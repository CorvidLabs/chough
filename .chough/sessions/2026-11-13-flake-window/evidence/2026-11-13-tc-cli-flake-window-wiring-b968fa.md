---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-wiring-b968fa
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-wiring]
case_digest: [03d0f18b6f7c3f3416763c407dffaf5a954527710c901efc8c5952455012c242]
command: [python3, .github/scripts/test-flake-window.py, WiringTests]
exit_code: 0
duration_seconds: 0
revision: 1d711eddf2ac335963f51cd802604e839b212a7e
content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
environment: local-macos-arm64
fingerprint: sha256:fe8ad70cd7d04840fb847fb044ab3213794a24cb07a69ef8461ca6a8f75a7322
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:50:34Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 1d711eddf2ac, observed 2026-11-13T03:50:34Z. The raw log stays in the machine-local store.
