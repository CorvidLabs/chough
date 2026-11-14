---
change: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
artifact: context
---

# Context

On 2026-11-13 at 03:30Z Leif answered ch10-flake's (claude-code, hc t38r) design
question Q3, "Run CI once a day on main so a quiet week still counts?", with "Yes,
macOS + Linux (Recommended)". The answer was relayed by t370.

The flake window (REQ-cli-062, PR #252) restarts after any full UTC day with no green
main run on both platforms. Its first live run found two such days in the last 14,
2026-10-30 and 2026-11-09, when nothing merged.

A scheduled event has no pull request base and no `before` commit. Today the SpecSync
Archive step passes `github.event.pull_request.base.sha || github.event.before`, and
check-specsync-archive.py refuses an empty base ("A real trusted base revision is
required"). Every scheduled run would therefore be red and restart the window. The
run's own commit is a real base: the inspector then walks that one revision and still
authenticates every archive.
