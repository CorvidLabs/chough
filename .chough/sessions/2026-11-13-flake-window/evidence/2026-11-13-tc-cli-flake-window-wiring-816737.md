---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-wiring-816737
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
revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
environment: local-macos-arm64
fingerprint: sha256:2651f01cb66585ce829185f1e4cb01d8394ab91d01098d9fa5ae8b294d12b046
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T05:02:32Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 6d69df93c2d2, observed 2026-11-13T05:02:32Z. The raw log stays in the machine-local store.
