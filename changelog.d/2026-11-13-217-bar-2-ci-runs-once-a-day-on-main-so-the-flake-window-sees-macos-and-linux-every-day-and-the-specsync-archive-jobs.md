- CI also runs once a day on main at 05:23 UTC, so the flake window sees macOS and
  Linux every day even when nothing merges. On a scheduled run the `SpecSync Archive`
  jobs take the run's own commit as their trusted base, since the event has neither a
  pull request base nor a previous commit (REQ-cli-064, #217).
