---
kind: evidence
id: 2026-11-13-tc-cli-flake-window-restarts-ec7983
aliases: []
claim: "Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: failed (exit 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-flake-window-restarts]
case_digest: [666a0615f7ef82a6be52c3d549f5243735eff1462328f4111a0a6f9d29402ecc]
command: [python3, .github/scripts/test-flake-window.py, RestartTests]
exit_code: 1
duration_seconds: 0
revision: decb87e8551e30566ff07465e2c1cfa86287a151
content: 7a0326c434e6f3578d5feabd4edfac273f1a1637286afbba5750f9d3f5f5881d
environment: local-macos-arm64
fingerprint: sha256:7817208cd92f26aba621ff47e37ec640875466d4ba23f4f48a632d6579851375
criteria: [LIFE-21, DRIVE-19]
requirements: [REQ-cli-062]
contracts: [specs/cli/cli.spec.md]
limitations:
  - "expected output missing: \"OK\""
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-13T05:01:14Z
actor:
  - agent: claude-code
    terminal: t38r
---
**Ran python3 .github/scripts/test-flake-window.py RestartTests for TC-cli-flake-window-restarts: failed (exit 1).**

Recorded as failed in local-macos-arm64 at revision decb87e8551e, observed 2026-11-13T05:01:14Z. 2 recorded lines were redacted. The raw log stays in the machine-local store.

## Why it failed

- The expected output was missing: "OK".

## Key failing lines

- .......F
- ======================================================================
- FAIL: test_two_runs_that_each_pass_only_one_product_job_do_not_cover_their_day (__main__.RestartTests.test_two_runs_that_each_pass_only_one_product_job_do_not_cover_their_day)
- ----------------------------------------------------------------------
- Traceback (most recent call last):
-   File "<redacted:home-path>", line 215, in test_two_runs_that_each_pass_only_one_product_job_do_not_cover_their_day
-     self.assert_running(receipt, "no-green-run", end_of_gap)
-     ~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   File "<redacted:home-path>", line 123, in assert_running
-     self.assertEqual(receipt["latest_restart"]["kind"], kind, receipt)
-     ~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- AssertionError: 'red-main-run' != 'no-green-run'
- - red-main-run
- + no-green-run
-  : {'schema': 'chough-flake-window/1', 'repository': None, 'computed_at': '2026-11-20T12:00:00Z', 'required_days': 14, 'verdict': 'running', 'latest_restart': {'kind': 'red-main-run', 'at': '2026-11-17T08:00:00Z', 'detail': 'main run 1016 did not succeed: Product / ubuntu-latest'}, 'clean_from': '2026-12-01T08:00:00Z', 'days_since_latest_restart': 3.17, 'blocked_by': [], 'restarts_in_window': 2, 'recent_restarts': [{'kind': 'red-main-run', 'at': '2026-11-17T06:00:00Z', 'detail': 'main run 1015 did not succeed: Product / macos-latest'}, {'kind': 'red-main-run', 'at': '2026-11-17T08:00:00Z', 'detail': 'main run 1016 did not succeed: Product / ubuntu-latest'}], 'green_main_runs': {'linux': 15, 'macos': 15}, 'flakes': {'open': [], 'closed': []}, 'inputs': {'runs': 16, 'main_runs': 16, 'jobs_read': 16, 'flake_issues': 0, 'bug_records': 0, 'flake_bug_records': 0, 'lookback_days': None, 'sha256': '35d3446814bd418fa3c679a87105368ee3f531db849b5902dd745cb7bfdb9322'}}
- ----------------------------------------------------------------------
- Ran 8 tests in 0.033s
- FAILED (failures=1)
