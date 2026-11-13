---
change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
artifact: requirements
---

# Requirements

REQ-cli-064 (added): CI also runs once a day on main, so the flake window sees macOS
and Linux every day. On an event with no base, the SpecSync Archive jobs take the run's
own commit as their base. The full text is in `deltas/cli.md`.
