---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-fails-closed-cbf9e4
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-fails-closed]
case_digest: [0e0806d15d6b566c3f6d2c5bfe6480ed7feb0f861f604189adc7b8c0b473a2d4]
command: [python3, .github/scripts/test-flake-window.py, FailClosedTests]
exit_code: 1
duration_seconds: 1
revision: decb87e8551e30566ff07465e2c1cfa86287a151
content: 7a0326c434e6f3578d5feabd4edfac273f1a1637286afbba5750f9d3f5f5881d
environment: local-macos-arm64
fingerprint: sha256:b3739b6d72bafbab8af578aeaff262baa42c3e91edb54794b6e8f89078a6d22b
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"OK\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T05:01:18Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py FailClosedTests for TC-cli-flake-window-fails-closed: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision decb87e8551e, observed 2026-11-13T05:01:18Z. 5 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "OK".

## Key failing lines

-                               ~~~~~~~~~^^^^^^^^^^^
-   File "<redacted:home-path>", line 229, in read_bugs
-     text = path.read_text(encoding="utf-8").replace("\r\n", "\n")
-            ~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^
-   File "/opt/homebrew/Cellar/python@3.14/3.14.8/Frameworks/Python.framework/Versions/3.14/lib/python3.14/pathlib/__init__.py", line 788, in read_text
-     return f.read()
-            ~~~~~~^^
-   File "<frozen codecs>", line 325, in decode
- UnicodeDecodeError: 'utf-8' codec can't decode byte 0xff in position 37: invalid start byte
- ======================================================================
- ERROR: test_the_receipt_names_a_digest_of_the_bug_records_it_read (__main__.FailClosedTests.test_the_receipt_names_a_digest_of_the_bug_records_it_read)
- ----------------------------------------------------------------------
- Traceback (most recent call last):
-   File "<redacted:home-path>", line 420, in test_the_receipt_names_a_digest_of_the_bug_records_it_read
-     first = json.loads(stdout)["inputs"]["bug_records_sha256"]
-             ~~~~~~~~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^
- KeyError: 'bug_records_sha256'
- ----------------------------------------------------------------------
- Ran 8 tests in 1.400s
- FAILED (errors=2)
