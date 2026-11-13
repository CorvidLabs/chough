---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-blocked
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-blocked]
case_digest: [a480bb1749670a6d79b43f09a94931f954195cc93c5a4928953b9518accdb81b]
command: [python3, .github/scripts/test-flake-window.py, BlockedTests]
exit_code: 1
duration_seconds: 0
revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
environment: local-macos-arm64
fingerprint: sha256:9627d49e16e01bc200ff1fa0fd4e4e71311b4d374b2870e11af08c44d30bd0b2
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"OK\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:41:18Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py BlockedTests for TC-cli-flake-window-blocked: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision 2f6a807d912c, observed 2026-11-13T03:41:18Z. 20 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "OK".

## Key failing lines

- ======================================================================
- ERROR: test_an_open_flake_issue_blocks_the_window (__main__.BlockedTests.test_an_open_flake_issue_blocks_the_window)
- ----------------------------------------------------------------------
- Traceback (most recent call last):
-   File "<redacted:home-path>", line 211, in setUp
-     super().setUp()
-     ~~~~~~~~~~~~~^^
-   File "<redacted:home-path>", line 115, in setUp
-     self.script = load_script()
-                   ~~~~~~~~~~~^^
-   File "<redacted:home-path>", line 38, in load_script
-     spec.loader.exec_module(module)
-     ~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^
-   File "<frozen importlib._bootstrap_external>", line 759, in exec_module
-   File "<frozen importlib._bootstrap_external>", line 896, in get_code
-   File "<frozen importlib._bootstrap_external>", line 954, in get_data
- FileNotFoundError: [Errno 2] No such file or directory: '<redacted:home-path>'
- ----------------------------------------------------------------------
- Ran 5 tests in 0.002s
- FAILED (errors=5)
