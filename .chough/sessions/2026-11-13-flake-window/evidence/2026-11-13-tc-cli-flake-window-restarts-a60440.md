---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-restarts-a60440
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-restarts]
case_digest: [666a0615f7ef82a6be52c3d549f5243735eff1462328f4111a0a6f9d29402ecc]
command: [python3, .github/scripts/test-flake-window.py, RestartTests]
exit_code: 0
duration_seconds: 0
revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
environment: local-macos-arm64
fingerprint: sha256:86337cfd8ed176d8d90be3f7c4587b63318e82de083d026a6942ae2da96bba02
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T05:02:23Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 6d69df93c2d2, observed 2026-11-13T05:02:23Z. The raw log stays in the machine-local store.
