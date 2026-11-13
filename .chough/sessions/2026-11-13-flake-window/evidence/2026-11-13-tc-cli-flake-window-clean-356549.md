---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-clean-356549
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
revision: 1d711eddf2ac335963f51cd802604e839b212a7e
content: 552e0adb8469b6e05a2648cdca2cc07d6e9cd0eed92c842f1170d1e653297e27
environment: local-macos-arm64
fingerprint: sha256:1585fd3811c11e4cba8b310791c888030101c7170b1980e31983330a39dc9e52
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:50:16Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 1d711eddf2ac, observed 2026-11-13T03:50:16Z. The raw log stays in the machine-local store.
