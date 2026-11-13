## ADDED

### REQUIREMENT REQ-cli-064

Run CI once a day on main. The CI workflow SHALL run on a daily schedule as well as on every push to main and every pull request. GitHub runs a schedule on the default branch's latest commit, so the flake window (REQ-cli-062) sees a run of `Product / ubuntu-latest` and `Product / macos-latest` every day even when nothing merges. A scheduled run SHALL run the same jobs as a push to main. When an event has neither a pull request base nor a previous commit, the SpecSync Archive jobs SHALL take the run's own commit as their trusted base, so a scheduled run never hands the archive inspector an empty base. Trace: LIFE-21, DRIVE-19.

Acceptance Criteria
- `.github/workflows/ci.yml` has a `schedule` trigger with one daily `cron` entry, beside its `push` to main and `pull_request` triggers.
- The SpecSync Archive jobs' base falls back to `github.sha` after the pull request base and the previous commit.
- `tests/package.rs` fails, naming what is missing, when the schedule or the fallback is removed.
