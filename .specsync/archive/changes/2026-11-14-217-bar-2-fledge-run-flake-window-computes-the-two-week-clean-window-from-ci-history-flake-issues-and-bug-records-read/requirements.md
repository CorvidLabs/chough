---
change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
artifact: requirements
---

# Requirements

REQ-cli-062 (added): `fledge run flake-window` computes the 1.0 flake window (LIFE-21
bar 2) from the CI workflow's run history, the `flake` issues and the bug records,
writes nothing, and prints one JSON receipt with a verdict of `clean`, `running` or
`blocked`. A red or partly skipped main run, any rerun, a filed flake and a day
without a green main run on both OSes restart it. Open flakes and an issue without its
bug record block it. Unreadable GitHub data exits 2. The full text is in
`deltas/cli.md`.
