---
change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
artifact: testing
---

# Testing

`tests/package.rs` reads `.github/workflows/ci.yml`. It checks for one daily
`cron` under `schedule:` beside the `push` and `pull_request` triggers, and for
`|| github.sha` ending the SpecSync Archive base. It is red at the test commit and
green at the fix. The first scheduled run after the merge is observed on GitHub and
recorded.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-064 | `tests/package.rs`: ci_runs_once_a_day_on_main_with_a_real_archive_base |
