---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-clean-5effb3
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-clean]
case_digest: [5fb48fb0e12045738962b757c647c98d98cc3e5853ffaefa7966dce77e4024b9]
command: [python3, .github/scripts/test-flake-window.py, CleanWindowTests]
exit_code: 0
duration_seconds: 0
revision: 6d69df93c2d29ee38648271b4a18cbf094859e40
content: 9ed9840c2fc80f35edc628a943b212bbbf09d2045b865361738bcd65bf5bed15
environment: local-macos-arm64
fingerprint: sha256:f920ee0e8b42b181363bd792271236b9351c43fda3f29cd63ebd200d677c8eaf
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T05:02:21Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 6d69df93c2d2, observed 2026-11-13T05:02:21Z. The raw log stays in the machine-local store.
