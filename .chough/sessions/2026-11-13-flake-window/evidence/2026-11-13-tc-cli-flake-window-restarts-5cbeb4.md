---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-restarts-5cbeb4
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-restarts]
case_digest: [2ba347ea0fd6652b41287956e98b3f8e4272cb790fd007afb15f7e160805027a]
command: [python3, .github/scripts/test-flake-window.py, RestartTests]
exit_code: 0
duration_seconds: 0
revision: 1d711eddf2ac335963f51cd802604e839b212a7e
content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
environment: local-macos-arm64
fingerprint: sha256:afbd3a6b8c662d0b567f788d3ac47decb966b63939d50ee6b3f5c8e3a37f5288
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:50:20Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 1d711eddf2ac, observed 2026-11-13T03:50:20Z. The raw log stays in the machine-local store.
