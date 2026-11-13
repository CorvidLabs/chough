---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-wiring
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-wiring]
case_digest: [03d0f18b6f7c3f3416763c407dffaf5a954527710c901efc8c5952455012c242]
command: [python3, .github/scripts/test-flake-window.py, WiringTests]
exit_code: 1
duration_seconds: 0
revision: 2f6a807d912cc30cf740c99b1b92cfc5896f5756
content: 65c737cfb72492c9aff2cfe5ae2aff47f17987e8b55e4eb7297da321befdb1e6
environment: local-macos-arm64
fingerprint: sha256:76e8dce46c198774485ceee13712e159630771496e7c235045662c7abdd8d6fb
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"OK\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T03:41:23Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py WiringTests for TC-cli-flake-window-wiring: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision 2f6a807d912c, observed 2026-11-13T03:41:23Z. 5 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "OK".

## Key failing lines

- FileNotFoundError: [Errno 2] No such file or directory: '<redacted:home-path>'
- ======================================================================
- FAIL: test_fledge_runs_the_check_and_its_fixtures (__main__.WiringTests.test_fledge_runs_the_check_and_its_fixtures)
- ----------------------------------------------------------------------
- Traceback (most recent call last):
-   File "<redacted:home-path>", line 402, in test_fledge_runs_the_check_and_its_fixtures
-     self.assertEqual(tasks.get("flake-window"), "python3 .github/scripts/flake-window.py")
-     ~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- AssertionError: None != 'python3 .github/scripts/flake-window.py'
- ======================================================================
- FAIL: test_the_ci_gate_fixtures_job_runs_the_fixtures (__main__.WiringTests.test_the_ci_gate_fixtures_job_runs_the_fixtures)
- ----------------------------------------------------------------------
- Traceback (most recent call last):
-   File "<redacted:home-path>", line 413, in test_the_ci_gate_fixtures_job_runs_the_fixtures
-     self.assertIn("run: python3 .github/scripts/test-flake-window.py", job)
-     ~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- AssertionError: 'run: python3 .github/scripts/test-flake-window.py' not found in "  ci-fixtures:\n    name: CI gate fixtures\n    needs: archive-gate\n    runs-on: ubuntu-latest\n    timeout-minutes: 10\n    steps:\n      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2\n        with:\n          fetch-depth: 0\n          persist-credentials: false\n      - uses: CorvidLabs/spec-sync@b87bbd88efea06ffd9474af02018240f24300419 # v6.3.0\n        with:\n          version: '6.3.0'\n          strict: 'true'\n          require-coverage: '100'\n      - run: python3 .github/scripts/test-specsync-archive.py"
- ----------------------------------------------------------------------
- Ran 3 tests in 0.001s
- FAILED (failures=2, errors=1)
