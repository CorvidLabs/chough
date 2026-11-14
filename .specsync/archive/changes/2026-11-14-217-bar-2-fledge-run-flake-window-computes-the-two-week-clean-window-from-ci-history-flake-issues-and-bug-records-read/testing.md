---
change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
artifact: testing
---

# Testing

`.github/scripts/test-flake-window.py` (unittest, standard library only) builds run,
job, issue and bug fixtures in code and checks the receipt. It also runs the script
with a fake `gh` on PATH, so the fetch, pagination and fail-closed exits are
exercised without the network. Red at the test commit, green at the fix. Chough cases
bind each test class through the `command` adapter.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-062 | `.github/scripts/test-flake-window.py`: CleanWindowTests, RestartTests, BlockedTests, FailClosedTests and WiringTests |
