---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-clean
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-clean]
case_digest: [5fb48fb0e12045738962b757c647c98d98cc3e5853ffaefa7966dce77e4024b9]
command: [python3, .github/scripts/test-flake-window.py, CleanWindowTests]
exit_code: 1
duration_seconds: 0
revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
environment: local-macos-arm64
fingerprint: sha256:c49badcc265d8c506f95a7fdc3fc68ad3e3345ddf55e250ee56df322af6d6f05
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"OK\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:41:13Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py CleanWindowTests for TC-cli-flake-window-clean: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision 2f6a807d912c, observed 2026-11-13T03:41:13Z. 9 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "OK".

## Key failing lines

-   File "<frozen importlib._bootstrap_external>", line 896, in get_code
-   File "<frozen importlib._bootstrap_external>", line 954, in get_data
- FileNotFoundError: [Errno 2] No such file or directory: '<redacted:home-path>'
- ======================================================================
- ERROR: test_fourteen_green_days_without_reruns_or_flakes_are_clean (__main__.CleanWindowTests.test_fourteen_green_days_without_reruns_or_flakes_are_clean)
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
- Ran 3 tests in 0.001s
- FAILED (errors=3)
