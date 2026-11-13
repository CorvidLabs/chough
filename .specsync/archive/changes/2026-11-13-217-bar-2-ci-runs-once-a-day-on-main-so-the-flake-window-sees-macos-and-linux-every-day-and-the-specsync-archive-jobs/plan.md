---
change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
artifact: plan
---

# Plan

## Approach

Add one `schedule` trigger to ci.yml and a `|| github.sha` fallback to the
SpecSync Archive base. A test in tests/package.rs pins both. docs/ci.md and the
CHANGELOG say so.

## Out of scope

The flake-window check itself (#252) and the seed PR.

## Steps

1. A failing test in tests/package.rs, with its red run recorded.
2. The trigger and the fallback in ci.yml.
3. docs/ci.md and the CHANGELOG.
4. Green, the full suite and the audit. After merge, the first scheduled run is
   observed on GitHub.

## Risks

GitHub can delay a scheduled run or skip it under load. A skipped day restarts the
window, which the receipt shows as `no-green-run`.

## Constraints consulted

check-specsync-archive.py's base rule, and specs/cli/cli.3md's CI lessons.
