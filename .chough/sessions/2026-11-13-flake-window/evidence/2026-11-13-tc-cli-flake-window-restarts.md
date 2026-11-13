---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-restarts
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-restarts]
case_digest: [2ba347ea0fd6652b41287956e98b3f8e4272cb790fd007afb15f7e160805027a]
command: [python3, .github/scripts/test-flake-window.py, RestartTests]
exit_code: 1
duration_seconds: 0
revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
environment: local-macos-arm64
fingerprint: sha256:7f93aa456d0d55a79b30d7b8bf790578df85ab53f689dabce906b85712fc82c0
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"OK\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:41:16Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision 2f6a807d912c, observed 2026-11-13T03:41:16Z. 21 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "OK".

## Key failing lines

-   File "<frozen importlib._bootstrap_external>", line 896, in get_code
-   File "<frozen importlib._bootstrap_external>", line 954, in get_data
- FileNotFoundError: [Errno 2] No such file or directory: '<redacted:home-path>'
- ======================================================================
- ERROR: test_a_rerun_on_a_pull_request_branch_restarts_the_window (__main__.RestartTests.test_a_rerun_on_a_pull_request_branch_restarts_the_window)
- ----------------------------------------------------------------------
- Traceback (most recent call last):
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
- Ran 7 tests in 0.003s
- FAILED (errors=7)
