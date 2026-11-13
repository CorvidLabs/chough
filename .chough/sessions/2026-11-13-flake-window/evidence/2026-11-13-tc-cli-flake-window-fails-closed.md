---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-fails-closed
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-fails-closed]
case_digest: [b41c1406363789e9833fe3673f7285e4aaca6bc9cad9653be174651d49deea4d]
command: [python3, .github/scripts/test-flake-window.py, FailClosedTests]
exit_code: 1
duration_seconds: 0
revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
environment: local-macos-arm64
fingerprint: sha256:65e35ab3aaa8f293600301e0d87e12c2bb00748b2425863950b43194b03d9bed
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"OK\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:41:21Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision 2f6a807d912c, observed 2026-11-13T03:41:21Z. 24 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "OK".

## Key failing lines

- ======================================================================
- ERROR: test_the_fetch_reads_every_page_of_every_day_and_the_flake_issues (__main__.FailClosedTests.test_the_fetch_reads_every_page_of_every_day_and_the_flake_issues)
- ----------------------------------------------------------------------
- Traceback (most recent call last):
-   File "<redacted:home-path>", line 292, in setUp
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
- Ran 6 tests in 0.002s
- FAILED (errors=6)
