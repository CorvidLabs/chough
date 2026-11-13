---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-blocked-7a226e
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-blocked]
case_digest: [a480bb1749670a6d79b43f09a94931f954195cc93c5a4928953b9518accdb81b]
command: [python3, .github/scripts/test-flake-window.py, BlockedTests]
exit_code: 0
duration_seconds: 0
revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
environment: local-macos-arm64
fingerprint: sha256:216becf55cbda547302a144f477e2a6af4bc257164d7c3d1b84def3b52118a65
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T05:02:25Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 6d69df93c2d2, observed 2026-11-13T05:02:25Z. The raw log stays in the machine-local store.
